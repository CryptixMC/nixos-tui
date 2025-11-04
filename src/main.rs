use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Terminal,
};
use std::{error::Error, io, time::Duration};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Step {
    ProfileSelection,
    HostConfig,
    HostNamePrompt,
    Done,
}

impl Step {
    fn all() -> &'static [Step] {
        &[
            Step::ProfileSelection,
            Step::HostConfig,
            Step::HostNamePrompt,
            Step::Done,
        ]
    }

    fn title(&self) -> &'static str {
        match self {
            Step::ProfileSelection => "Profile Selection",
            Step::HostConfig => "Host Configuration",
            Step::HostNamePrompt => "Enter Host Name",
            Step::Done => "Done",
        }
    }
}

struct App {
    step_idx: usize,
    profile_idx: usize,
    host_idx: usize,
    host_is_new: bool,
    host_name: String,
    profiles: Vec<&'static str>,
    hosts: Vec<&'static str>,
}

impl App {
    fn new() -> Self {
        Self {
            step_idx: 0,
            profile_idx: 0,
            host_idx: 0,
            host_is_new: false,
            host_name: String::new(),
            profiles: vec!["desktop (nix-dots)", "server (nixos-server)"],
            hosts: vec!["carbon", "helium", "Create new host..."],
        }
    }

    fn next_step(&mut self) {
        if self.step_idx + 1 < Step::all().len() {
            self.step_idx += 1;
        }
    }

    fn prev_step(&mut self) {
        if self.step_idx > 0 {
            self.step_idx -= 1;
        }
    }

    fn current_step(&self) -> Step {
        Step::all()[self.step_idx]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            // Step title
            let title = Paragraph::new(app.current_step().title())
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            // Main content for each step
            match app.current_step() {
                Step::ProfileSelection => {
                    let items: Vec<ListItem> = app.profiles
                        .iter()
                        .enumerate()
                        .map(|(i, p)| {
                            if i == app.profile_idx {
                                ListItem::new(Span::styled(
                                    format!("> {}", p),
                                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::REVERSED),
                                ))
                            } else {
                                ListItem::new(Span::raw(format!("  {}", p)))
                            }
                        })
                        .collect();
                    f.render_widget(
                        List::new(items)
                            .block(Block::default().borders(Borders::ALL).title("Select Profile"))
                            .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
                        chunks[1],
                    );
                }
                Step::HostConfig => {
                    let items: Vec<ListItem> = app.hosts
                        .iter()
                        .enumerate()
                        .map(|(i, h)| {
                            if i == app.host_idx {
                                ListItem::new(Span::styled(
                                    format!("> {}", h),
                                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::REVERSED),
                                ))
                            } else {
                                ListItem::new(Span::raw(format!("  {}", h)))
                            }
                        })
                        .collect();
                    f.render_widget(
                        List::new(items)
                            .block(Block::default().borders(Borders::ALL).title("Select Host"))
                            .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
                        chunks[1],
                    );
                }
                Step::HostNamePrompt => {
                    f.render_widget(
                        Paragraph::new(format!(
                            "Enter new host name (type and press Enter):\n{}",
                            app.host_name
                        ))
                        .block(Block::default().borders(Borders::ALL).title("New Host"))
                        .alignment(Alignment::Left),
                        chunks[1],
                    );
                }
                Step::Done => {
                    let host = if app.host_is_new {
                        if app.host_name.is_empty() {
                            "<unnamed host>"
                        } else {
                            &app.host_name
                        }
                    } else {
                        app.hosts[app.host_idx]
                    };
                    let summary = format!(
                        "Profile: {}\nHost: {}",
                        app.profiles[app.profile_idx],
                        host
                    );
                    f.render_widget(
                        Paragraph::new(summary)
                            .block(Block::default().borders(Borders::ALL).title("Summary"))
                            .alignment(Alignment::Left),
                        chunks[1],
                    );
                }
            }

            // Step navigation info
            let nav = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("<Up/Down>", Style::default().fg(Color::Cyan)),
                    Span::raw(" Move  "),
                    Span::styled("<Right/Enter>", Style::default().fg(Color::Cyan)),
                    Span::raw(" Next  "),
                    Span::styled("<Left>", Style::default().fg(Color::Cyan)),
                    Span::raw(" Prev  "),
                    Span::styled("<q>", Style::default().fg(Color::Red)),
                    Span::raw(" Quit"),
                ])
            ])
            .alignment(Alignment::Center);
            f.render_widget(nav, chunks[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.current_step() {
                    Step::ProfileSelection => match key.code {
                        KeyCode::Down => {
                            if app.profile_idx + 1 < app.profiles.len() { app.profile_idx += 1; }
                        }
                        KeyCode::Up => {
                            if app.profile_idx > 0 { app.profile_idx -= 1; }
                        }
                        KeyCode::Right | KeyCode::Enter => app.next_step(),
                        KeyCode::Left => app.prev_step(),
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    Step::HostConfig => match key.code {
                        KeyCode::Down => {
                            if app.host_idx + 1 < app.hosts.len() { app.host_idx += 1; }
                        }
                        KeyCode::Up => {
                            if app.host_idx > 0 { app.host_idx -= 1; }
                        }
                        KeyCode::Right | KeyCode::Enter => {
                            if app.host_idx == app.hosts.len() - 1 {
                                // "Create new host..." selected
                                app.host_is_new = true;
                                app.host_name = String::new();
                                app.step_idx = Step::all().iter().position(|s| *s == Step::HostNamePrompt).unwrap();
                            } else {
                                app.host_is_new = false;
                                app.step_idx = Step::all().iter().position(|s| *s == Step::Done).unwrap();
                            }
                        }
                        KeyCode::Left => app.prev_step(),
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    Step::HostNamePrompt => match key.code {
                        KeyCode::Enter => app.step_idx = Step::all().iter().position(|s| *s == Step::Done).unwrap(),
                        KeyCode::Backspace => { app.host_name.pop(); },
                        KeyCode::Char('q') => break,
                        KeyCode::Char(c) => app.host_name.push(c),
                        KeyCode::Esc | KeyCode::Left => app.prev_step(),
                        _ => {}
                    },
                    Step::Done => match key.code {
                        KeyCode::Left => {
                            if app.host_is_new {
                                app.step_idx = Step::all().iter().position(|s| *s == Step::HostNamePrompt).unwrap();
                            } else {
                                app.step_idx = Step::all().iter().position(|s| *s == Step::HostConfig).unwrap();
                            }
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                }
            }
        }
    }
    Ok(())
}
