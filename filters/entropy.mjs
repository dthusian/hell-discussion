export function entropyFilter() {
  return {
    filter: s => /^there['"\u2019]s a horse in aisle five$/.test(s) || /^my house is full of traps$/.test(s),
    errorMsg: "must be either 'my house is full of traps' or 'there's a horse in aisle five'"
  }
}