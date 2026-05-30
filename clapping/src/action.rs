#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    NextStep,
    PrevStep,
    Reset,
    TogglePause,
}
