turbo::cfg! {r#"
    name = "Tic Tac Turbo"
    version = "1.0.0"
    author = "Monty"
    description = "Game of tic tac toe"
    [settings]
    resolution = [300, 300]
"#}

turbo::init! {
    struct GameState {
        player_move: enum Move {
            PlayerX,
            Empty,
            PlayerO,
        },
        board: Vec<Vec<Move>>,
        grid_size: i32,
        size_offset: i32,
        cursor_x: i32,
        cursor_y: i32,
        total_moves: i32,
        winner: Move,
        tied: bool,
    } = {
        Self {
            player_move: Move::Empty,
            board: vec![vec![Move::Empty; 6]; 6],
            grid_size: 50,
            size_offset: 10,
            cursor_x: 0,
            cursor_y: 0,
            total_moves: 0,
            winner: Move::Empty,
            tied: false,
        }
    }
}

turbo::go! ({
    // load game state
    let mut state = GameState::load();

    // Get game size
    let size = state.board.len() as i32;

    if state.winner != Move::Empty || state.tied {
        // game is finished
        clear(0x000000ff);
        let [canvas_width, canvas_height] = canvas_size!();

        let mut message = "";
        if state.tied {
            message = "Nobody wins!"
        }
        if state.winner == Move::PlayerX {
            message = "Player X wins!"
        }
        if state.winner == Move::PlayerO {
            message = "Player O wins!"
        }

        text!(message,
              x = canvas_width / 2 - (message.len() as u32 * 2),
              y = canvas_height / 2,
              );
    } else {
        // game loop
        let m = mouse(0);
        if m.left.just_released() {
            // find grid position from mouse click
            let [mx, my] = m.position;
            let row = mx / state.grid_size;
            let col = my / state.grid_size;

            let row_idx = row as usize;
            let col_idx = col as usize;

            if state.board[row_idx][col_idx] == Move::Empty {
                state.board[row_idx][col_idx] = current_player(state.total_moves);
                state.total_moves+=1;
            }

        }

        let winner = find_winner(state.board.clone());
        if winner != Move::Empty {
            state.winner = winner;
        }

        if state.total_moves == size*size {
            state.tied = true;
		}

        // Set the background color
        clear(0xffffffff);
        for i in 1..size {
            path!(
                start = (0, i * state.grid_size),
                end = (size * state.grid_size, i * state.grid_size),
                width = 2,
                color = 0x000000ff,
                );

            path!(
                start = (i * state.grid_size, 0),
                end = (i * state.grid_size, size * state.grid_size),
                width = 2,
                color = 0x000000ff,
                );
        }

        // draw the moves
        let draw_offset = state.size_offset / 2;
        for (i, row) in state.board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == Move::PlayerX {
                    path!(
                        start = (i as i32 * state.grid_size + draw_offset, j as i32 * state.grid_size + draw_offset),
                        end = (i as i32 * state.grid_size + state.grid_size - draw_offset, j as i32 * state.grid_size + state.grid_size - draw_offset),
                        width = 4,
                        color = 0x0088FFFF,
                        );
                    path!(
                        start = (i as i32 * state.grid_size + state.grid_size - draw_offset, j as i32 * state.grid_size + draw_offset),
                        end = (i as i32 * state.grid_size + draw_offset, j as i32 * state.grid_size + state.grid_size - draw_offset),
                        width = 4,
                        color = 0x0088FFFF,
                        );
                } else if *col == Move::PlayerO {
                    circ!(
                        d = state.grid_size - state.size_offset,
                        x = state.grid_size * (i as i32) + draw_offset,
                        y = state.grid_size * (j as i32) + draw_offset,
                        border_width = 4,
                        border_color = 0xFF2200FF,
                        );
                }
            }
        }
    }

    state.save();
});

fn current_player(count: i32) -> Move {
    if count%2 == 0 {
        Move::PlayerX
    } else {
        Move::PlayerO
    }

}

fn move_to_point(m: Move) -> i32 {
    match m {
        Move::PlayerX => -1,
        Move::PlayerO => 1,
        _ => 0,
    }
}

fn find_winner_from_score(size: usize, score: i32) -> Move {
    let player_o_winning_score = size as i32;
    let player_x_winning_score = player_o_winning_score * -1;

    if score == player_x_winning_score {
        Move::PlayerX
    } else if score == player_o_winning_score {
        Move::PlayerO
    } else {
        Move::Empty
    }
}

fn find_winner(board: Vec<Vec<Move>>) -> Move {
    let size = board.len();

    // row scoring
    for (_, row) in board.iter().enumerate() {
        let mut row_score = 0;
        for (_, col) in row.iter().enumerate() {
            row_score += move_to_point(col.clone());
        }

        let winner = find_winner_from_score(size, row_score);
        if winner != Move::Empty {
            return winner
        }
    }

    // col scoring
    for i in 0..size {
        let mut col_score = 0;
        for (_, row) in board.iter().enumerate() {
            col_score += move_to_point(row[i].clone());
        }

        let winner = find_winner_from_score(size, col_score);
        if winner != Move::Empty {
            return winner
        }
    }

    // diagonal scoring
    let mut diag_left_score = 0;
    let mut diag_right_score = 0;
    for (i, row) in board.iter().enumerate() {
        diag_left_score += move_to_point(row[i].clone());
        diag_right_score += move_to_point(row[size - i - 1].clone());
    }

    let diag_left_winner = find_winner_from_score(size, diag_left_score);
    if diag_left_winner != Move::Empty {
        return diag_left_winner
    }

    let diag_right_winner = find_winner_from_score(size, diag_right_score);
    if diag_right_winner != Move::Empty {
        return diag_right_winner
    }

    Move::Empty
}
