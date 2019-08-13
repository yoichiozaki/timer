extern crate ncurses;
extern crate eventual;

use ncurses::*;
use eventual::Timer;
use std::env;
use std::str::FromStr;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    run(&args)
}

fn run(args: &Vec<String>) {
    let mut cols = 0;
    let mut rows = 0;

    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    getmaxyx(stdscr(), &mut rows, &mut cols);

    let timer = Timer::new();
    let ticks = timer.interval_ms(1000).iter();

    centering_print(&rows, &cols, &"--- Start! ---");

    let seconds = i32::from_str(&args[1]).unwrap();
    let mut i = seconds;
    for _ in ticks {
        if i < 0 {
            break;
        }
        centering_print(&rows, &cols, &format!("{}s left", i));
        i -= 1;
    }
    centering_print(&rows, &cols, &format!("Finish!!!"));
    getch();
    endwin();
}

fn centering_print(rows: &i32, cols: &i32, message: &str) {
    clear();
    let pos_x :i32 = cols/2 - ((message.len() / 2) as i32);
    mvprintw(rows/2, pos_x, &message);
    refresh();
}

fn print_usage() {
    println!("Usage: timer [time in second]")
}