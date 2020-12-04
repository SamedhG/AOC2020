use regex::Regex;
use super::*;

struct Entry {
    min: usize,
    max: usize,
    c: char,
    password: String
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)\-(\d+) (\w): (\w+)$").unwrap();
}

impl Entry {
    fn parse(s: String) -> Self {
        let cap = RE.captures(&s).unwrap();
        let min = cap.get(1).unwrap().as_str().parse().unwrap();
        let max = cap.get(2).unwrap().as_str().parse().unwrap();
        let c = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = String::from(cap.get(4).unwrap().as_str());
        Entry { min, max, c, password }
    }
}

pub fn check_passwords1() -> usize {
    let mut num_valid = 0;
    for line in read_lines("./resources/day2.txt").unwrap() {
        let e = Entry::parse(line.unwrap());
        let count = e.password.chars().filter(|x| *x == e.c).count(); 
        if count >= e.min && count <= e.max { num_valid += 1; }
    }
    num_valid
}

pub fn check_passwords2() -> usize {
    let mut num_valid = 0;
    for line in read_lines("./resources/day2.txt").unwrap() {
        let e = Entry::parse(line.unwrap());
        let char1 = e.password.chars().nth(e.min - 1);
        let char2 = e.password.chars().nth(e.max - 1);
        if (char1 == Some(e.c)) ^ (char2 == Some(e.c)) { num_valid += 1; }
    }
    num_valid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(check_passwords1(), 515);
        assert_eq!(check_passwords2(), 711);
    }
}

