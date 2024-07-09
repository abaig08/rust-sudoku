fn is_valid(board: &mut Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
    for i in 0..9 {
        if board[i][col] == c || board[row][i] == c || board[3 * (row / 3) + i / 3][3 * (col / 3) + i % 3] == c {
            return false;
        }
    }
    true
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                for c in '1'..='9' {
                    if is_valid(board, i, j, c) {
                        board[i][j] = c;
                        if solve_sudoku(board) {
                            return true;
                        } else {
                            board[i][j] = '.';
                        }
                    }
                }
                return false;
            }
        }
    }
    true
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    if solve_sudoku(&mut board) {
        print_board(&board);
    } else {
        println!("No solution exists");
    }
}