use std::io::Stdout;
use std::sync::mpsc::Receiver;
use std::{
    io, thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::layout::Alignment;
use tui::widgets::{Paragraph, Wrap};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use shared::message::PublicLeaderBoard;

#[derive(Debug)]
pub struct ClientData {
    pub public_leader_board: PublicLeaderBoard,
    pub username: String,
}

pub fn start_ui_display(reader: Receiver<ClientData>) {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .expect("failed to enter alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).expect("Failed to create terminal");

    let tick_rate = Duration::from_millis(250);
    run_app(terminal, reader, tick_rate);
}

fn run_app(
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
    reader: Receiver<ClientData>,
    tick_rate: Duration,
) {
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        for message in reader {
            terminal.draw(|f| ui(f, message)).expect("Failed to draw");

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("Failed to poll for event") {
                if let Event::Key(key) = event::read().expect("Failed to read event") {
                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        // restore terminal
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("Failed to restore terminal");
        terminal.hide_cursor().expect("Failed to hide cursor");
    });
}

fn ui<B: Backend>(f: &mut Frame<B>, data: ClientData) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let paragraph = Paragraph::new(format!("La patate chaude client {}", &data.username))
        .style(Style::default().fg(Color::LightMagenta))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let mut items: Vec<ListItem> = data
        .public_leader_board
        .iter()
        .rev()
        .map(|player| {
            let s = match player.is_active {
                true => Style::default().fg(Color::Green),
                false => Style::default().fg(Color::Red),
            };
            let lines = Spans::from(vec![
                Span::styled(format!("{:<10}", player.name), s),
                Span::raw(" "),
                Span::styled(
                    format!("{:<5}", player.score.to_string()),
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{:<5}", player.steps.to_string()),
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
            ]);

            ListItem::new(vec![lines])
        })
        .collect();
    items.insert(
        0,
        ListItem::new(vec![Spans::from(vec![
            Span::styled(
                format!("{:<10}", "username"),
                Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::raw(" "),
            Span::styled(
                format!("{:<5}", "score"),
                Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::raw(" "),
            Span::styled(
                format!("{:<5}", "steps"),
                Style::default().add_modifier(Modifier::ITALIC),
            ),
        ])]),
    );
    let events_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .start_corner(Corner::TopLeft);
    f.render_widget(events_list, chunks[1]);
}
