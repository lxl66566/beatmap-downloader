use std::{borrow::BorrowMut, collections::HashMap};

use ratatui::{
    layout::{Constraint, Direction},
    prelude::{Alignment, Frame, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Block, BorderType, Borders, HighlightSpacing, List, ListState, Paragraph, Widget,
    },
};

use crate::app::{App, DEFAULT_BLOCK};

/// Render main widget.
pub fn render_main(app: &mut App, f: &mut Frame) {
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
                DEFAULT_BLOCK
                    .clone()
                    .title(t!("mode"))
                    .style(Style::default().fg(app.session_color(0))),
            ),
        layouts[0][0],
        ListState::default()
            .with_selected(Some(app.mode.cursor))
            .borrow_mut(),
    );

    // session 1: special session
    match app.mode.cursor {
        // calendar
        0 => {
            let store =
                CalendarEventStore(HashMap::from([(app.date, Style::default().red().bold())]));
            Monthly::new(app.date, store)
                .show_surrounding(Style::default().white())
                .show_month_header(Style::default().bold())
                .block(
                    DEFAULT_BLOCK
                        .clone()
                        .title(t!("calendar"))
                        .style(Style::default().fg(app.session_color(1))),
                )
                .render(layouts[0][1], f.buffer_mut());
        }
        1 | 2 => {
            app.text
                .set_placeholder_style(Style::default().fg(app.session_color(1)));
            app.text.set_placeholder_text(t!("input.num"));
            app.validate(1);
            f.render_widget(app.text.widget(), layouts[0][1]);
        }
        3 => {
            app.text2
                .set_placeholder_style(Style::default().fg(app.session_color(1)));
            app.text2.set_placeholder_text(t!("input.name"));
            app.text2.set_block(
                DEFAULT_BLOCK
                    .title(t!("mode.search"))
                    .style(Style::default().fg(app.session_color(1))),
            );
            f.render_widget(app.text2.widget(), layouts[0][1]);
        }
        _ => unreachable!("mode numeber exceeded."),
    }

    // session 2
    app.item.render(
        f,
        DEFAULT_BLOCK
            .clone()
            .title(t!("gamemode"))
            .style(Style::default().fg(app.session_color(2))),
        layouts[1][0],
    );

    // session 3
    app.map_state.render(
        f,
        DEFAULT_BLOCK
            .clone()
            .title(t!("mapstate"))
            .style(Style::default().fg(app.session_color(3))),
        layouts[1][1],
    );
}

/// Render help widget.
pub fn render_help(_: &mut App, f: &mut Frame) {
    let mut help_message = Text::raw(t!(
        "help.content",
        updown = "↑↓",
        leftright = "←→",
        accept = "Enter",
        quit = "ESC"
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
