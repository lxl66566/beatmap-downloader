use std::borrow::BorrowMut;

use ratatui::{
    layout::{Constraint, Direction},
    prelude::{Alignment, Frame, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        calendar::Monthly, Block, BorderType, Borders, HighlightSpacing, List, ListState, Paragraph,
    },
};

use crate::app::App;

/// Render main widget.
pub fn render_main(app: &mut App, f: &mut Frame) {
    // default session color is green
    let session_color = |session: usize| {
        if session == app.session {
            Color::Green
        } else {
            Color::White
        }
    };

    // make a grid layout
    let v_layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(f.size());

    let layouts = [
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(v_layouts[0]),
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(v_layouts[1]),
    ];

    // render in the layout

    // session 0
    f.render_stateful_widget(
        List::new(app.mode.list.clone())
            .style(Style::default().fg(Color::White))
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )
            .block(
                Block::default()
                    .title(t!("mode"))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(session_color(0))),
            ),
        layouts[0][0],
        ListState::default()
            .with_selected(Some(app.mode.cursor))
            .borrow_mut(),
    );
    // session 1: special session
    match app.session {
        // after date
        0 => {}
        1 => {}
        _ => {}
    }

    // session 2
    let styled_list = List::new(app.item.list.clone())
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title(t!("gamemode"))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(session_color(2))),
        );
    f.render_stateful_widget(
        styled_list,
        layouts[1][0],
        ListState::default()
            .with_selected(Some(app.item.cursor()))
            .borrow_mut(),
    );
}

/// Render help widget.
pub fn render_help(_: &mut App, f: &mut Frame) {
    let mut help_message = Text::raw(t!(
        "help.content",
        updown = "↑↓",
        leftright = "←→",
        accept = "Enter",
        quit = "q"
    ));
    help_message.extend(vec![Line::from(vec![
        Span::styled(t!("help.select"), Style::default().fg(Color::Green)),
        Span::raw(t!("separate")),
        Span::styled(t!("help.not_select"), Style::default().fg(Color::White)),
    ])]);

    f.render_widget(
        Paragraph::new(help_message)
            .block(
                Block::default()
                    .title(t!("help"))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        f.size(),
    );
}
