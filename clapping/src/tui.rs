use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;

//stdout과 Stdout은 조금 다르니 rust-analyzer를 설치해서 한번쯤 차이를 느껴보는게 좋다.
//pub fn init()은 Java의 public void init()과 비슷하다, pub가 없으면 private이다.
//rust는 객체지향 언어가 아니니까 그냥 비슷하구나 정도로만 생각하고 공부하는 것이 좋다.
//no instance, flutter와 비슷한 지점은 StatefulWidget에도 dispose가 필요한데 그 역할을 restore가 해줘야 된다.
//tui는 터미널의 시작과 끝을 생각해야 된다. rawmode on/off가 필요함.
//rust는 클래스 대신 모듈과 구조체로 표현한다.
//Terminal<...> 부터 반환타입 시작임.
//flutter에서 BuildContext를 std::io::Stdout이 담당한 것과 비슷하다.
//unwrap()은 에러처리다.
//rust는 실패할 수 있는 함수가 Result를 반환하고 unwrap은 성공하면 값을 반환하고(진행 되고)
//실패하면 패닉으로 프로그램 종료하게 한다. no throw Exception.
//try/catch로 쓸 때에는 unwrap() 없이 바로 함수에 ?를 붙여서 쓸 수 있다.
pub fn init() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    Terminal::new(CrosstermBackend::new(stdout())).unwrap()
}

pub fn restore() {
    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
}
