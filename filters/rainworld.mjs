function spacesRegex() {
  return /[ \n\r\t_/!*()=+-]/g;
}

export function rainWorldFilter() {
  function filter(msg) {
    msg = msg.toLowerCase().replace(/\$/g, "");

    let parts = msg.split(",");
    if(parts.length != 2) return false;

    function filterNItems(s) {
      const numberWords = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
        "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "ninety"
      ];
      let words = s.split(spacesRegex());
      if(words[0].match(/^[0-9]$/)) return true;
      if(numberWords.includes(words[0])) return true;
      return false;
    }

    function filterProbablySecond(s) {
      // basically all pronouns
      const definitelyDisallowed = [
        "i", "you", "he", "she", "him", "her", "his", "their", "your", "they", "am"
      ];
      const prepositionalWords = [
        "on", "at", "to", "in", "by", "across", "below", "past", "until", "after", "before", "into", "through",
        "over", "under", "inside", "above", "among", "amongst", "beside", "towards", "around", "with", "amid",
        "outside", "throughout", "upon", "within", "onto", "without"
      ];
      const adjectiveSuffixes = [
        "en", "ing"
      ];

      let words = s.split(spacesRegex());
      if(words.length > 5) return false;
      if(prepositionalWords.includes(words[0])) return true;
      if(adjectiveSuffixes.some(s => words[0].endsWith(s))) return true;
      if(definitelyDisallowed.includes(words[0])) return false;
      return true;
    }

    if(filterNItems(msg[0])) return true;
    if(filterNItems(msg[0]) || filterProbablySecond(msg[1])) return true;

    return false;
  }

  return {
    filter: filter,
    errorMsg: "must be a rain world ancient name",
    errorMsgInv: "must not be a rain world ancient name"
  }
}