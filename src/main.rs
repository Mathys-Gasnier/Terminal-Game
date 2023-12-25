use std::{
    io,
    time::{Duration, Instant},
};

mod game;
mod lexer;
mod parser;
mod term;
use game::Game;

const FIXED_TIME: Duration = Duration::from_secs(1);

fn main() -> io::Result<()> {
    let mut game = Game::new()?;

    game.draw()?;
    let mut last_fixed = Instant::now();
    loop {
        let elapsed = last_fixed.elapsed();
        if elapsed > FIXED_TIME {
            game.fixed_update()?;
            last_fixed += FIXED_TIME;
        }
        if game.update()? {
            break;
        }
        game.draw()?;
    }

    Ok(())
}
