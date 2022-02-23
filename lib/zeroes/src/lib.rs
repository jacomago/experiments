use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

use bigdecimal::{BigDecimal, ToPrimitive};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zero {
    pos: u64,
    bigd: BigDecimal,
}

impl Zero {
    fn new(pos: u64, bigd: BigDecimal) -> Self {
        Self { pos, bigd }
    }

    fn int_val(&self) -> i64 {
        self.bigd.to_i64().unwrap()
    }

    fn float_val(&self) -> f64 {
        self.bigd.to_f64().unwrap()
    }

    fn fract(&self) -> BigDecimal {
        self.bigd.clone() - BigDecimal::from(self.int_val())
    }

    fn fract_float(&self) -> f32 {
        self.fract().to_f32().unwrap()
    }
}

pub struct Zeroes {
    track_pos: usize,
    zeroes: Vec<Zero>,
}

fn parse_line(line: String) -> Result<Zero, ParseIntError> {
    let mut split = line.split(' ');
    let pos: u64 = split.next().unwrap().parse::<u64>()?;
    let part2 = split.next().unwrap();
    let dec = BigDecimal::from_str(part2);
    Ok(Zero::new(pos, dec.unwrap()))
}

fn read_zeroes() -> Vec<Zero> {
    let file = File::open("zeros10000.txt").expect("file at zeroes 10000 txt");

    let buf_reader = BufReader::new(file);
    let mut zeroes = Vec::new();
    for line in buf_reader.lines().map(|l| l.unwrap()) {
        let pair = parse_line(line).expect("no parse errors");
        zeroes.push(pair);
    }
    zeroes
}

impl Zeroes {
    pub fn load() -> Self {
        Self {
            track_pos: 0,
            zeroes: read_zeroes(),
        }
    }
}
impl Iterator for Zeroes {
    type Item = Zero;

    fn next(&mut self) -> Option<Self::Item> {
        self.track_pos += 1;

        if self.track_pos < self.zeroes.len() {
            Some(self.zeroes[self.track_pos].clone())
        } else {
            None
        }
    }
}

#[test]
fn test_first_zero() {
    use num_bigint::BigInt;
    let zeroes = Zeroes::load();
    let first_zero = Zero::new(
        1,
        BigDecimal::new(BigInt::from(141347251417346937904572519835625_i128), 31),
    );
    assert_eq!(first_zero, *zeroes.zeroes.first().unwrap());
}
