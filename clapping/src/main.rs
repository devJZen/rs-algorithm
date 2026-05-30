mod action;
mod app;
mod cli;
mod components;
mod tui;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    let mut terminal = tui::init();
    // algorithm/size 없으면 인터랙티브 선택 화면으로 시작
    let mut app = app::App::new(cli.algorithm, cli.size);
    app.run(&mut terminal).expect("app error");
    tui::restore();
}
