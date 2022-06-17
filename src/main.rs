use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Tabs};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};
struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["split windows", "inputs", "multi inputs"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
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
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tab_rect = chunks[0];

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(format!("index: {}", app.index)))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, tab_rect);

    match app.index {
        0 => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            let left_block_text = "このように日本語も表示される\n改行を入れれば改行される\nwrapすれば改行をせずに一文で表示できる\n長すぎると見切れるるううううううううううううううううううううううううううううううううううううううううううううううううう";
            let left_block = Paragraph::new(left_block_text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Left);
            f.render_widget(left_block, chunks[0]);

            let middle_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);
            let middle_block_text =
                "This is a sentence.\ntomorrow and tomorrow and tomorrow\nThis is a pen.";
            let middle_top_block = Paragraph::new(middle_block_text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Center);
            f.render_widget(middle_top_block, middle_chunks[0]);

            let center_block = Paragraph::new(middle_block_text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Center);
            f.render_widget(center_block, middle_chunks[1]);

            let middle_bottom = Paragraph::new(middle_block_text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Center);
            f.render_widget(middle_bottom, middle_chunks[2]);

            let right_block_text =
                "This is a right block's text.\n Do you see me?\nToday is the day";
            let right_block = Paragraph::new(right_block_text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Right);
            f.render_widget(right_block, chunks[2]);
        }
        1 => {
            let block = Block::default().title("Inner 1").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        }
        2 => {
            let block = Block::default().title("Inner 1").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        }
        _ => {}
    }
}
