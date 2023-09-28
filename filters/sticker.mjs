export function stickerFilter() {
  return {
    filter: (s, msg) => !!msg.stickers.size,
    errorMsg: "must contain sticker",
    errorMsgInv: "must not contain sticker"
  }
}