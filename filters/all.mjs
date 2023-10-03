import { rainWorldFilter } from "./rainworld.mjs";
import { requireHyperlinkFilter } from "./hyperlink.mjs";
import { requireImageFilter } from "./image.mjs"
import { noStupidFirstMessage } from "./first.mjs";
import { primeOfPrimeFilter } from "./prime.mjs"
import { latexFilter } from "./latex.mjs";
import { entropyFilter } from "./entropy.mjs";
import { stickerFilter } from "./sticker.mjs";
import { wordleFilter } from "./wordle.mjs";
import { sekaiFilter } from "./sekai.mjs";

export const filters = {
  "rainworld": rainWorldFilter(),
  "hyperlink": requireHyperlinkFilter(),
  "image": requireImageFilter(),
  "first": noStupidFirstMessage(),
  "prime": primeOfPrimeFilter(),
  "latex": latexFilter(),
  "entropy": entropyFilter(),
  "sticker": stickerFilter(),
  "wordle": wordleFilter(),
  "sekai": sekaiFilter()
};