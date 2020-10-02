#[allow(dead_code)]
mod scramble_data;
use crate::scramble_data::scramble_data_vec;
use std::{error::Error, io, io::Read, thread, time::Duration};
use stopwatch::Stopwatch;
use termion::{async_stdin, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut scramble_vec: Vec<String> = scramble_data_vec();
    let mut stopwatch_obj = Stopwatch::new();
    // let mut scroll: u16 = 0;
    let mut stdin = async_stdin().bytes();
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        // Constraint::Percentage(25),
                        // Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(size);

            // let text = vec![
            //     Spans::from("This is a line "),
            //     Spans::from(Span::styled("This is a line   ", Style::default().fg(Color::Red))),
            //     Spans::from(Span::styled("This is a line", Style::default().bg(Color::Blue))),
            //     Spans::from(Span::styled(
            //         "This is a longer line",
            //         Style::default().add_modifier(Modifier::CROSSED_OUT),
            //     )),
            //     Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
            //     Spans::from(Span::styled(
            //         "This is a line",
            //         Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC),
            //     )),
            // ];
            let scramble_text = vec![
                Spans::from(vec![
                    Span::styled(
                        &scramble_vec[0],
                        Style::default()
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("   "),
                    Span::styled(
                        &scramble_vec[1],
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("   "),
                    Span::styled(
                        &scramble_vec[2],
                        Style::default()
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("   "),
                    Span::styled(
                        &scramble_vec[3],
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("   "),
                    Span::styled(
                        &scramble_vec[4],
                        Style::default()
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    ),
                    // Span::styled(
                    //     &scramble_vec[1],
                    //     Style::default().add_modifier(Modifier::ITALIC),
                    // ),
                    // Span::raw("."),
                ]),
                // Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
            ];

            let timer_text = vec![Spans::from(vec![Span::styled(
                format!(
                    "{:02}:{:02}",
                    &stopwatch_obj.elapsed().as_secs() / 60,
                    &stopwatch_obj.elapsed().as_secs(),
                ),
                Style::default()
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )])];

            let create_block = |title| {
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::White).fg(Color::Black))
                    .title(Span::styled(
                        title,
                        Style::default().add_modifier(Modifier::BOLD),
                    ))
            };
            let paragraph = Paragraph::new(scramble_text.clone())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .block(create_block("Last Layer"))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[0]);
            let paragraph = Paragraph::new(timer_text.clone())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .block(create_block("Timer"))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[1]);
            // let paragraph = Paragraph::new(text.clone())
            //     .style(Style::default().bg(Color::White).fg(Color::Black))
            //     .block(create_block("Center, wrap"))
            //     .alignment(Alignment::Center)
            //     .wrap(Wrap { trim: true })
            //     .scroll((scroll, 0));
            // f.render_widget(paragraph, chunks[2]);
            // let paragraph = Paragraph::new(text)
            //     .style(Style::default().bg(Color::White).fg(Color::Black))
            //     .block(create_block("Right, wrap"))
            //     .alignment(Alignment::Right)
            //     .wrap(Wrap { trim: true });
            // f.render_widget(paragraph, chunks[3]);
        })?;

        // scroll += 1;
        // scroll %= 10;
        let b = stdin.next();
        if let Some(Ok(b' ')) = b {
            if stopwatch_obj.is_running() {
                stopwatch_obj.stop();
            } else {
                stopwatch_obj.reset();
                stopwatch_obj.start();
            }
        }

        if let Some(Ok(b'n')) = b {
            scramble_vec = scramble_data_vec();
        }

        if let Some(Ok(b'q')) = b {
            break;
        }
        // adds delay to loop end so the scrolling is slower
        // aka screen redraw iteration
        // And I think time till any key presses get picked up.
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
