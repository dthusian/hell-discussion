export function requireImageFilter() {
  return {
    filter: (s, msg) => !!msg.attachments.size,
    errorMsg: "must contain file",
    errorMsgInv: "must not contain file"
  }
}