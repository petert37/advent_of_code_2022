use std::fs;

fn snafu_digit_to_decimal(digit: &char) -> i64 {
    match digit {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Invalid snafu digit"),
    }
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| 5i64.pow(i as u32) * snafu_digit_to_decimal(&c))
        .sum()
}

fn decimal_to_snafu(decimal: u64) -> String {
    let mut x = decimal;
    let mut overflow = 0;
    let mut result = vec![];
    loop {
        let m = (x + overflow) % 5;
        x = (x + overflow) / 5;
        let c = if m > 2 {
            overflow = 1;
            match m - 2 {
                1 => '=',
                2 => '-',
                _ => panic!("too much overflow"),
            }
        } else {
            overflow = 0;
            char::from_digit(m as u32, 3).unwrap()
        };
        result.push(c);
        if x == 0 && overflow == 0 {
            break;
        }
    }
    result.iter().rev().collect()
}

fn main() {
    let input = fs::read_to_string("src/day25/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let sum = lines.iter().map(|line| snafu_to_decimal(line)).sum::<i64>();
    let snafu_sum = decimal_to_snafu(sum as u64);
    println!("{}", snafu_sum);
}

#[cfg(test)]
mod test {
    use crate::{decimal_to_snafu, snafu_to_decimal};

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1), "1");
        assert_eq!(decimal_to_snafu(2), "2");
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(4), "1-");
        assert_eq!(decimal_to_snafu(5), "10");
        assert_eq!(decimal_to_snafu(6), "11");
        assert_eq!(decimal_to_snafu(7), "12");
        assert_eq!(decimal_to_snafu(8), "2=");
        assert_eq!(decimal_to_snafu(9), "2-");
        assert_eq!(decimal_to_snafu(10), "20");
        assert_eq!(decimal_to_snafu(15), "1=0");
        assert_eq!(decimal_to_snafu(20), "1-0");
        assert_eq!(decimal_to_snafu(2022), "1=11-2");
        assert_eq!(decimal_to_snafu(12345), "1-0---0");
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(snafu_to_decimal("12111"), 906);
        assert_eq!(snafu_to_decimal("2=0="), 198);
        assert_eq!(snafu_to_decimal("21"), 11);
        assert_eq!(snafu_to_decimal("2=01"), 201);
        assert_eq!(snafu_to_decimal("111"), 31);
        assert_eq!(snafu_to_decimal("20012"), 1257);
        assert_eq!(snafu_to_decimal("112"), 32);
        assert_eq!(snafu_to_decimal("1=-1="), 353);
        assert_eq!(snafu_to_decimal("1-12"), 107);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("122"), 37);
    }
}
