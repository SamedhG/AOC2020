use std::collections::HashSet;
use super::*;


fn day6(f: fn(&str) -> HashSet<char>) -> usize {
    let data = read_string("./resources/day6.txt").unwrap();
    data.split("\n\n").map(|x| f(x).len()).sum()
}


fn union(group: &str) -> HashSet<char> {
    let mut p = HashSet::new();
    for passenger in group.split("\n") {
        passenger.chars().for_each(|c| { p.insert(c); });
    }
    p
}

fn intersection(group: &str) -> HashSet<char> {
    let mut p = HashSet::new();
    let mut iter = group.split("\n");
    iter.next().unwrap().chars().for_each(|c| { p.insert(c); });
    for passenger in iter {
        p = p.intersection(&passenger.chars().collect()).cloned().collect()
    }
    p 
}

pub fn part1() -> usize {
    day6(union)
}
pub fn part2() -> usize {
    day6(intersection)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(part1(), 6596);
        assert_eq!(part2(), 3219);
    }
}
