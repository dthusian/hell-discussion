use std::env::args;
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::Path;
use image::io::Reader;
use anyhow::Result;
use image::RgbaImage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Weights {
  sat: Vec<f64>,
  vis: Vec<f64>
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

fn test_histogram(hist: (&Vec<f64>, &Vec<f64>), weight: (&Vec<f64>, &Vec<f64>)) -> f64 {
  hist.0.iter().zip(weight.0.iter()).map(|(a, b)| *a * *b).sum::<f64>() +
    hist.1.iter().zip(weight.1.iter()).map(|(a, b)| *a * *b).sum::<f64>()
}

fn read_image<P: AsRef<Path>>(path: P) -> Result<RgbaImage> {
  Ok(Reader::open(path)?.with_guessed_format()?.decode()?.to_rgba8())
}

fn hist_to_weights(mut v: f64, count: i32) -> f64 {
  v /= count as f64;
  v = if v != 0.0 { v.log2() } else { 0.0 };
  v
}

fn generate_weights() {
  let mut cs = vec![0f64; 256];
  let mut cv = vec![0f64; 256];
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
    cs.iter_mut().zip(s).for_each(|(a, b)| {
      *a += b;
    });
    cv.iter_mut().zip(v).for_each(|(a, b)| {
      *a += b;
    });
  });
  cs.iter_mut().for_each(|v| *v = hist_to_weights(*v, count));
  cv.iter_mut().for_each(|v| *v = hist_to_weights(*v, count));
  let weights = Weights {
    sat: cs,
    vis: cv
  };
  let mut f = File::create("assets/sekaiweights.json")
    .unwrap();
  f.write(&serde_json::to_vec(&weights).unwrap())
    .unwrap();
}

fn main() -> Result<()> {
  if args().len() < 2 {
    generate_weights();
    return Ok(());
  }
  let test_image = read_image(args().nth(1).unwrap())?;
  let (sv, vv) = gen_histogram(&test_image);
  let weight: Weights = serde_json::from_reader(File::open("assets/sekaiweights.json").unwrap()).unwrap();
  let score = test_histogram((&sv, &vv), (&weight.sat, &weight.vis));
  println!("{}", score);
  Ok(())
}
