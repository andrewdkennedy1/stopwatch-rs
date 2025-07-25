use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

struct App {
    start_time: Instant,
    is_running: bool,
    laps: Vec<(Duration, Duration)>, // (lap_time, total_time)
    last_lap: Instant,
    laps_list_state: ListState,
}

impl App {
    fn new() -> App {
        let now = Instant::now();
        App {
            start_time: now,
            is_running: true,
            laps: Vec::new(),
            last_lap: now,
            laps_list_state: ListState::default(),
        }
    }

    fn add_lap(&mut self) {
        let now = Instant::now();
        let lap_time = now.duration_since(self.last_lap);
        let total_time = now.duration_since(self.start_time);
        self.laps.push((lap_time, total_time));
        self.last_lap = now;

        // Auto-select the newest lap (at index 0 after reversing)
        if !self.laps.is_empty() {
            self.laps_list_state.select(Some(0));
        }
    }

    fn toggle_pause(&mut self) {
        self.is_running = !self.is_running;
        if self.is_running {
            let now = Instant::now();
            let paused_duration = now.duration_since(self.last_lap);
            self.start_time += paused_duration;
            self.last_lap = now;
        }
    }

    fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_lap = now;
        self.is_running = true;
        self.laps.clear();
        self.laps_list_state.select(None);
    }

    fn elapsed(&self) -> Duration {
        if self.is_running {
            Instant::now().duration_since(self.start_time)
        } else {
            self.last_lap.duration_since(self.start_time)
        }
    }

    fn scroll_up(&mut self) {
        if self.laps.is_empty() {
            return;
        }

        let selected = match self.laps_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.laps.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.laps_list_state.select(Some(selected));
    }

    fn scroll_down(&mut self) {
        if self.laps.is_empty() {
            return;
        }

        let selected = match self.laps_list_state.selected() {
            Some(i) => {
                if i >= self.laps.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.laps_list_state.select(Some(selected));
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs_f64();
    if total_seconds < 60.0 {
        format!("{:.2}s", total_seconds)
    } else if total_seconds < 3600.0 {
        let minutes = (total_seconds / 60.0).floor();
        let seconds = total_seconds % 60.0;
        format!("{:.0}m {:.2}s", minutes, seconds)
    } else {
        let hours = (total_seconds / 3600.0).floor();
        let remaining_seconds = total_seconds % 3600.0;
        let minutes = (remaining_seconds / 60.0).floor();
        let seconds = remaining_seconds % 60.0;
        format!("{:.0}h {:.0}m {:.1}s", hours, minutes, seconds)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events, not key release or repeat
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => app.add_lap(),
                        KeyCode::Char('p') => app.toggle_pause(),
                        KeyCode::Char('r') => app.reset(),
                        KeyCode::Up => app.scroll_up(),
                        KeyCode::Down => app.scroll_down(),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(5), // Main timer
            Constraint::Length(3), // Status/controls
            Constraint::Min(0),    // Laps list
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("⏱️  STOPWATCH")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main timer display
    let elapsed = app.elapsed();
    let elapsed_str = format_duration(elapsed);

    let timer_color = if elapsed.as_secs() < 10 {
        Color::Green
    } else if elapsed.as_secs() < 60 {
        Color::Yellow
    } else if elapsed.as_secs() < 300 {
        Color::Cyan
    } else {
        Color::Magenta
    };

    let status_indicator = if app.is_running { "⏸" } else { "▶" };

    let timer_text = vec![Line::from(vec![
        Span::styled(status_indicator, Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled(
            elapsed_str,
            Style::default()
                .fg(timer_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let timer = Paragraph::new(timer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Elapsed Time"));
    f.render_widget(timer, chunks[1]);

    // Controls
    let controls = if app.is_running {
        "SPACE: Lap  •  P: Pause  •  R: Reset  •  ↑↓: Scroll  •  Q: Quit"
    } else {
        "P: Resume  •  R: Reset  •  ↑↓: Scroll  •  Q: Quit"
    };

    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls_widget, chunks[2]);

    // Laps list
    if !app.laps.is_empty() {
        let laps: Vec<ListItem> = app
            .laps
            .iter()
            .enumerate()
            .rev()
            .map(|(i, (lap_time, total_time))| {
                let lap_num = i + 1; // Correct lap numbering: first lap = 1, second = 2, etc.
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("Lap {:2}: ", lap_num),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format_duration(*lap_time),
                        Style::default().fg(Color::White),
                    ),
                    Span::raw("  (Total: "),
                    Span::styled(
                        format_duration(*total_time),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::raw(")"),
                ]))
            })
            .collect();

        let laps_widget = List::new(laps)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Laps ({}) - Use ↑↓ to scroll", app.laps.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(laps_widget, chunks[3], &mut app.laps_list_state);
    } else {
        let no_laps = Paragraph::new("Press SPACE to record your first lap!")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Laps"));
        f.render_widget(no_laps, chunks[3]);
    }
}
