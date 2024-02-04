// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn evaluate(board: [[i32; 3]; 3]) -> i32 {
    for row in board {
        if row.iter().all(|&cell| cell == 1) {
            return 10;
        } else if row.iter().all(|&cell| cell == -1) {
            return -10;
        }
    }
    for col in 0..3 {
        if (0..3).all(|row| board[row][col] == 1) {
            return 10;
        } else if (0..3).all(|row| board[row][col] == -1) {
            return -10;
        }
    }
    if (0..3).all(|i| board[i][i] == 1) {
        return 10;

    } else if (0..3).all(|i| board[i][i] == -1) {
        return -10;

    } else if (0..3).all(|i| board[i][2 - i] == 1) {
        return 10;

    } else if (0..3).all(|i| board[i][2 - i] == -1) {
        return -10;

    }
    0
}

fn minimax(board: &mut [[i32; 3]; 3], player: i32) -> i32 {
    let score = evaluate(*board);
    if score != 0 {
        return score;
    }
    let mut moves = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 {
                moves.push((i, j));
            }
        }
    }
    if moves.is_empty() {
        return 0;
    }
    let mut best = if player == 1 { -1000 } else { 1000 };
    for (i, j) in moves {
        board[i][j] = player;
        let value = minimax(board, -player);
        board[i][j] = 0;
        if player == 1 {
            best = best.max(value);
        } else {
            best = best.min(value);
        }
    }
    best
}
#[tauri::command]
fn best_move(mut board: [[i32; 3]; 3]) -> (i32, i32) {
    let mut best = -1000;
    let mut move_pos = (0, 0);
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 {
                board[i][j] = 1;
                let value = minimax(&mut board, -1);
                board[i][j] = 0;
                if value > best {
                    best = value;
                    move_pos = (i as i32, j as i32);
                }
            }
        }
    }
    move_pos
}

#[tauri::command]
fn is_game_over(board: [[i32; 3]; 3]) -> bool {
    for row in board {
        if row.iter().all(|&cell| cell == 1) || row.iter().all(|&cell| cell == -1) {
            return true;
        }
    }
    for col in 0..3 {
        if (0..3).all(|row| board[row][col] == 1) || (0..3).all(|row| board[row][col] == -1) {
            return true;
        }
    }
    if (0..3).all(|i| board[i][i] == 1) || (0..3).all(|i| board[i][i] == -1) {
        return true;
    }
    if (0..3).all(|i| board[i][2 - i] == 1) || (0..3).all(|i| board[i][2 - i] == -1) {
        return true;
    }
    for row in board {
        for cell in row {
            if cell == 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_game_over, evaluate, best_move, ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
