use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn view_raise_guide(frame: &mut Frame, area: Rect) {
    let guide_text = format!("Mode: RAISE | (Esc) Inquery Mode | (Ctrl+Return) Confirm");

    frame.render_widget(
        Paragraph::new(guide_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Control Guide"),
            ),
        area,
    );
}
