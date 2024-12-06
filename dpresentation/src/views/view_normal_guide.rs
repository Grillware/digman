use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn view_normal_guide(frame: &mut Frame, area: Rect) {
    let guide_text = format!("Mode: NORMAL | (q) Exit | (k) Up | (j) Down | (l) Inquery Mode");

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
