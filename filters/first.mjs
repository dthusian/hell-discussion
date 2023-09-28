export function noStupidFirstMessage() {
  return {
    filter: s => s.toLowerCase() === "first",
    errorMsg: "must be \"first\"",
    errorMsgInv: "must not be \"first\""
  }
}