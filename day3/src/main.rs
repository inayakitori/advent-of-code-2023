use std::fmt::{Display, Formatter, write};
use std::fs;
use std::ops::{Bound, Deref};
use std::str::from_utf8;
use regex::bytes::Regex;

type Number = (u32, Bounds);
type Symbol = (char, Bounds);
type Numbers = Vec<Number>;
type Symbols = Vec<Symbol>;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let numbers: Numbers = find_bounds_in(&input, Regex::new("[0-9]+").unwrap())
        .iter().copied().map(|(txt, bounds)| {
        (txt.parse::<u32>().unwrap(), bounds)
    }).collect();

    let symbols: Symbols = find_bounds_in(&input, Regex::new(r"\*").unwrap())
        .iter().copied().map(|(txt, bounds)| {
        (txt.chars().next().unwrap(), bounds)
    }).collect();

    let gear_ratio_sum: u32 = symbols.iter().flat_map(|(_,gear_bounds)| {
        let adjacent_gears: Vec<Number> = numbers.iter().cloned()
            .filter(|(_, number_bounds)|
                gear_bounds.expand(1).intersects(number_bounds)
            ).collect();
        return if adjacent_gears.len() == 2 {
            Some(adjacent_gears.iter().map(|(num, bounds)| num ).product::<u32>())
        } else {
            None
        }
    }).sum();


    println!("{gear_ratio_sum}");
}

fn find_bounds_in(input: &str, num_regex: Regex) -> Vec<(&str, Bounds)> {
    input.lines().enumerate().flat_map(|(line_index, line)|
        num_regex.find_iter(line.as_bytes()).map(|re_match| {
            (from_utf8(re_match.as_bytes()).unwrap(),
             Bounds([
                 Position([re_match.start() as i32, line_index as i32]),
                 Position([(re_match.end()-1) as i32, line_index as i32])
             ]))
        }).collect::<Vec<(&str, Bounds)>>()
    ).collect()
}

#[derive(Debug, Copy, Clone, Default)]
struct Position([i32;2]);
impl Deref for Position{
    type Target = [i32;2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for Position{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self[0], self[1])
    }
}
#[derive(Debug, Copy, Clone, Default)]
struct Bounds([Position ; 2]);
impl Bounds{
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Bounds {
        Bounds([Position([x1,y1]),Position([x2,y2])])
    }

    fn expand(&self, i: i32) -> Bounds {
        Bounds::new(
            self.xmin() - i,
            self.ymin() - i,
            self.xmax() + i,
            self.ymax() + i
        )
    }

    fn xmin(&self) -> i32 {
        return self[0][0]
    }
    fn xmax(&self) -> i32 {
        return self[1][0]
    }
    fn ymin(&self) -> i32 {
        return self[0][1]
    }
    fn ymax(&self) -> i32 {
        return self[1][1]
    }

    fn intersects(&self, other: &Bounds) -> bool {
        self.xmax() >= other.xmin() &&
        self.xmin() <= other.xmax() &&
        self.ymax() >= other.ymin() &&
        self.ymin() <= other.ymax()
    }

}
impl Deref for Bounds{
    type Target = [Position;2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for Bounds{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self[0], self[1])
    }
}