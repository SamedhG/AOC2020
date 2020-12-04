use super::*;


pub fn count_trees(right: usize, down: usize) -> usize { 
    // All indexes are mod 32
    let line_len = 31;
    let mut counter = 0;
    let mut num_trees = 0;
    for line in read_lines("./resources/day3.txt").unwrap().step_by(down) {
        if line.unwrap().chars().nth(counter).unwrap() == '#' { num_trees += 1}
        counter = (counter + right) % line_len;
    }
    num_trees
}

pub fn count_trees_product() -> usize {
    let settings = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    settings.iter().fold(1, |acc, setting| acc * count_trees(setting.0, setting.1))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(count_trees(3, 1), 270);
        assert_eq!(count_trees_product(), 2122848000);
    }
}
