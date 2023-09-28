function containsLink(s) {
  return /([^a-z0-9]|^)(https|http):\/\/[a-z0-9_-]+(\.[a-z0-9_-])+/i.test(s)
}

export function requireHyperlinkFilter() {
  return {
    filter: s => containsLink(s),
    errorMsg: "must contain link",
    errorMsgInv: "must not contain link"
  }
}