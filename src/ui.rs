use ratatui::{
    layout::{Constraint, Direction},
    prelude::{Alignment, Frame, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListState, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());
    f.render_widget(
        Paragraph::new("123")
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        layout[0],
    );

    let styled_list = List::new(app.item.list.clone())
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    let mut state = ListState::default();
    state.select(Some(app.item.cursor()));
    // Render the list
    f.render_stateful_widget(styled_list, layout[1], &mut state)
}
