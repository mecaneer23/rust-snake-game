extern crate ncurses;

use ncurses::*;
use rand::prelude::*;
use std::env::args;

fn snake() -> String {
    // Initialize ncurses
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    use_default_colors();
    noecho();
    nodelay(stdscr(), true);
    timeout(100);
    keypad(stdscr(), true);
    start_color();
    if args().count() > 1 && args().nth(1).unwrap() == "--color" {
        init_pair(1, COLOR_WHITE, -1);
        init_pair(2, COLOR_GREEN, -1);
        init_pair(3, COLOR_RED, -1);
    } else {
        init_pair(1, -1, -1);
        init_pair(2, -1, -1);
        init_pair(3, -1, -1);
    }

    // Set global variables
    let ROWS: i32 = getmaxy(stdscr()) - 1;
    let COLS: i32 = getmaxx(stdscr()) - 1;
    const CHAR_SNAKE: &str = "#";
    const CHAR_FOOD: &str = "*";
    const CHAR_BG: &str = ".";

    // Initialize game variables
    let mut snake: Vec<(i32, i32)> = vec![(5, 5), (5, 4), (5, 3)];
    let mut food: (i32, i32) = (ROWS / 2, COLS / 2);
    let mut direction: i32 = 100;
    let paused: bool = false;
    let mut score: i32 = 0;
    let mut next_direction: i32;
    let mut new_head: (i32, i32);
    let mut new_food: (i32, i32);
    let mut headless_snake: Vec<(i32, i32)>;
    let mut tail: (i32, i32);
    let mut rng = rand::thread_rng();

    // draw board
    color_set(1);
    for y in 0..ROWS {
        for x in 0..COLS {
            mvaddstr(y, x, CHAR_BG);
        }
    }

    // draw snake
    color_set(2);
    for i in 0..snake.len() {
        mvaddstr(snake[i].0, snake[i].1, CHAR_SNAKE);
    }

    // draw food
    color_set(3);
    mvaddstr(food.0, food.1, CHAR_FOOD);

    color_set(1);
    mvaddstr(ROWS, 0, "Controls: wasd or arrow keys, q to quit | Score: 0");

    // main loop
    while true {
        next_direction = getch();
        direction = if next_direction == -1 { direction } else { next_direction };
        if snake[0].0 == ROWS || snake[0].0 == -1 {
            return "Snake out of bounds vertically, score: ".to_owned() + &score.to_string();
        }
        if snake[0].1 == COLS || snake[0].1 == -1 {
            return "Snake out of bounds horizontally, score: ".to_owned() + &score.to_string();
        }
        headless_snake = snake.clone();
        headless_snake.remove(0);
        if headless_snake.contains(&snake[0]) {
            return "Snake can't eat itself, score: ".to_owned() + &score.to_string();
        }
        new_head = snake[0].clone();
        if direction == 119 || direction == 259 {
                new_head.0 -= 1;
        } else if direction == 97 || direction == 260 {
                new_head.1 -= 1;
        } else if direction == 115 || direction == 258 {
                new_head.0 += 1;
        } else if direction == 100 || direction == 261 {
                new_head.1 += 1;
        } else if direction == 113 || direction == 27 {
                return "Quit, score: ".to_owned() + &score.to_string();
        } else {
            continue;
        }
        if !paused {
            snake.insert(0, new_head);
            if snake[0] == food {
                food = (0, 0);
                while food == (0, 0) {
                    new_food = (
                        rng.gen_range(0..ROWS),
                        rng.gen_range(0..COLS),
                    );
                    food = if !snake.contains(&new_food) { new_food } else { (0, 0) };
                }
                color_set(3);
                mvaddstr(food.0, food.1, CHAR_FOOD);
                score += 1;
            } else {
                tail = snake.pop().unwrap();
                color_set(1);
                mvaddstr(tail.0, tail.1, CHAR_BG);
            }
            color_set(2);
            mvaddstr(snake[0].0, snake[0].1, CHAR_SNAKE);
        }
        color_set(1);
        mvaddstr(ROWS, 49, &score.to_string());
        refresh();
    }
    return "Unreachable".to_owned();
}

fn main() {
    let ret = snake();
    endwin();
    println!("Game over: {}", ret);
}