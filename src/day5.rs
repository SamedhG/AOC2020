use super::*;

fn convert_seat(seat: String) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut counter = 1;
    for c in seat.chars().rev().take(3) {
        match c {
            'R' => col += counter,
            'L' => (),
            _ => unreachable!()
        }
        counter *= 2;
    }
    counter = 1;
    for c in seat.chars().rev().skip(3) {
        match c {
            'B' => row += counter,
            'F' => (),
            _ => unreachable!()
        }
        counter *= 2;
    }
    (row * 8) + col
}

pub fn part1() -> usize {
    let mut v = Vec::new();
    for line in read_lines("resources/day5.txt").unwrap() {
        v.push(convert_seat(line.unwrap()));
    }
    *v.iter().max().unwrap_or(&0)
}

pub fn part2() -> usize {
    let mut v = Vec::new();
    for line in read_lines("resources/day5.txt").unwrap() {
        v.push(convert_seat(line.unwrap()));
    }
    v.sort();
    let mut curr = v[0];
    for n in v.into_iter().skip(1) {
        if n - curr == 2 { return curr+1 }
        curr = n;
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(part1(), 955);
        assert_eq!(part2(), 569);
    }
}
