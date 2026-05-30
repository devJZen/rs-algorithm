use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    io::Stdout,
    time::{Duration, Instant},
};

use crate::cli::{Algorithm, Topic, ALL_TOPICS};
use crate::components::algorithms;

type Step = (Vec<i32>, usize, usize);

// ── 화면 상태 ───────────────────────────────────────────────────────────

enum Screen {
    Select {
        matches: Vec<usize>,
        selected: usize,
        filter: String,
        typing: bool,
        scroll: u16,          // 목록 뷰포트 오프셋
    },
    SizeInput {
        algorithm: Algorithm,
        input: String,
    },
    Visualize {
        algorithm: Algorithm,
        steps: Vec<Step>,
        current: usize,
        paused: bool,
        last_tick: Instant,
        explanation_scroll: u16,  // 설명 패널 스크롤
    },
}

// ── App ─────────────────────────────────────────────────────────────────

pub struct App {
    screen: Screen,
    quit: bool,
    tick_rate: Duration,
}

impl App {
    pub fn new(algorithm: Option<Algorithm>, size: Option<usize>) -> Self {
        let screen = match (algorithm, size) {
            (Some(alg), Some(n)) => {
                let steps = algorithms::generate_steps(&alg, &gen_data(n));
                Screen::Visualize {
                    algorithm: alg, steps, current: 0, paused: true,
                    last_tick: Instant::now(), explanation_scroll: 0,
                }
            }
            (Some(alg), None) => Screen::SizeInput { algorithm: alg, input: "10".into() },
            _ => select_screen(),
        };
        Self { screen, quit: false, tick_rate: Duration::from_millis(400) }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            let timeout = match &self.screen {
                Screen::Visualize { paused, last_tick, .. } if !paused => {
                    self.tick_rate.saturating_sub(last_tick.elapsed())
                }
                _ => Duration::from_millis(50),
            };

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }

            if let Screen::Visualize { paused, current, steps, last_tick, .. } = &mut self.screen {
                if !*paused && last_tick.elapsed() >= self.tick_rate {
                    if *current < steps.len().saturating_sub(1) { *current += 1; }
                    else { *paused = true; }
                    *last_tick = Instant::now();
                }
            }

            if self.quit { break; }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) {
        enum Action { None, Quit, ToSizeInput(Algorithm), ToSelect, ToVisualize(Algorithm, usize) }

        let action = match &mut self.screen {
            Screen::Select { matches, selected, filter, typing, scroll } => match code {
                KeyCode::Char('q') if !*typing => Action::Quit,
                KeyCode::Esc => {
                    if *typing {
                        *typing = false;
                        filter.clear();
                        *matches = all_indices();
                        *selected = 0;
                        *scroll = 0;
                        Action::None
                    } else {
                        Action::Quit
                    }
                }
                KeyCode::Char('/') if !*typing => { *typing = true; Action::None }
                KeyCode::Char(c) if *typing => {
                    filter.push(c);
                    rebuild_matches(matches, filter);
                    *selected = 0;
                    *scroll = 0;
                    Action::None
                }
                KeyCode::Backspace if *typing => {
                    filter.pop();
                    rebuild_matches(matches, filter);
                    *selected = 0;
                    *scroll = 0;
                    Action::None
                }
                KeyCode::Up => {
                    *selected = selected.saturating_sub(1);
                    sync_scroll(scroll, matches, *selected);
                    Action::None
                }
                KeyCode::Down => {
                    if *selected + 1 < matches.len() { *selected += 1; }
                    sync_scroll(scroll, matches, *selected);
                    Action::None
                }
                KeyCode::Enter => {
                    if let Some(&idx) = matches.get(*selected) {
                        let topic = ALL_TOPICS[idx];
                        if topic.is_implemented() {
                            if let Topic::Algo(alg) = topic { Action::ToSizeInput(alg) }
                            else { Action::None }
                        } else { Action::None }
                    } else { Action::None }
                }
                _ => Action::None,
            },

            Screen::SizeInput { algorithm, input } => match code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Esc => Action::ToSelect,
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if input.len() < 3 { input.push(c); }
                    Action::None
                }
                KeyCode::Backspace => { input.pop(); Action::None }
                KeyCode::Enter => {
                    let n = input.parse::<usize>().unwrap_or(10).clamp(2, 30);
                    Action::ToVisualize(*algorithm, n)
                }
                _ => Action::None,
            },

            Screen::Visualize { paused, current, steps, explanation_scroll, .. } => match code {
                KeyCode::Char('q') | KeyCode::Esc => Action::ToSelect,
                KeyCode::Char(' ') => { *paused = !*paused; Action::None }
                KeyCode::Right => {
                    *paused = true;
                    if *current < steps.len().saturating_sub(1) { *current += 1; }
                    Action::None
                }
                KeyCode::Left => {
                    *paused = true;
                    *current = current.saturating_sub(1);
                    Action::None
                }
                // ↑↓ = 설명 패널 스크롤
                KeyCode::Up => {
                    *explanation_scroll = explanation_scroll.saturating_sub(1);
                    Action::None
                }
                KeyCode::Down => {
                    *explanation_scroll += 1;
                    Action::None
                }
                KeyCode::Char('r') => { *current = 0; *paused = true; Action::None }
                _ => Action::None,
            },
        };

        match action {
            Action::None => {}
            Action::Quit => self.quit = true,
            Action::ToSizeInput(alg) => {
                self.screen = Screen::SizeInput { algorithm: alg, input: "10".into() };
            }
            Action::ToSelect => { self.screen = select_screen(); }
            Action::ToVisualize(alg, n) => {
                let steps = algorithms::generate_steps(&alg, &gen_data(n));
                self.screen = Screen::Visualize {
                    algorithm: alg, steps, current: 0, paused: true,
                    last_tick: Instant::now(), explanation_scroll: 0,
                };
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        match &self.screen {
            Screen::Select { matches, selected, filter, typing, scroll } => {
                draw_select(f, matches, *selected, filter, *typing, *scroll);
            }
            Screen::SizeInput { algorithm, input } => {
                draw_size_input(f, algorithm, input);
            }
            Screen::Visualize { algorithm, steps, current, paused, explanation_scroll, .. } => {
                draw_visualize(f, algorithm, steps, *current, *paused, *explanation_scroll);
            }
        }
    }
}

