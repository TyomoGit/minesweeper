use minesweeper::game::Game;

fn main() {
    let mut game = Game::new(10, 10, 10);
    
    loop {
        println!("{}", game);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim().replace(' ', "");
        let mut input = input.chars();

        let Some(row) = input.next().and_then(|c| c.to_digit(10).map(|d| d as usize)) else {
            continue;
        };
        let Some(col) = input.next().map(|c| c.to_ascii_lowercase() as usize - 'a' as usize) else {
            continue;
        };

        match game.open(row, col) {
            Ok(()) => (),
            Err(error) => {
                println!("{:?}", error);
                match error {
                    minesweeper::error::Error::OutOfBounds => continue,
                    minesweeper::error::Error::AlreadyOpened => continue,
                    minesweeper::error::Error::Lose => break,
                    minesweeper::error::Error::Win => break,
                }
            }
        }
    }

    println!("{}", game);
}
