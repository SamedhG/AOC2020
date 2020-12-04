use std::collections::HashMap;
use regex::Regex;
use super::*;

lazy_static! { 
    static ref RE_HEX: Regex = Regex::new(r"^#[a-f\d]{6}$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref RE_HGT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
}
pub fn parseports() -> Vec<HashMap<String, String>> {
    let data = read_string("./resources/day4.txt").unwrap();
    let mut v = Vec::new();
    for passport in data.split("\n\n").collect::<Vec<&str>>() {
        let mut p = HashMap::new();
        for pair in passport.split_whitespace() {
            let mut split = pair.split(":");
            let key = String::from(split.next().unwrap());
            let value = String::from(split.next().unwrap());
            p.insert(key, value);
        }
        v.push(p);
    }
    v
}

fn validate_present(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr") &&
    passport.contains_key("iyr") &&
    passport.contains_key("eyr") &&
    passport.contains_key("hgt") &&
    passport.contains_key("hcl") &&
    passport.contains_key("ecl") &&
    passport.contains_key("pid")
}


fn check_height(height: &str) -> bool {
    let captures = match RE_HGT.captures(height) {
        Some(h) => h,
        None => { return false; }
    };
    let value: usize = captures[1].parse().unwrap_or(0);
    if &captures[2] == "cm" {
        value >= 150 && value <= 193
    } else {
        value >=59 && value <= 76
    }
}

fn validate_values(passport: &HashMap<String, String>) -> bool {
    if !validate_present(&passport) { return false };
    let byr : usize = passport["byr"].parse().unwrap();
    let iyr : usize = passport["iyr"].parse().unwrap();
    let eyr : usize = passport["eyr"].parse().unwrap();
    let hcl = RE_HEX.is_match(passport["hcl"].as_str());
    let ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"].as_str());
    let pid = RE_PID.is_match(passport["pid"].as_str());
    let hgt = check_height(passport["hgt"].as_str());
    byr >= 1920 && byr <= 2002 &&
    iyr >= 2010 && iyr <= 2020 &&
    eyr >= 2020 && eyr <= 2030 &&
    hcl && ecl && pid && hgt
}

pub fn validate_fields1() -> usize {
    let passports = parseports();
    passports.iter().filter(|x| validate_present(x)).count()    
}

pub fn validate_fields2() -> usize {
    let passports = parseports();
    passports.iter().filter(|x| validate_values(x)).count()    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(validate_fields1(), 213);
        assert_eq!(validate_fields2(), 147);
    }
}