// ── 헬퍼 ────────────────────────────────────────────────────────────────

fn gen_data(n: usize) -> Vec<i32> {
    (0..n as i32).map(|i| ((i * 7 + 3) % n as i32) + 1).collect()
}

fn all_indices() -> Vec<usize> { (0..ALL_TOPICS.len()).collect() }

fn select_screen() -> Screen {
    Screen::Select {
        matches: all_indices(), selected: 0,
        filter: String::new(), typing: false, scroll: 0,
    }
}

fn rebuild_matches(matches: &mut Vec<usize>, filter: &str) {
    let f = filter.to_lowercase();
    *matches = if f.is_empty() {
        all_indices()
    } else {
        ALL_TOPICS.iter().enumerate()
            .filter(|(_, t)| {
                t.keyword().contains(f.as_str()) ||
                t.name().to_lowercase().contains(f.as_str())
            })
            .map(|(i, _)| i)
            .collect()
    };
}

// 선택된 항목의 시각적 줄 번호 (카테고리 헤더 포함)
fn visual_line_of(matches: &[usize], selected: usize) -> u16 {
    let mut line = 0u16;
    let mut last_is_algo: Option<bool> = None;
    for (i, &idx) in matches.iter().enumerate() {
        let is_algo = ALL_TOPICS[idx].is_algo();
        if last_is_algo != Some(is_algo) {
            line += 1; // 카테고리 헤더 줄
            last_is_algo = Some(is_algo);
        }
        if i == selected { return line; }
        line += 1;
    }
    line
}

// 목록 패널 내부 높이 추정 (터미널 전체 높이 - 고정 영역)
fn list_inner_height() -> u16 {
    terminal::size()
        .map(|(_, rows)| rows.saturating_sub(15))
        .unwrap_or(10)
        .max(3)
}

// selected 항목이 화면에 보이도록 scroll 조정
fn sync_scroll(scroll: &mut u16, matches: &[usize], selected: usize) {
    let vis = visual_line_of(matches, selected);
    let h = list_inner_height();
    if vis < *scroll {
        *scroll = vis;
    } else if vis >= scroll.saturating_add(h) {
        *scroll = vis - h + 1;
    }
}

// ── 배너 (공통) ──────────────────────────────────────────────────────────

const BANNER: &str = "    /\\_____/\\\n   /  o   o  \\\n  ( ==  ^  == )\n   )         (\n  (           )\n ( (  )   (  ) )\n(__(__)___(__)__)";

