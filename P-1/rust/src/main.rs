
fn main() {
    // Menu of soaps. first[i] goes with second[i] for all indices i. Example: "Carnival Calliope".
    let firsts = vec!["Dazzling".into(), "Chickadee's".into(), "Pumpkin".into(), "Daily".into(), "Deepest".into(), "Carnival".into()];
    let seconds = vec!["Daylight".into(), "Song".into(), "Spice".into(), "Lailac".into(), "Purple".into(), "Calliope".into()];

    // List of possible soaps.
    let mut possible_soaps: Vec<((i32, i32), String)> = Vec::new();

    for i in 0..firsts.len() {
        for j in 0..seconds.len() {
            possible_soaps.push(((digest(&firsts[i]), digest(&seconds[j])), firsts[i].clone() + " " + &seconds[j]));
        }
    }

    // Each soap has a code: "(D1, D2)". Where D1 and D2 are integers given by
    // the digest function below.

    // Initialize a container to hold the digests.
    let mut first_digests = vec![0; firsts.len()];
    let mut second_digests = vec![0; seconds.len()];

    for i in 0..firsts.len() {
        first_digests[i] = digest(&firsts[i]);
    }
    // Remove extra elements (zeros).
    first_digests = first_digests.into_iter().filter(|&x| x != 0).collect();

    // Repeat for second words.
    for i in 0..seconds.len() {
        second_digests[i] = digest(&seconds[i]);
    }
    second_digests = second_digests.into_iter().filter(|&x| x != 0).collect();

    // Loop through the digests and match them up with soap names within possible soaps.
    for i in 0..first_digests.len() {
        if let Some(s) = possible_soaps.iter().find(|&x| x.0 == (first_digests[i], second_digests[i])) {
            println!("{:?}", s.1);
        }
    }
}


fn digest(s: &String) -> i32 {
    let sv:Vec<char> = s.to_lowercase().chars().collect();
    let mut hash = 0;

    for c in sv {
        // make letter a = code 0
        hash += char_code(c) - char_code('a');
    }

    hash
}

fn char_code(c: char) -> i32 {
    let mut b = [0; 1];

    let result = c.encode_utf8(&mut b);
    result.bytes().nth(0).unwrap() as i32
}

