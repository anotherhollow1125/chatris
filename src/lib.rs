pub mod cli;
pub mod game;
pub mod next;
pub mod print_img;

#[cfg(test)]
mod tests {
    use super::cli;
    use super::game::{Game, WorldRuleMinoGenerator};
    use rand::{thread_rng, Rng};

    #[test]
    fn play_game() {
        let mut rng = thread_rng();
        let rand_gen = Box::new(move || rng.gen::<u32>());
        let minogen = WorldRuleMinoGenerator::new(rand_gen);
        let mut g = Game::new(minogen, "".to_string());
        let command = "RLSEW;;";
        while cli::input_string_command(&mut g, command) {}
    }
}