fn banner_lines() -> Vec<Line<'static>> {
    let mut lines = vec![Line::raw("")];
    for (i, raw) in BANNER.lines().enumerate() {
        lines.push(match i {
            2 => Line::from(vec![
                Span::styled(raw, Style::default().fg(Color::White)),
                Span::styled("   algoviz", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            3 => Line::from(vec![
                Span::styled(raw, Style::default().fg(Color::White)),
                Span::styled("   algorithm visualizer", Style::default().fg(Color::DarkGray)),
            ]),
            _ => Line::styled(raw, Style::default().fg(Color::White)),
        });
    }
    lines
}

// ── 알고리즘 / 자료구조 선택 화면 ────────────────────────────────────────

fn draw_select(f: &mut Frame, matches: &[usize], selected: usize, filter: &str, typing: bool, scroll: u16) {
    let area = f.area();
    let [banner_area, input_area, list_area, hint_area] = Layout::vertical([
        Constraint::Length(9),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ]).areas(area);

    f.render_widget(Paragraph::new(Text::from(banner_lines())), banner_area);

    // 검색 입력
    let cursor: &str = if typing { "▌" } else { "" };
    let input_text = if typing || !filter.is_empty() {
        format!("  topic  /{}{}", filter, cursor)
    } else {
        format!("  topic  {}", cursor)
    };
    f.render_widget(
        Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(if typing { Color::Yellow } else { Color::White })),
        input_area,
    );

    // 항목 목록
    let mut list_lines: Vec<Line<'static>> = Vec::new();
    if matches.is_empty() {
        list_lines.push(Line::styled("   (검색 결과 없음)", Style::default().fg(Color::DarkGray)));
    } else {
        let mut last_is_algo: Option<bool> = None;
        for (i, &idx) in matches.iter().enumerate() {
            let topic = ALL_TOPICS[idx];
            let is_algo = topic.is_algo();

            if last_is_algo != Some(is_algo) {
                let header: &'static str = if is_algo {
                    "  ── Sorting Algorithms ──────────────────────────────────"
                } else {
                    "  ── Data Structures ─────────────────────────────────────"
                };
                list_lines.push(Line::styled(header, Style::default().fg(Color::DarkGray)));
                last_is_algo = Some(is_algo);
            }

            let is_sel = i == selected;
            let is_impl = topic.is_implemented();
            let (row_fg, row_bg, prefix): (Color, Color, &'static str) = match (is_impl, is_sel) {
                (true,  true)  => (Color::Black,    Color::Yellow,   " ▶ "),
                (false, true)  => (Color::Gray,     Color::DarkGray, " · "),
                (true,  false) => (Color::White,    Color::Reset,    "   "),
                (false, false) => (Color::DarkGray, Color::Reset,    "   "),
            };
            let row_style = Style::default().fg(row_fg).bg(row_bg);
            let dim_style = Style::default()
                .fg(if is_impl && is_sel { Color::Black } else { Color::DarkGray })
                .bg(row_bg);

            list_lines.push(Line::from(vec![
                Span::styled(prefix, row_style.add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:<14}", topic.keyword()), row_style.add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:<22}", topic.name()), row_style),
                Span::styled(topic.complexity(), dim_style),
                Span::styled("  ", row_style),
            ]));
        }
    }

    f.render_widget(
        Paragraph::new(Text::from(list_lines))
            .block(Block::default().borders(Borders::ALL).title(" 알고리즘 / 자료구조 선택 "))
            .scroll((scroll, 0)),   // ← 뷰포트 스크롤
        list_area,
    );

    f.render_widget(
        Paragraph::new(
            "  [↑↓] 이동   [/] 검색   [Enter] 선택   [Esc] 취소   [q] 종료   ※ 회색(·) = 준비중"
        ).style(Style::default().fg(Color::DarkGray)),
        hint_area,
    );
}

// ── 원소 개수 입력 화면 ──────────────────────────────────────────────────

fn draw_size_input(f: &mut Frame, algorithm: &Algorithm, input: &str) {
    let area = f.area();
    let [banner_area, algo_area, size_area, _, hint_area] = Layout::vertical([
        Constraint::Length(9),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ]).areas(area);

    f.render_widget(Paragraph::new(Text::from(banner_lines())), banner_area);

    f.render_widget(
        Paragraph::new(format!("  algorithm  {}  ✓", algorithm.keyword()))
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Green)),
        algo_area,
    );
    f.render_widget(
        Paragraph::new(format!("  size  {}▌  (2~30)", input))
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow)),
        size_area,
    );
    f.render_widget(
        Paragraph::new("  [Enter] 시작   [Esc] 뒤로   [q] 종료")
            .style(Style::default().fg(Color::DarkGray)),
        hint_area,
    );
}

// ── 시각화 화면 ──────────────────────────────────────────────────────────

