#[derive(PartialEq)]
#[derive(Debug)]
enum FaveColor {
    Blue,
    Red,
    Green,
    Orange
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Side {
    Top,
    Right,
    Left,
    Bottom
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Profile {
    fave_color: FaveColor,
    num: i32,
    side: Side
}

impl Profile {
    fn new(fave_color: FaveColor, num: i32, side: Side) -> Self {
        Profile {
            fave_color,
            num,
            side
        }
    }
}

#[derive(Debug)]
struct Caller {
    fave_color: Option<FaveColor>,
    num: Option<i32>,
    side: Option<Side>
}

impl Caller {
    fn new(fave_color: Option<FaveColor>, num: Option<i32>, side: Option<Side>) -> Self {
        Caller {
            fave_color,
            num,
            side
        }
    }
}


fn main() {

    // There are four profiles and four callers...
    let mut profiles = Vec::new();

    profiles.push(Profile::new(FaveColor::Blue, 3, Side::Top));
    profiles.push(Profile::new(FaveColor::Orange, 2, Side::Left));
    profiles.push(Profile::new(FaveColor::Orange, 1, Side::Top));
    profiles.push(Profile::new(FaveColor::Red, 3, Side::Right));

    let mut callers = Vec::new();

    callers.push(Caller::new(Some(FaveColor::Red), None, Some(Side::Right)));
    callers.push(Caller::new(Some(FaveColor::Orange), Some(2), None));
    callers.push(Caller::new(None, Some(1), Some(Side::Top)));
    callers.push(Caller::new(None, Some(3), None));

    // TODO: How do I iterate over the callers, assuming each caller is the hacker one at a time...
    // I can make a truth table that says whether each caller could match each profile. Then I can
    // test to see if the table makes sense with the is_valid function. is_valid will check to see
    // if there's one caller per profile. If assuming one of the callers is lying yields a valid
    // truth table, they could be the hacker. If only one assumptions makes sense, then only one
    // profile could be the hacker...
    //
    // match_table makes truth tables and is_valid validates them.

}

// In order for a table of caller-to-profile matches to be valid,
// at least one caller has to match every profile.
fn is_valid(match_table: &Vec<Vec<bool>>) -> bool {
    for caller in 0..match_table[0].len() {
        let mut all_false = true;
        for profile in 0..match_table.len() {
            if match_table[profile][caller] {
                all_false = false;
            }
        }
        if all_false {
            return false
        }
    }
    true
}

// Take some profiles and some callers and return which callers
// could match the profiles.
fn match_table(profiles: &Vec<Profile>, callers: &Vec<Caller>) -> Vec<Vec<bool>> {
    let mut patterns: Vec<Vec<bool>> = Vec::new();

    for a in callers {
        let mut pattern: Vec<bool> = Vec::new();
        for p in profiles {
            pattern.push(could_match(p, a));
        }
        patterns.push(pattern);
    }
    patterns
}

// A caller could match a profile if all answered questions match.
// Unanswered questions are ignored.
fn could_match(profile: &Profile, caller: &Caller) -> bool {
    (caller.fave_color.is_none() || &profile.fave_color == caller.fave_color.as_ref().unwrap()) &&
    (caller.num.is_none() || &profile.num == caller.num.as_ref().unwrap()) &&
    (caller.side.is_none() || &profile.side == caller.side.as_ref().unwrap())
}
