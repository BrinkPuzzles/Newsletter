
// Menu of soaps. first[i] goes with second[i] for all indices i. Example: "Carnival Calliope".
const firsts = ["Dazzling", "Chickadee's", "Pumpkin", "Daily", "Deepest", "Carnival"]
const seconds = ["Daylight", "Song", "Spice", "Lailac", "Purple", "Calliope"]

// List of possible soaps.
const possible_soaps = []

const digest = (s) => {
    const s_lower = s.toLowerCase()
    const sv = s_lower.split('')
    let hash_code = 0

    sv.forEach(c => {
        // make letter a = code 0
        hash_code += c.charCodeAt() - 'a'.charCodeAt()
    })
    return hash_code
}

firsts.forEach(f => {
    seconds.forEach(s => {
        possible_soaps.push([[digest(f), digest(s)], f + ' ' + s])
    })
})

// Each soap has a code: "(D1, D2)". Where D1 and D2 are integers given by
// the digest function below.

// Initialize a container to hold the digests.
let first_digests = new Array(firsts.len).fill(0)
let second_digests = new Array(seconds.len).fill(0)

firsts.forEach((e, i) => {
    first_digests[i] = digest(e)
})

// Remove extra elements (zeros).
first_digests = first_digests.filter(x => x != 0)

// Repeat for second words.
seconds.forEach((e, i) => {
    second_digests[i] = digest(e)
})

second_digests = second_digests.filter(x => x != 0)

// Loop through the digests and match them up with soap names within possible soaps.
first_digests.forEach((d, i) => {
    let matches = possible_soaps.filter(x => JSON.stringify(x[0]) === JSON.stringify([first_digests[i], second_digests[i]]))
    if (matches.length > 0) {
        console.log(matches[0][1])
    }
})
