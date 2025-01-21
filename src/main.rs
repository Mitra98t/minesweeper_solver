// Minesweeper game

use colored::Colorize;
use std::{thread, time::Duration};

const DIM_X: usize = 75;
const DIM_Y: usize = 28;
const NUM_MINES: usize = 300;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Mine,
    Number(u8),
    Empty,
    Unknown,
    Selected,
    Flagged,
}

fn main() {
    let mut sol = Solver { is_stuck: false };
    let rand_x = rand::random::<usize>() % DIM_X;
    let rand_y = rand::random::<usize>() % DIM_Y;

    let real_board = create_board(rand_x, rand_y);

    let mut player_board = [[Cell::Unknown; DIM_X]; DIM_Y];

    println!("x:{}, y:{}", rand_x, rand_y);
    let mut to_print = real_board.clone();
    to_print[rand_y][rand_x] = Cell::Selected;
    println!();

    print!("\x1B[2J\x1B[1;1H");
    player_board = sol.fist_step(real_board, player_board, rand_x, rand_y);
    print_board(player_board);
    println!(
        "Unknowns: {} | Highest: {}",
        count_unknowns(player_board),
        find_highest_number(player_board)
    );
    loop {
        //wait for input
        if sol.is_stuck {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
        print!("\x1B[2J\x1B[1;1H");

        let prev_board = player_board.clone();
        player_board = sol.solve_step(real_board, player_board);
        print_board(player_board);
        println!(
            "Unknowns: {} | Highest: {}",
            count_unknowns(player_board),
            find_highest_number(player_board)
        );
        sol.is_stuck = false;

        if prev_board == player_board {
            sol.is_stuck = true;
            println!("Stuck, press enter to {}", "random guess".yellow());
        }

        if check_lost(real_board, player_board) {
            println!("Boom!");
            break;
        }

        if count_unknowns(player_board) == NUM_MINES {
            println!("Solved!");
            break;
        }

        thread::sleep(Duration::from_millis(500));
    }
}

fn check_lost(real_board: [[Cell; DIM_X]; DIM_Y], player_board: [[Cell; DIM_X]; DIM_Y]) -> bool {
    for y in 0..DIM_Y {
        for x in 0..DIM_X {
            if player_board[y][x] != Cell::Unknown
                && player_board[y][x] != Cell::Flagged
                && real_board[y][x] == Cell::Mine
            {
                return true;
            }
        }
    }

    false
}

fn find_highest_number(board: [[Cell; DIM_X]; DIM_Y]) -> u8 {
    let mut highest_number = 0;

    for y in 0..DIM_Y {
        for x in 0..DIM_X {
            match board[y][x] {
                Cell::Number(n) => {
                    if n > highest_number {
                        highest_number = n;
                    }
                }
                _ => {}
            }
        }
    }

    highest_number
}

fn count_unknowns(board: [[Cell; DIM_X]; DIM_Y]) -> usize {
    let mut count = 0;

    for row in board.iter() {
        for cell in row.iter() {
            if *cell == Cell::Unknown || *cell == Cell::Flagged {
                count += 1;
            }
        }
    }

    count
}

#[derive(Clone, Copy, PartialEq)]
struct Solver {
    is_stuck: bool,
}

impl Solver {
    fn fist_step(
        self,
        real_board: [[Cell; DIM_X]; DIM_Y],
        player_board: [[Cell; DIM_X]; DIM_Y],
        x: usize,
        y: usize,
    ) -> [[Cell; DIM_X]; DIM_Y] {
        let mut new_board = player_board.clone();

        let mut visited = [[false; DIM_X]; DIM_Y];
        do_move(&real_board, &mut new_board, &mut visited, x, y);

        new_board
    }

    fn random_step(
        mut self,
        real_board: [[Cell; DIM_X]; DIM_Y],
        player_board: [[Cell; DIM_X]; DIM_Y],
    ) -> [[Cell; DIM_X]; DIM_Y] {
        let mut rand_x;
        let mut rand_y;
        loop {
            rand_x = rand::random::<usize>() % DIM_X;
            rand_y = rand::random::<usize>() % DIM_Y;
            if player_board[rand_y][rand_x] == Cell::Unknown {
                break;
            }
        }
        let mut new_board = player_board.clone();
        do_move(
            &real_board,
            &mut new_board,
            &mut [[false; DIM_X]; DIM_Y],
            rand_x,
            rand_y,
        );
        self.is_stuck = false;
        new_board
    }

    fn solve_step(
        mut self,
        real_board: [[Cell; DIM_X]; DIM_Y],
        player_board: [[Cell; DIM_X]; DIM_Y],
    ) -> [[Cell; DIM_X]; DIM_Y] {
        if self.is_stuck {
            self.is_stuck = false;
            return self.random_step(real_board, player_board);
        }
        let mut new_board = player_board.clone();

        let mut interest_points = Vec::new();
        for y in 0..DIM_Y {
            for x in 0..DIM_X {
                match new_board[y][x] {
                    Cell::Number(n) => {
                        let neighbours = get_neighbour_coordinates(x, y);
                        let mut count_unknowns: u8 = 0;
                        for (neighbour_x, neighbour_y) in neighbours.clone() {
                            if new_board[neighbour_y][neighbour_x] == Cell::Unknown
                                || new_board[neighbour_y][neighbour_x] == Cell::Flagged
                            {
                                count_unknowns += 1;
                            }
                        }
                        let board_snapshot = new_board.clone();
                        if count_unknowns == n {
                            for (neighbour_x, neighbour_y) in neighbours {
                                if board_snapshot[neighbour_y][neighbour_x] == Cell::Unknown
                                    || board_snapshot[neighbour_y][neighbour_x] == Cell::Flagged
                                {
                                    interest_points.push((neighbour_x, neighbour_y));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        //print informational_board
        // for row in informational_board.iter() {
        //     for cell in row.iter() {
        //         print!("{} ", cell);
        //     }
        //     println!();
        // }

        // find the cell with the lowest number of mines

        for (x, y) in interest_points {
            new_board[y][x] = Cell::Flagged;
        }

        let mut interest_points = Vec::new();
        for y in 0..DIM_Y {
            for x in 0..DIM_X {
                match new_board[y][x] {
                    Cell::Number(n) => {
                        let neighbours = get_neighbour_coordinates(x, y);
                        let mut count_unknowns: u8 = 0;
                        let mut count_flags: u8 = 0;
                        for (neighbour_x, neighbour_y) in neighbours.clone() {
                            if new_board[neighbour_y][neighbour_x] == Cell::Unknown {
                                count_unknowns += 1;
                            }
                            if new_board[neighbour_y][neighbour_x] == Cell::Flagged {
                                count_flags += 1;
                            }
                        }
                        let board_snapshot = new_board.clone();
                        if count_unknowns > 0 && count_flags == n {
                            for (neighbour_x, neighbour_y) in neighbours {
                                if board_snapshot[neighbour_y][neighbour_x] == Cell::Unknown {
                                    interest_points.push((neighbour_x, neighbour_y));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        for (x, y) in interest_points {
            do_move(
                &real_board,
                &mut new_board,
                &mut [[false; DIM_X]; DIM_Y],
                x,
                y,
            );
        }

        new_board
    }
}

fn get_neighbour_coordinates(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = x as i32 + i;
            let new_y = y as i32 + j;

            if new_x >= 0 && new_x < DIM_X as i32 && new_y >= 0 && new_y < DIM_Y as i32 {
                neighbours.push((new_x as usize, new_y as usize));
            }
        }
    }

    neighbours
}

fn create_board(player_x: usize, player_y: usize) -> [[Cell; DIM_X]; DIM_Y] {
    let mut board = [[Cell::Empty; DIM_X]; DIM_Y];

    for _ in 0..NUM_MINES {
        let mut rand_x;
        let mut rand_y;
        loop {
            rand_x = rand::random::<usize>() % DIM_X;
            rand_y = rand::random::<usize>() % DIM_Y;
            if rand_x != player_x && rand_y != player_y && board[rand_y][rand_x] != Cell::Mine {
                break;
            }
        }

        board[rand_y][rand_x] = Cell::Mine;
    }

    for y in 0..DIM_Y {
        for x in 0..DIM_X {
            if board[y][x] == Cell::Mine {
                continue;
            }

            let count = count_neighbours(board, x, y);

            if count > 0 {
                board[y][x] = Cell::Number(count);
            }
        }
    }

    board
}

fn count_neighbours(board: [[Cell; DIM_X]; DIM_Y], x: usize, y: usize) -> u8 {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = x as i32 + i;
            let new_y = y as i32 + j;

            if new_x >= 0 && new_x < DIM_X as i32 && new_y >= 0 && new_y < DIM_Y as i32 {
                if board[new_y as usize][new_x as usize] == Cell::Mine {
                    count += 1;
                }
            }
        }
    }

    count
}

fn print_board(board: [[Cell; DIM_X]; DIM_Y]) {
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Mine => print!("❤ "),
                Cell::Number(n) => {
                    if *n == 1 {
                        print!("{} ", "1".blue())
                    }
                    if *n == 2 {
                        print!("{} ", "2".green())
                    }
                    if *n == 3 {
                        print!("{} ", "3".red())
                    }
                    if *n == 4 {
                        print!("{} ", "4".yellow())
                    }
                    if *n == 5 {
                        print!("{} ", "5".cyan())
                    }
                    if *n == 6 {
                        print!("{} ", "6".magenta())
                    }
                    if *n == 7 {
                        print!("{} ", "7".purple().bold())
                    }
                    if *n == 8 {
                        print!("{} ", "8".red().bold())
                    }
                }
                Cell::Empty => print!("  "),
                Cell::Unknown => print!("⬛"),
                Cell::Flagged => print!("🚩"),
                Cell::Selected => print!("🔴"),
            }
        }
        println!();
    }
}

fn do_move(
    real_board: &[[Cell; DIM_X]; DIM_Y],
    player_board: &mut [[Cell; DIM_X]; DIM_Y],
    visited: &mut [[bool; DIM_X]; DIM_Y],
    x: usize,
    y: usize,
) {
    if x >= DIM_X || y >= DIM_Y || visited[y][x] {
        return;
    }

    visited[y][x] = true;

    match real_board[y][x] {
        Cell::Number(n) => {
            player_board[y][x] = Cell::Number(n);
        }
        Cell::Empty => {
            player_board[y][x] = Cell::Empty;

            if x > 0 {
                do_move(real_board, player_board, visited, x - 1, y);
            }
            if x < DIM_X - 1 {
                do_move(real_board, player_board, visited, x + 1, y);
            }
            if y > 0 {
                do_move(real_board, player_board, visited, x, y - 1);
            }
            if y < DIM_Y - 1 {
                do_move(real_board, player_board, visited, x, y + 1);
            }
            if x > 0 && y > 0 {
                do_move(real_board, player_board, visited, x - 1, y - 1);
            }
            if x < DIM_X - 1 && y > 0 {
                do_move(real_board, player_board, visited, x + 1, y - 1);
            }
            if x > 0 && y < DIM_Y - 1 {
                do_move(real_board, player_board, visited, x - 1, y + 1);
            }
            if x < DIM_X - 1 && y < DIM_Y - 1 {
                do_move(real_board, player_board, visited, x + 1, y + 1);
            }
        }
        Cell::Mine => {
            player_board[y][x] = Cell::Mine;
        }
        _ => {}
    }
}
