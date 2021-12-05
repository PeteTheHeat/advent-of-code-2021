use std::fs;

fn parse_input() -> String {
    fs::read_to_string("./../input.txt").unwrap()
}

fn main() {
    println!("Part 1: {}", solve_part_1(parse_input()));
    println!("Part 2: {}", solve_part_2(parse_input()));
}

fn solve_part_1(contents: String) -> i32 {
    let (calls, mut boards) = parse_calls_boards(contents);
    for call in calls {
        for board in &mut boards {
            call_number_on_board(call, board);
        }
        for board in &boards {
            if check_bingo(board) {
                let board_sum = sum_board(board);
                println!(
                    "Bingo!! Sum:{} Call:{} Board:{:?}",
                    board_sum.to_string(),
                    call.to_string(),
                    board
                );
                return board_sum * call;
            }
        }
    }

    0
}

fn solve_part_2(contents: String) -> i32 {
    let (calls, mut boards) = parse_calls_boards(contents);
    for call in calls {
        for board in &mut boards {
            call_number_on_board(call, board);
        }
        if boards.len() > 1 {
            boards.retain(|board| !check_bingo(board));
        }
        if boards.len() == 1 && check_bingo(&boards[0]) {
            let board_sum = sum_board(&boards[0]);
            println!(
                "Found last one!! Sum:{} Call:{} Board:{:?}",
                board_sum.to_string(),
                call.to_string(),
                &boards[0]
            );
            return board_sum * call;
        }
    }
    0
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<i32>>,
}

fn parse_calls_boards(input: String) -> (Vec<i32>, Vec<Board>) {
    let mut lines = input.lines().peekable();
    let calls: Vec<i32> = lines
        .next()
        .unwrap()
        .to_string()
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    lines.next();
    let mut boards: Vec<Board> = Vec::new();

    loop {
        if !lines.peek().is_some() {
            return (calls, boards);
        }
        let mut rows = Vec::new();
        for _i in 0..5 {
            let str = lines.next().unwrap();
            let row: Vec<i32> = str
                .to_string()
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect();
            rows.push(row);
        }
        lines.next();
        boards.push(Board { rows });
    }
    (calls, boards)
}

fn call_number_on_board(call: i32, board: &mut Board) {
    // println!("Calling: {}", call.to_string());
    for i in 0..5 {
        for j in 0..5 {
            if board.rows[i][j] == call {
                board.rows[i][j] = 0;
            }
        }
    }
}

fn check_bingo(board: &Board) -> bool {
    // println!("Checking board: {:?}",board);
    // check rows
    for i in 0..5 {
        let sum: i32 = board.rows[i].iter().sum();
        if sum == 0 {
            return true;
        }
    }
    //check cols
    for i in 0..5 {
        let mut sum = 0;
        for j in 0..5 {
            sum += board.rows[j][i];
        }
        if sum == 0 {
            return true;
        }
    }
    false
}

fn sum_board(board: &Board) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..5 {
        sum += board.rows[i].iter().sum::<i32>();
    }
    sum
}