fn draw_visualize(
    f: &mut Frame,
    algorithm: &Algorithm,
    steps: &[Step],
    current: usize,
    paused: bool,
    explanation_scroll: u16,
) {
    let area = f.area();
    let [header_area, body_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
    ]).areas(area);

    let [left_area, right_area] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ]).areas(body_area);

    // 헤더 (↑↓ 스크롤 안내 추가)
    let status = if paused { "■ paused" } else { "▶ playing" };
    let total = steps.len().saturating_sub(1);
    let header = format!(
        "  {}  │  step {}/{}  │  {}  │  [space] [←→] step  [↑↓] 설명 스크롤  [r] reset  [q/Esc] 선택으로",
        algorithm.name(), current, total, status,
    );
    f.render_widget(
        Paragraph::new(header)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan)),
        header_area,
    );

    // 왼쪽: 설명 (↑↓ 스크롤 가능)
    f.render_widget(
        Paragraph::new(algorithms::explanation(algorithm))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Explanation  [↑↓ scroll {}] ", explanation_scroll)),
            )
            .wrap(Wrap { trim: false })
            .scroll((explanation_scroll, 0)),
        left_area,
    );

    // 오른쪽: ASCII 아트 시각화
    f.render_widget(
        Paragraph::new(build_viz(steps, current))
            .block(Block::default().borders(Borders::ALL).title(" Visualization ")),
        right_area,
    );
}

fn build_viz(steps: &[Step], current: usize) -> Text<'static> {
    if steps.is_empty() { return Text::raw("  No data."); }

    let (state, ci, cj) = &steps[current];
    let ci = *ci;
    let cj = *cj;
    let n = state.len();
    let is_done = current == steps.len().saturating_sub(1);
    let show_markers = !is_done && ci < n;

    let mut lines: Vec<Line<'static>> = Vec::new();
    lines.push(Line::raw(""));

    let top: String = {
        let mut s = String::from("  ┌");
        for i in 0..n { s.push_str("──"); s.push(if i < n - 1 { '┬' } else { '┐' }); }
        s
    };
    lines.push(Line::raw(top));

    let mut val_spans: Vec<Span<'static>> = vec![Span::raw("  │")];
    for (idx, &val) in state.iter().enumerate() {
        let s = format!("{:>2}│", val);
        let style = if is_done {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else if show_markers && idx == ci {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if show_markers && idx == cj {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        val_spans.push(Span::styled(s, style));
    }
    lines.push(Line::from(val_spans));

    let bot: String = {
        let mut s = String::from("  └");
        for i in 0..n { s.push_str("──"); s.push(if i < n - 1 { '┴' } else { '┘' }); }
        s
    };
    lines.push(Line::raw(bot));

    if show_markers {
        let mut arrows: Vec<Span<'static>> = vec![Span::raw("   ")];
        for idx in 0..n {
            arrows.push(if idx == ci {
                Span::styled(" ↑ ", Style::default().fg(Color::Yellow))
            } else if idx == cj {
                Span::styled(" ↑ ", Style::default().fg(Color::Red))
            } else {
                Span::raw("   ")
            });
        }
        lines.push(Line::from(arrows));

        let mut labels: Vec<Span<'static>> = vec![Span::raw("   ")];
        for idx in 0..n {
            labels.push(if idx == ci {
                Span::styled(" i ", Style::default().fg(Color::Yellow))
            } else if idx == cj {
                Span::styled(" j ", Style::default().fg(Color::Red))
            } else {
                Span::raw("   ")
            });
        }
        lines.push(Line::from(labels));
    }

    lines.push(Line::raw(""));

    if is_done {
        lines.push(Line::styled(
            "  ✓  Sorted!",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ));
    } else if show_markers {
        let a = state[ci];
        let b = state[cj];
        lines.push(Line::from(vec![
            Span::raw("  comparing  "),
            Span::styled(format!("arr[{}]={}", ci, a), Style::default().fg(Color::Yellow)),
            Span::raw("  and  "),
            Span::styled(format!("arr[{}]={}", cj, b), Style::default().fg(Color::Red)),
        ]));
        lines.push(Line::raw(""));
        if a > b {
            lines.push(Line::styled(
                format!("  {} > {}  →  SWAP!", a, b),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ));
        } else {
            lines.push(Line::styled(
                format!("  {} ≤ {}  →  순서 유지", a, b),
                Style::default().fg(Color::Green),
            ));
        }
    } else {
        lines.push(Line::styled(
            "  [Space] 를 눌러 시작하세요",
            Style::default().fg(Color::DarkGray),
        ));
    }

    Text::from(lines)
}
