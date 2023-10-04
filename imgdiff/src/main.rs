use std::env::args;
use std::fs::{File, read_dir};
use std::io::Write;
use std::ptr::swap;
use std::path::Path;
use image::io::Reader;
use anyhow::Result;
use image::RgbaImage;
use rustfft::FftPlanner;
use rustfft::num_complex::{Complex, ComplexFloat};
use serde::{Deserialize, Serialize};

const REF_FFT_SIZE: usize = 384;
const ENABLE_FFT: bool = false;

#[derive(Serialize, Deserialize)]
struct Weights {
  hist_sat_global: f64,
  hist_vis_global: f64,
  hist_sat: Vec<f64>,
  hist_vis: Vec<f64>,
  fft_vis_global: f64,
  fft_vis: Vec<Vec<f64>>
}

fn rgb_to_sv(r: u8, g: u8, b: u8) -> (u8, u8) {
  let cmax = r.max(g).max(b) as u32;
  let cmin = r.min(g).min(b) as u32;
  let delta = cmax - cmin;
  let sat = if cmax != 0 {
    255 * delta / cmax
  } else {
    0
  };
  let vis = cmax;
  return (sat as u8, vis as u8);
}

fn gen_histogram(test: &RgbaImage) -> (Vec<f64>, Vec<f64>) {
  let mut sv = vec![0f64; 256];
  let mut vv = vec![0f64; 256];
  let mut count = 0;
  test.pixels().for_each(|rgba| {
    let (s, v) = rgb_to_sv(rgba.0[0], rgba.0[1], rgba.0[2]);
    if rgba.0[3] > 10 && v < 254 && s > 1 {
      sv[s as usize] += 1.0;
      vv[v as usize] += 1.0;
      count += 1;
    }
  });
  sv.iter_mut().for_each(|v| {
    *v /= count as f64;
    *v *= 1000.0;
  });
  vv.iter_mut().for_each(|v| {
    *v /= count as f64;
    *v *= 1000.0;
  });
  (sv, vv)
}

fn test_histogram(hist: &Vec<f64>, weight: &Vec<f64>) -> f64 {
  hist.iter().zip(weight.iter()).map(|(a, b)| *a * *b).sum::<f64>()
}

fn transpose_mat<T>(sq_mat: &mut Vec<Vec<T>>) {
  for y in 0..sq_mat.len() {
    assert_eq!(sq_mat.len(), sq_mat[y].len());
    for x in (y + 1)..sq_mat[y].len() {
      unsafe { swap(&mut sq_mat[y][x], &mut sq_mat[x][y]) };
    }
  }
}

