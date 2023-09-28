const psunRegex = /^(?:(?!\\[A-Za-z]+\{[^{}]*\}|\$[^$]+\$|\\\[.*?\\\]|\\[A-Za-z]+).)*$/;

export function latexFilter() {
  return {
    filter: s => !psunRegex.test(s),
    errorMsg: "must contain latex",
    errorMsgInv: "must not contain latex"
  }
}