use rand::seq::SliceRandom;
use rand::Rng;

fn generate_random_board(size: usize) -> Vec<Vec<u8>> {
    let mut board: Vec<u8> = (0..(size * size) as u8).collect();
    let mut rng = rand::thread_rng();
    board.shuffle(&mut rng);

    let mut result: Vec<Vec<u8>> = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            result[i][j] = board[i * size + j];
        }
    }

    result
}

fn is_solvable(board: &Vec<Vec<u8>>) -> bool {
    let mut flattened: Vec<u8> = board.iter().flatten().cloned().collect();
    flattened.retain(|&x| x != 0); // Remove the empty tile (represented as 0)

    let mut inversions = 0;
    let len = flattened.len();

    for i in 0..len {
        for j in i + 1..len {
            if flattened[i] > flattened[j] {
                inversions += 1;
            }
        }
    }

    // For boards with odd dimensions (e.g., 3x3), add the row number of the empty tile
    if board.len() % 2 == 1 {
        let empty_row = board.iter().position(|row| row.contains(&0)).unwrap();
        inversions += empty_row;
    }

    inversions % 2 == 0 // Check if the number of inversions is even (solvable)
}

fn make_random_move(board: &Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>> {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=3);

    let mut result = match random_number {
        0 => make_move(&board,"up"),
        1 => make_move(&board,"down"),
        2 => make_move(&board,"left"),
        3 => make_move(&board,"right"),
        _ => None,
    };

    result = match result {
        Some(new_board) => {
            Some(new_board)
        },
        None => {
            make_random_move(board)
        }
    };
    result
}

fn make_move(board: &Vec<Vec<u8>>, direction: &str) -> Option<Vec<Vec<u8>>> {
    let size = board.len();

    // Find the coordinates of the empty tile (0)
    let mut empty_row = 0;
    let mut empty_col = 0;
    for i in 0..size {
        for j in 0..size {
            if board[i][j] == 0 {
                empty_row = i;
                empty_col = j;
            }
        }
    }

    // Calculate the new coordinates after the move
    let (new_row, new_col) = match direction {
        "up" if empty_row > 0 => (empty_row - 1, empty_col),
        "down" if empty_row < size - 1 => (empty_row + 1, empty_col),
        "left" if empty_col > 0 => (empty_row, empty_col - 1),
        "right" if empty_col < size - 1 => (empty_row, empty_col + 1),
        _ => return None, // Invalid move
    };

    // Clone the current board to create a new one
    let mut new_board = board.clone();

    // Swap the empty tile (0) with the tile to be moved
    new_board[empty_row][empty_col] = board[new_row][new_col];
    new_board[new_row][new_col] = 0;

    Some(new_board)
}

fn is_solved(board: &Vec<Vec<u8>>) -> bool {
    let size = board.len();
    let mut expected_value: u8 = 1;

    for row in board {
        for &tile in row {
            if tile != expected_value {
                // If any tile is not in the expected order, the board is not solved
                return false;
            }
            expected_value += 1;
            // Skip checking the last tile, which should be empty (0)
            if expected_value as usize == size * size {
                break;
            }
        }
    }

    true
}

fn main() {
    let size = 2; // Change the size as needed
    let mut random_board = generate_random_board(size);
    while !is_solvable(&random_board) {
        random_board = generate_random_board(size);
    }

    print_board(&random_board);
    while !is_solved(&random_board) {
        match make_random_move(&random_board) {
            Some(new_board) => {
                random_board = new_board
            }
            None => {
                println!("Caught an invalid move");
            }
        }
    }
    print_board(&random_board);
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board{
        println!("{:?}", row);
    }
}
