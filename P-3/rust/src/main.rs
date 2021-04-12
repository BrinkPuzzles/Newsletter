fn swap_beginning_end(l: Vec<i32>) -> Vec<i32> {
    print!("sport ");
    let mut beginning = l.get(..l.len() / 2).unwrap().to_vec();
    let mut end = l.get(l.len() / 2..l.len()).unwrap().to_vec();
    end.append(&mut beginning);
    end
}

fn value_to_index(l: Vec<i32>) -> Vec<i32> {
    print!("hawk ");
    l.iter().map(|&e| l[e as usize]).collect()
}

fn swap_first_last(l: Vec<i32>) -> Vec<i32> {
    print!("glass ");
    let mut v = vec![];
    v.push(l[l.len() - 1]);
    v.append(&mut l.get(1..l.len() - 1).unwrap().to_vec());
    v.push(l[0]);
    v
}

fn do_si_do(l: Vec<i32>) -> Vec<i32> {
    print!("snap ");
    let mut mixed: Vec<i32> = vec![];
    let mut it = l.iter();
    while let Some(e) = it.next() {
        let nxt = it.next();
        if nxt.is_some() {
            mixed.push(*nxt.unwrap());
        }
        mixed.push(*e);
    }
    mixed
}

fn rotate(mut l: Vec<i32>) -> Vec<i32> {
    print!("gossip ");
    let last = l.pop().unwrap();
    l.insert(0, last);
    return l;
}

fn shuffle(l: Vec<i32>) -> Vec<i32> {
    print!("alter ");
    let mut mixed: Vec<i32> = vec![];
    let beginning = l.get(..l.len() / 2).unwrap().to_vec();
    let end = l.get(l.len() / 2..l.len()).unwrap().to_vec();
    for (i, e) in end.iter().enumerate() {
        mixed.push(*e);
        if i < beginning.len() {
            mixed.push(beginning[i]);
        }
    }
    mixed
}

fn substitution_table(l: Vec<i32>) -> Vec<i32> {
    print!("avocado ");
    return vec![l[4], l[0], l[2], l[3], l[1], l[5]];
}

fn substitution_table2(l: Vec<i32>) -> Vec<i32> {
    print!("alcohol ");
    return vec![l[5], l[1], l[2], l[0], l[4], l[3]];
}

fn twin_propeller(l: Vec<i32>) -> Vec<i32> {
    print!("dolphin ");
    let mut beginning = l.get(..l.len() / 2).unwrap().to_vec();
    let mut end = l.get(l.len() / 2..l.len()).unwrap().to_vec();
    beginning.reverse();
    end.reverse();
    beginning.append(&mut end);
    beginning
}

fn fold_shuffle(l: Vec<i32>) -> Vec<i32> {
    print!("material ");
    let mut mixed: Vec<i32> = vec![];
    let mut beginning = l.get(..l.len() / 2).unwrap().to_vec();
    let mut end = l.get(l.len() / 2..l.len()).unwrap().to_vec();
    beginning.reverse();
    end.reverse();
    for (i, e) in end.iter().enumerate() {
        mixed.push(*e);
        if i < beginning.len() {
            mixed.push(beginning[i]);
        }
    }
    mixed
}

fn propeller_shuffle(l: Vec<i32>) -> Vec<i32> {
    print!("scene ");
    let mut mixed: Vec<i32> = vec![];
    let mut beginning = l.get(..l.len() / 2).unwrap().to_vec();
    beginning.reverse();
    let end = l.get(l.len() / 2..l.len()).unwrap().to_vec();

    for (i, e) in end.iter().enumerate() {
        mixed.push(*e);
        if i < beginning.len() {
            mixed.push(beginning[i]);
        }
    }
    mixed
}

fn reverse(mut l: Vec<i32>) -> Vec<i32> {
    print!("floor ");
    l.reverse();
    l
}

fn run_first_6(l: Vec<i32>) -> Vec<i32> {
    let mut v = xxxxxxxx(l);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    return v;
}

fn run_second_6(l: Vec<i32>) -> Vec<i32> {
    let mut v = xxxxxxxx(l);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    v = xxxxxxxx(v);
    return v;
}

fn test_a() -> bool {
    run_first_6(vec![0, 1, 2, 3, 4, 5]) == vec![3, 0, 1, 4, 5, 2]
}

// clw answerwallet --passphrase "X Y Z"
// address 131ykwoBx3uPiWwUEWw63KYLLjX93d3L1E
fn test_b() -> bool {
    run_second_6(run_first_6(vec![0, 1, 2, 3, 4, 5])) == vec![5, 4, 3, 2, 1, 0]
}

fn main() {
    if test_a() && test_b() {
        println!("PASS");
    } else {
        println!("FAIL");
    }
}
