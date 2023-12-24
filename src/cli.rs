use super::game::{Block, FieldArray, Game, InputKey::*, MinoColors};

pub fn input_string_command(game: &mut Game, input: &str) -> bool {
    for c in input.chars() {
        let res = match c {
            'r' | 'R' => game.handle_input(&RSpin),
            'l' | 'L' => game.handle_input(&LSpin),
            'v' | 'V' => game.handle_input(&Down),
            '!' => game.handle_input(&HardDrop),
            '>' => game.handle_input(&Right),
            '<' => game.handle_input(&Left),
            'H' => game.handle_input(&Hold),
            ';' => game.handle_input(&Commit),
            _ => true,
        };

        if !res {
            return false;
        }
    }

    true
}

pub fn set_field(game: &mut Game, field_str: &str) {
    let mut field: FieldArray = [[Block::new(false, MinoColors::TRANS); 10]; 21];
    for (x, line) in field_str.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            field[x][y] = Block::from_char(&c);
        }
    }

    let boolean_field = field
        .iter()
        .map(|row| {
            row.iter()
                .map(|block| block.is_filled())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{:?}", boolean_field);

    game.set_field(field);
}
