use minesweeper::game::Game;

fn main() {
    let Some((init_row, init_col)) = input() else {
        panic!("Invalid input");
    };
    let mut game = Game::new(10, 10, 10, init_row, init_col);
    
    loop {
        println!("{}", game);

        let Some((row, col)) = input() else {
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

fn input() -> Option<(usize, usize)> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    let input = input.trim().replace(' ', "");
    let mut input = input.chars();

    let Some(row) = input.next().and_then(|c| c.to_digit(10).map(|d| d as usize)) else {
        return None;
    };
    let Some(col) = input.next().map(|c| c.to_ascii_lowercase() as usize - 'a' as usize) else {
        return None;
    };

    Some((row, col))
}
