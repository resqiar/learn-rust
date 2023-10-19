use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Team {
    RedBull,
    YellowBanyan,
}

pub fn main() {
    let mut scores: HashMap<Team, i32> = HashMap::new();

    for i in 0..100 {
        let team = if i % 3 == 0 {
            Team::RedBull
        } else {
            Team::YellowBanyan
        };

        let old_val = scores.entry(team).or_insert(1);
        *old_val += 1;
    }

    println!("{:?}", scores);
}
