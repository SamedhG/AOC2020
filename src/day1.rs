use anyhow::Result;
use crate::read_nums;


/// Checks if 2 numbers in the given list add to x
fn sum_to(list: &Vec<isize>, x: isize) -> Option<isize> {
    let mut iter = list.iter().enumerate();
    iter.find(|(i,v)| {
        list.iter().skip(*i).find(|u| *u + *v == x).is_some()
    }).map(|(_, v)| *v)
}

/// Checks if 2 numbers in the file sum to 2020 then returns their product
pub fn sum2() -> Result<isize> {
    let data = read_nums("resources/day1.txt")?;
    let val = sum_to(&data, 2020).unwrap();
    dbg!(val);
    Ok(val * (2020 - val))
}

/// Checks if 3 numbers in the file sum to 2020 then returns their product
pub fn sum3() -> Result<isize> {
    let data = read_nums("resources/day1.txt")?;
    let mut iter = data.iter().enumerate();
    let (idx, val1) = iter.find(|(i, v)| {
        let remaining : Vec<isize> = data.clone().into_iter().skip(*i).collect();
        sum_to(&remaining, 2020 - *v).is_some()
    }).unwrap();
    let remaining : Vec<isize> = data.clone().into_iter().skip(idx).collect();
    let val2 = sum_to(&remaining, 2020 - val1).unwrap();
    Ok(val1 * val2 * (2020 - val1 - val2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_task1() {
        let num = sum2().unwrap();
        assert_eq!(num, 960075)
    }

    #[test]
    fn run_task2() {
        let num = sum3().unwrap();
        assert_eq!(num, 212900130)
    }

}

