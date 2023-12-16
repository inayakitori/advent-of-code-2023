use std::fs;
use std::str::FromStr;
use regex::{Match, Regex};


fn main() {
    let input = fs::read_to_string("input").unwrap();

    //have to do multiple regex searches otherwise examples
    // like "threeight" will not read second number
    let searches: Vec<Regex> = vec![
        "[0-9]",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ].iter().map(|txt| Regex::new(txt).unwrap())
        .collect();

    //for every line
    let calibration_values = input.lines()
        .map(|line| {
            //for every pair of (num index, num value) in the line (getting by each regex search)
            let numbers_in_line: Vec<(u32, u32)> = searches.iter().enumerate().flat_map( |(i,re)| {
                re.find_iter(line).map(|re_match| {
                    //if it's a digit search, use digit, otherwise use index of regex
                    let value = match i {
                        0 => line.chars().nth(re_match.start()).unwrap()
                            .to_digit(10).unwrap(),
                        _ => i as u32
                    };
                    (re_match.start() as u32, value)
                }).collect::<Vec<(u32, u32)>>()
            }).collect();

            let first_number = numbers_in_line.iter()
                .min_by(|(i1,_), (i2,_)| i1.cmp(i2));
            let last_number = numbers_in_line.iter()
                .max_by(|(i1,_), (i2,_)| i1.cmp(i2));
            let conc_number = first_number.map_or(0,|(_,first)|
                first * 10 + &last_number.unwrap().1
            );
            conc_number
        }).collect::<Vec<_>>();

    println!("calibration values: {calibration_values:?}");
    let sum: u32 = calibration_values.iter().sum();
    println!("sum: {sum}")
}


