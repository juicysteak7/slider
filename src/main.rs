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

fn make_random_moves(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut this_board = board.clone();
    let mut last_move:u8 = 0;
    let mut failed_move: bool = false;
    let mut moves:u64 = 0;
    while !is_solved(&this_board) {

        //Generate random move
        let mut random_number:u8 = rng.gen_range(0..=3);
        //Check if last move failed and generate a new move
        while failed_move == true && last_move == random_number {
            random_number = rng.gen_range(0..=3);
        }

        this_board = match random_number {
            0 => {
                match make_move(&this_board, "up") {
                    Some(board) => {
                        failed_move = false;
                        board
                    }
                    None => {
                        last_move = 0;
                        failed_move = true;
                        this_board
                    }
                }
            },
            1 => {
                match make_move(&this_board, "down") {
                    Some(board) => {
                        failed_move = false;
                        board
                    }
                    None => {
                        failed_move = true;
                        last_move = 1;
                        this_board
                    }
                }
            },
            2 => {
                match make_move(&this_board, "left") {
                    Some(board) => {
                        failed_move = false;
                        board
                    }
                    None => {
                        failed_move = true;
                        last_move = 2;
                        this_board
                    }
                }
            },
            3 => {
                match make_move(&this_board, "right") {
                    Some(board) => {
                        failed_move = false;
                        board
                    }
                    None => {
                        failed_move = true;
                        last_move = 3;
                        this_board
                    }
                }
            },
            _ => this_board,
        };
        moves += 1;
    }
    println!("Total moves made: {}", moves);
    this_board
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
    random_board = make_random_moves(&random_board);
    print_board(&random_board);
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board{
        println!("{:?}", row);
    }
}
