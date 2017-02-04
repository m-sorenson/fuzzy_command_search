extern crate ncurses;

mod command_search;

use command_search::CommandSearch;

fn main() {
    ncurses::initscr();
    ncurses::refresh();
    let mut search = CommandSearch::new();

    clear_screen();
    command_prompt("");
    loop {
        let c = ncurses::getch() as u8;
        let matches = search.input(c);
        let command = search.command.clone();

        clear_screen();
        print_results(matches);
        command_prompt(&command);
    }
}

fn screen_height() -> usize {
    let mut max_x = 0;
    let mut max_y = 0;

    ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);

    max_y as usize
}

fn clear_screen() {
    let height = screen_height();

    for l in 0..(height) {
        ncurses::mv(l as i32, 0);
        ncurses::clrtoeol();
    }
}

fn print_results(commands: Vec<String>) {
    let height = screen_height();
    let bottom = height - 2;
    let mut down = 0;

    let seq = commands.into_iter()
        .take(bottom);

    for command in seq {
        ncurses::mv(down, 0);
        ncurses::printw(command.as_str());
        down += 1;
    }
}

fn command_prompt(command: & str) {
    let height = screen_height();

    ncurses::mv((height-1) as i32, 0);
    ncurses::printw("> ");
    ncurses::printw(command);
}
