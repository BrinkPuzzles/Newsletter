const swap_beginning_end = (l) => {
  process.stdout.write('sport ')
  const beginning = l.slice(0, l.length / 2)
  const end = l.slice(l.length / 2, l.length)
  return end.concat(beginning)
}

const value_to_index = (l) => {
  process.stdout.write('hawk ')
  return l.map(e => l[e])
}

const swap_first_last = (l) => {
  process.stdout.write('glass ')
  const first = l.shift()
  const last = l.pop()
  return [last].concat(l).concat(first)
}

const do_si_do = (l) => {
  process.stdout.write('snap ')
  const mixed = []
  for (let i = 0; i < l.length; i += 2) {
    const e = l[i]
    if (i + 1 < l.length) {
      const nxt = l[i + 1]
      mixed.push(nxt)
    }
    mixed.push(e)
  }
  return mixed
}

const rotate = (l) => {
  process.stdout.write('gossip ')
  l.unshift(l.pop())
  return l
}

const shuffle = (l) => {
  process.stdout.write('alter ')
  const mixed = []
  const beginning = l.slice(0, l.length / 2)
  const end = l.slice(l.length / 2, l.length)
  for (let i = 0; i < end.length; i++) {
    const e = end[i]
    mixed.push(e)
    if (i < beginning.length) {
      mixed.push(beginning[i])
    }
  }
  return mixed
}

const substitution_table = (l) => {
  process.stdout.write('avocado ')
  return [l[4], l[0], l[2], l[3], l[1], l[5]]
}

const substitution_table2 = (l) => {
  process.stdout.write('alcohol ')
  return [l[5], l[1], l[2], l[0], l[4], l[3]]
}

const twin_propeller = (l) => {
  process.stdout.write('dolphin ')
  const beginning = l.slice(0, l.length / 2)
  const end = l.slice(l.length / 2, l.length)
  beginning.reverse()
  end.reverse()
  return beginning.concat(end)
}

const fold_shuffle = (l) => {
  process.stdout.write('material ')
  const mixed = []
  const beginning = l.slice(0, l.length / 2)
  const end = l.slice(l.length / 2, l.length)
  beginning.reverse()
  end.reverse()
  for (let i = 0; i < end.length; i++) {
    const e = end[i]
    mixed.push(e)
    if (i < beginning.length) {
      mixed.push(beginning[i])
    }
  }
  return mixed
}

const propeller_shuffle = (l) => {
  process.stdout.write('scene ')
  const mixed = []
  const beginning = l.slice(0, l.length / 2)
  beginning.reverse()
  const end = l.slice(l.length / 2, l.length)
  for (let i = 0; i < end.length; i++) {
    const e = end[i]
    mixed.push(e)
    if (i < beginning.length) {
      mixed.push(beginning[i])
    }
  }
  return mixed
}

const reverse = (l) => {
  process.stdout.write('floor ')
  l.reverse()
  return l
}

const run_first_6 = (l) => {
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  return l
}

const run_second_6 = (l) => {
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  l = xxxxxxxx(l)
  return l
}

const test_A = () => {
  return run_first_6([0, 1, 2, 3, 4, 5]).toString() === [3, 0, 1, 4, 5, 2].toString()
}

// clw answerwallet --passphrase "X Y Z"
// address 131ykwoBx3uPiWwUEWw63KYLLjX93d3L1E
const test_B = () => {
  return run_second_6(run_first_6([0, 1, 2, 3, 4, 5])).toString() === [5, 4, 3, 2, 1, 0].toString()
}

if (test_A() && test_B()) {
  console.log('PASS')
} else {
  console.log('FAIL')
}
