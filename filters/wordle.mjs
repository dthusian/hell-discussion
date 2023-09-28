import { readFileSync } from "fs";

const wordlist = new Set(readFileSync("assets/wordlist.txt").toString("utf8").split("\n"));

function spacesRegex() {
  return /[ \n\r\t_-]/g;
}

export function wordleFilter() {
  return {
    filter: s => s.split(spacesRegex()).filter(v => v).some(s2 => wordlist.has(s2.toUpperCase())),
    errorMsg: "must contain a wordle word",
    errorMsgInv: "must not contain a wordle word"
  }
}