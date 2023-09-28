function spacesRegex() { return /[^a-z0-9]+/gi; }

function isPrime(x) {
  let lut = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false];
  if(x <= 20) return lut[x];
  for(let i = 2; i*i <= x; i++) {
    if(x % i === 0) return false;
  }
  return true;
}

export function primeOfPrimeFilter() {
  return {
    filter: s => {
      let words = s.replace(/\$/g, "").split(spacesRegex());
      let nPrimeLengthWords = words.filter(s => isPrime(s.length)).length;
      return isPrime(nPrimeLengthWords);
    },
    errorMsg: "must contain prime number of prime-length words",
    errorMsgInv: "must not contain prime number of prime-length words"
  }
}