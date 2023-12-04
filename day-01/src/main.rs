//// PART 1

pub fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

pub fn sum(a: i64, b: i64) -> i64 {
    a + b
}

pub fn calibrate_p1(doc: &str) -> i64 {
    doc.split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first = line.chars().find(is_digit).unwrap();
            let last = line.chars().rev().find(is_digit).unwrap();

            let num: i64 = format!("{}{}", first, last).parse().unwrap();

            num
        })
        .reduce(sum)
        .unwrap()
}

// PART 2

const DIGIT_STRINGS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn find_digit(string: &str) -> Option<i64> {
    let chars: Vec<char> = string.chars().collect();
    if let Some(d) = chars.get(0) {
        if let Ok(x) = d.to_string().parse() {
            return Some(x);
        }
    }

    for (i, d) in DIGIT_STRINGS.into_iter().enumerate() {
        if (string).starts_with(d) {
            return Some(i as i64);
        }
    }

    None
}

pub fn fix_str(line: &str) -> (i64, i64) {
    let first = (0..line.len())
        .find_map(|i| find_digit(&line[i..line.len()]))
        .unwrap();
    let last = (0..line.len())
        .rev()
        .find_map(|i| find_digit(&line[i..line.len()]))
        .unwrap();
    (first, last)
}

pub fn calibrate_p2(doc: &str) -> i64 {
    doc.split('\n')
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, last) = fix_str(line);

            println!("{} : {}{}", line, first, last);

            let num: i64 = format!("{}{}", first, last).parse().unwrap();

            num
        })
        .reduce(sum)
        .unwrap()
}

//// MAIN

fn main() {
    println!("{}", calibrate_p2(include_str!("input.txt")))
}

//// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(calibrate_p1(sample_input), 142);
    }

    #[test]
    fn part_two() {
        let sample_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(calibrate_p2(sample_input), 281);
    }

    #[test]
    fn fix_str_test() {
        assert_eq!(fix_str("onetwothreefourfivesixseveneightninezero"), (1, 0));
        assert_eq!(fix_str("two1nine"), (2, 9));
        assert_eq!(fix_str("2oneight"), (2, 8));
        assert_eq!(fix_str("eightwothree"), (8, 3));
    }
}
