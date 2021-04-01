const FaveColor = {
  Blue: 'blue',
  Red: 'red',
  Green: 'green',
  Orange: 'orange'
}

const Side = {
  Top: 'top',
  Right: 'right',
  Left: 'left',
  Bottom: 'bottom'
}

class Profile {
  constructor (faveColor, num, side) {
    this.faveColor = faveColor
    this.num = num
    this.side = side
  }
}

class Caller {
  constructor (faveColor, num, side) {
    this.faveColor = faveColor
    this.num = num
    this.side = side
  }
}

// Take some profiles and some callers and return which callers
// could match the profiles.
const matchTable = (profiles, callers) => {
  const patterns = []

  for (const a of callers) {
    const pattern = []
    for (const p of profiles) {
      pattern.push(couldMatch(p, a))
    }
    patterns.push(pattern)
  }
  return patterns
}

// In order for a table of caller-to-profile matches to be valid,
// at least one caller has to match every profile.
const isValid = (matchTable) => {
  for (const answer in matchTable[0]) {
    let allFalse = true
    for (const profile in matchTable) {
      if (matchTable[profile][answer]) {
        allFalse = false
      }
    }
    if (allFalse) {
      return false
    }
  }
  return true
}

// A caller could match a profile if all answered questions match.
// Unanswered questions are ignored.
const couldMatch = (profile, answer) => {
  return (
    (answer.faveColor == null || profile.faveColor === answer.faveColor) &&
    (answer.num == null || profile.num === answer.num) &&
    (answer.side == null || profile.side === answer.side)
  )
}

// There are four profiles and four callers...
const profiles = []

profiles.push(new Profile(FaveColor.Blue, 3, Side.Top))
profiles.push(new Profile(FaveColor.Orange, 2, Side.Left))
profiles.push(new Profile(FaveColor.Orange, 1, Side.Top))
profiles.push(new Profile(FaveColor.Red, 3, Side.Right))

const callers = []

callers.push(new Caller(FaveColor.Red, null, Side.Right))
callers.push(new Caller(FaveColor.Orange, 2, null))
callers.push(new Caller(null, 1, Side.Top))
callers.push(new Caller(null, 3, null))

// TODO: How do I iterate over the callers, assuming each caller is the hacker one at a time...
// I can make a truth table that says whether each caller could match each profile. Then I can
// test to see if the table makes sense with the is_valid function. is_valid will check to see
// if there's one caller per profile. If assuming one of the callers is lying yields a valid
// truth table, they could be the hacker. If only one assumptions makes sense, then only one
// profile could be the hacker...
//
// match_table makes truth tables and is_valid validates them.

