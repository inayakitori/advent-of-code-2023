use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::iter::Sum;
use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let games : Vec<Game> = input.lines()
        .flat_map(|line| {
            let game: Option<Game> = line.parse().ok();
            if let None = &game {
                println!("failed to parse \"{line}\"")
            }
            game
        }).collect();

    println!("{} games", games.len());

    let power: u32 = games.iter().map(Game::power).sum();

    println!("sum of game #s: {power}")
}


struct Game{
    index: u32,
    handfuls: Vec<Handful>
}

impl Game{
    fn is_valid(&self, max_hand: &Handful) -> bool {
        self.handfuls.iter().all(|hand|hand.is_valid(max_hand))
    }
    fn power(&self) -> u32 {
        let max_handful: Handful = self.handfuls.iter().cloned().sum();
        max_handful.power()
    }
}

impl Display for Game{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: ", self.index)?;
        self.handfuls.iter().try_for_each(|hand|
            write!(f, "{}, ", hand)
        )
    }
}

impl FromStr for Game{
    type Err = ();

    fn from_str(game_info: &str) -> Result<Self, Self::Err> {
        let split_info: Vec<&str> = game_info.split(":").collect();
        let (index_text, handful_text) = game_info.split_once(":").clone().ok_or(())?;
        let index: u32 = index_text
            .strip_prefix("Game ").ok_or(())?
            .parse().map_err(|_|())?;
        let handfuls: Vec<Handful> = handful_text.split(";")
            .map(|handful_txt| handful_txt.parse().ok())
            .collect::<Option<Vec<Handful>>>().ok_or(())?;

        Ok(Game{
            index,
            handfuls,
        })
    }
}

#[derive(Default, Clone)]
struct Handful{
    values: [u32;3],
    red: u32,
    green: u32,
    blue: u32
}
impl Handful{
    fn new(values: &[u32;3]) -> Handful {
        Handful{
            values: values.clone(),
            red: values[0].clone(),
            green: values[1].clone(),
            blue: values[2].clone(),
        }
    }

    fn is_valid(&self, max_hand: &Handful) -> bool {
        &self.red   <= &max_hand.red &&
        &self.green <= &max_hand.green &&
        &self.blue  <= &max_hand.blue
    }

    fn power(&self) -> u32 {
        &self.red * &self.green * &self.blue
    }

}
impl FromStr for Handful{
    type Err = ();

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        hand.split(",")
            .map_while(|single_color| {
            let (count_txt, color) = single_color.strip_prefix(" ")?
                .split_once(" ")?;
            let color_index: usize = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => return None,
            };
            let count: u32 = count_txt.parse().ok()?;
            let mut handful_data = [0u32,0,0];
                handful_data[color_index] += count;
            Some(Ok(Handful::new(&handful_data)))
        }).sum()
    }
}
impl Display for Handful {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "r:{:>2} g:{:>2} b:{:>2}", self.red, self.blue, self.green)
    }
}
impl Debug for Handful {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb: [{},{},{}], values: {:?}", self.red, self.blue, self.green, self.values)
    }
}
impl Deref for Handful {
    type Target = [u32;3];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl DerefMut for Handful {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
impl Add for Handful{
    type Output = Handful;
    fn add(self, rhs: Self) -> Self::Output {
        Handful::new(&[
            self.red.max(rhs.red),
            self.green.max(rhs.green),
            self.blue.max(rhs.blue)
        ])
    }
}
impl Sum for Handful {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Handful::default(), |acc, hand|acc + hand)
    }
}