fn gen_fft(test: &RgbaImage, max_extent: usize) -> Vec<Vec<f64>> {
  // [y][x]
  let mut image_mat = vec![vec![Complex::<f64>::default(); max_extent]; max_extent];
  let fft = FftPlanner::new().plan_fft_forward(max_extent);
  // load image
  let pixels = test.as_flat_samples();
  for y in 0..test.height() as usize {
    for x in 0..test.width() as usize {
      let idx = (x * pixels.strides_cwh().0 + y * pixels.strides_cwh().1) as usize;
      image_mat[y][x] = Complex {
        re: rgb_to_sv(pixels.samples[idx], pixels.samples[idx + 1], pixels.samples[idx + 2]).0 as f64 / 255.0,
        im: 0.0
      };
    }
  }
  // fft horizontal
  for i in 0..max_extent {
    fft.process(&mut image_mat[i]);
  }
  transpose_mat(&mut image_mat);
  // fft vertical
  for i in 0..max_extent {
    fft.process(&mut image_mat[i]);
  }
  transpose_mat(&mut image_mat);
  image_mat.into_iter().map(|v| v.into_iter().map(|v| v.re()).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn test_fft(fft: &Vec<Vec<f64>>, reference: &Vec<Vec<f64>>) -> f64 {
  fn clamp<T: Ord>(x: T, min: T, max: T) -> T {
    if x < min { return min }
    if x > max { return max }
    return x
  }
  let fft_size = fft.len();
  let mut score = 0.0;
  for y in 0..REF_FFT_SIZE {
    for x in 0..REF_FFT_SIZE {
      let sample_x = clamp((fft_size * x) / REF_FFT_SIZE, 0, fft_size - 1);
      let sample_y = clamp((fft_size * y) / REF_FFT_SIZE, 0, fft_size - 1);
      let sample = fft[sample_y][sample_x];
      let fft = reference[y][x];
      score += (sample - fft).powi(2);
    }
  }
  (200.0*2000.0)/(score / ((REF_FFT_SIZE * REF_FFT_SIZE) as f64)).sqrt()
}

fn read_image<P: AsRef<Path>>(path: P) -> Result<RgbaImage> {
  Ok(Reader::open(path)?.with_guessed_format()?.decode()?.to_rgba8())
}

fn hist_to_weights(mut v: f64, count: i32) -> f64 {
  v /= count as f64;
  v = v.sqrt();
  v
}

fn fft_to_weights(mut v: f64, count: i32) -> f64 {
  v /= count as f64;
  v
}

fn main_generate_weights() {
  let mut hist_s = vec![0f64; 256];
  let mut hist_v = vec![0f64; 256];
  let mut fft_v = vec![vec![0f64; REF_FFT_SIZE]; REF_FFT_SIZE];
  let mut count = 0;
  read_dir("assets/sekai/").unwrap().for_each(|v| {
    let v = v.unwrap();
    println!("{}", v.path().to_str().unwrap());
    if !v.file_type().unwrap().is_file() {
      return;
    }
    count += 1;
    let img = read_image(v.path()).unwrap();
    let (s, v) = gen_histogram(&img);
    hist_s.iter_mut().zip(s).for_each(|(a, b)| {
      *a += b;
    });
    hist_v.iter_mut().zip(v).for_each(|(a, b)| {
      *a += b;
    });
    let v = gen_fft(&img, REF_FFT_SIZE);
    fft_v.iter_mut().flatten().zip(v.iter().flatten()).for_each(|(a, b)| {
      *a += *b;
    });
  });
  hist_s.iter_mut().for_each(|v| *v = hist_to_weights(*v, count));
  hist_v.iter_mut().for_each(|v| *v = hist_to_weights(*v, count));
  fft_v.iter_mut().flatten().for_each(|v| *v = fft_to_weights(*v, count));
  let weights = Weights {
    hist_sat_global: 0.4,
    hist_vis_global: 0.6,
    fft_vis_global: 0.5,
    hist_sat: hist_s,
    hist_vis: hist_v,
    fft_vis: if ENABLE_FFT { fft_v } else { vec![] },
  };
  let mut f = File::create("assets/sekaiweights.json")
    .unwrap();
  f.write(&serde_json::to_vec(&weights).unwrap())
    .unwrap();
}

fn main_test_image() {
  let test_image = read_image(args().nth(1).unwrap()).unwrap();
  let (hist_s, hist_v) = gen_histogram(&test_image);
  let weight: Weights = serde_json::from_reader(File::open("assets/sekaiweights.json").unwrap()).unwrap();
  let hist_s_score = test_histogram(&hist_s, &weight.hist_sat);
  let hist_v_score = test_histogram(&hist_v, &weight.hist_vis);
  let mut score = hist_s_score * weight.hist_sat_global + hist_v_score * weight.hist_vis_global;
  if ENABLE_FFT {
    let fft_v = gen_fft(&test_image, test_image.width().max(test_image.height()) as usize);
    if args().len() > 2 {
      let mut f = File::create("debug_fft_output.json").unwrap();
      f.write(&serde_json::to_vec(&fft_v).unwrap()).unwrap();
    }
    let fft_v_score = test_fft(&fft_v, &weight.fft_vis);
    score += fft_v_score * weight.fft_vis_global;
  }
  eprintln!("{} {}", hist_s_score, hist_v_score);
  println!("{}", score);
}

fn main() {
  if args().len() < 2 {
    main_generate_weights();
  } else {
    main_test_image();
  }
}
