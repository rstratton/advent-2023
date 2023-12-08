fn part1() -> u32 {
    let games = parse_input();
    games
        .iter()
        .filter(game_is_possible)
        .map(|g| g.number)
        .sum()
}

fn game_is_possible(game: &&Game) -> bool {
    game.rounds.iter().all(round_is_possible)
}

fn round_is_possible(round: &Round) -> bool {
    round.quantities.iter().all(cube_quantity_is_possible)
}

fn cube_quantity_is_possible(cube_quantity: &CubeQuantity) -> bool {
    match cube_quantity.color {
        Color::Red => cube_quantity.quantity <= 12,
        Color::Green => cube_quantity.quantity <= 13,
        Color::Blue => cube_quantity.quantity <= 14,
    }
}

fn part2() -> u32 {
    let games = parse_input();
    games.iter().map(power).sum()
}

fn power(game: &Game) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for round in game.rounds.iter() {
        for quantity in round.quantities.iter() {
            match quantity.color {
                Color::Red => min_red = std::cmp::max(min_red, quantity.quantity),
                Color::Green => min_green = std::cmp::max(min_green, quantity.quantity),
                Color::Blue => min_blue = std::cmp::max(min_blue, quantity.quantity),
            }
        }
    }
    min_red * min_green * min_blue
}

// fn power(cubes: &Vec<CubeQuantity>) -> u32 {
//     cubes.into_iter().map(|c| c.quantity).product()
// }

enum Color {
    Red,
    Green,
    Blue,
}

struct CubeQuantity {
    quantity: u32,
    color: Color,
}

impl CubeQuantity {
    fn new(quantity: u32, color: Color) -> Self {
        Self { quantity, color }
    }
}

struct Round {
    quantities: Vec<CubeQuantity>,
}

impl Round {
    fn new(quantities: Vec<CubeQuantity>) -> Self {
        Self { quantities }
    }
}

struct Game {
    number: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(number: u32, rounds: Vec<Round>) -> Self {
        Self { number, rounds }
    }
}

fn parse_input() -> Vec<Game> {
    let input = include_str!("../../data/day_02.txt");

    peg::parser! {
        grammar parser() for str {
            pub(crate) rule parse() -> Vec<Game>
                = g:game() ++ "\r\n" "\r\n"? { g }

            rule game() -> Game
                = "Game " n:number() ": " r:rounds() { Game::new(n, r) }

            rule rounds() -> Vec<Round>
                = r:round() ++ "; " { r }

            rule round() -> Round
                = q:quantities() { Round::new(q) }

            rule quantities() -> Vec<CubeQuantity>
                = q:quantity() ++ ", " { q }

            rule quantity() -> CubeQuantity
                = n:number() " " c:color() { CubeQuantity::new(n, c) }

            rule number() -> u32
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule color() -> Color
                = red() / green() / blue()

            rule red() -> Color
                = "red" { Color::Red }

            rule green() -> Color
                = "green" { Color::Green }

            rule blue() -> Color
                = "blue" { Color::Blue }
        }
    }

    parser::parse(input).unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
