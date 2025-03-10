use std::str::FromStr;

use super::game::{Mino, MinoGenerator};
use anyhow::Result;
use dialoguer::Input;

pub struct LoopGenerator {
    loop_unit: Vec<&'static Mino>,
}

impl LoopGenerator {
    pub fn new(loop_unit: Vec<&'static Mino>) -> Self {
        Self { loop_unit }
    }

    pub fn dialog() -> Result<Self> {
        let input: String = Input::new().with_prompt("Input mino sequence").interact()?;
        Ok(Self::from_str(&input).unwrap())
    }
}

impl FromStr for LoopGenerator {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let loop_unit = s
            .chars()
            .filter_map(|c| Mino::from_char(&c))
            .collect::<Vec<_>>();
        Ok(Self::new(loop_unit))
    }
}

impl MinoGenerator for LoopGenerator {
    fn generate(&mut self) -> &'static Mino {
        let mino = self.loop_unit.remove(0);
        self.loop_unit.push(mino);
        mino
    }
}
