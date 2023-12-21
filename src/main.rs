use std::io;

mod term;
mod game;
use game::Game;
mod lexer;
mod parser;

fn main() -> io::Result<()> {
    let mut game = Game::new()?;

    game.draw()?;
    loop {
        if game.update()? {
            break;
        }
        game.draw()?;
    }

    Ok(())
}
