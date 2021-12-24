use std::fs;

fn parse_input() -> String {
    fs::read_to_string("./../input.txt").unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<i32>>,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

fn main() {
    println!("Part 1: {}", solve_part_1(parse_input()));
    println!("Part 2: {}", solve_part_2(parse_input()));
}

fn solve_part_1(contents: String) -> i32 {
    let lines = parse_into_vec(contents);
    let mut board: Board = Board { rows: Vec::new() };
    for _i in 0..999 {
        let mut row: Vec<i32> = Vec::new();
        for _j in 0..999 {
            row.push(0);
        }
        board.rows.push(row);
    }

    for line in lines {
        if line.is_vertical() || line.is_horizontal() {
            draw_straight_line_on_board(line, &mut board);
        }
    }

    let mut soln = 0;
    for i in 0..999 {
        for j in 0..999 {
            if board.rows[i][j] > 1 {
                soln += 1;
            }
        }
    }
    soln
}

fn solve_part_2(contents: String) -> i32 {
    let lines = parse_into_vec(contents);
    let mut board: Board = Board { rows: Vec::new() };
    for _i in 0..999 {
        let mut row: Vec<i32> = Vec::new();
        for _j in 0..999 {
            row.push(0);
        }
        board.rows.push(row);
    }

    for line in lines {
        if line.is_vertical() || line.is_horizontal() {
            draw_straight_line_on_board(line, &mut board);
        } else {
            draw_diagonal_line_on_board(line, &mut board);
        }
    }

    let mut soln = 0;
    for i in 0..999 {
        for j in 0..999 {
            if board.rows[i][j] > 1 {
                soln += 1;
            }
        }
    }
    soln
}

fn draw_straight_line_on_board(line: Line, board: &mut Board) {
    if line.is_vertical() {
        let (start, end);
        if line.start.y > line.end.y {
            start = line.end.y;
            end = line.start.y;
        } else {
            end = line.end.y;
            start = line.start.y;
        }
        for i in start..end + 1 {
            let x = line.start.x as usize;
            let y = i as usize;
            board.rows[x][y] = board.rows[x][y] + 1;
        }
    }
    if line.is_horizontal() {
        let (start, end);
        if line.start.x > line.end.x {
            start = line.end.x;
            end = line.start.x;
        } else {
            end = line.end.x;
            start = line.start.x;
        }
        for i in start..end + 1 {
            let x = i as usize;
            let y = line.start.y as usize;
            board.rows[x][y] = board.rows[x][y] + 1;
        }
    }
}

fn draw_diagonal_line_on_board(line: Line, board: &mut Board) {
    // Make line point right, could still be up or down though.
    let (start, end);
    if line.start.x > line.end.x {
        end = line.start;
        start = line.end;
    } else {
        start = line.start;
        end = line.end;
    }
    for i in 0..end.x - start.x + 1 {
        let x = (start.x + i) as usize;
        let y;
        if end.y > start.y {
            // points down
            y = (start.y + i) as usize;
        } else {
            // points up
            y = (start.y - i) as usize;
        }
        board.rows[x][y] = board.rows[x][y] + 1;
    }
}

fn parse_into_vec(contents: String) -> Vec<Line> {
    let mut output: Vec<Line> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split("->").map(|i| i.trim()).collect();
        let start: Vec<i32> = parts[0]
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        let end: Vec<i32> = parts[1]
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        output.push(Line {
            start: Point {
                x: start[0],
                y: start[1],
            },
            end: Point {
                x: end[0],
                y: end[1],
            },
        });
    }
    output
}
