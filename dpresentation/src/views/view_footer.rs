use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn view_footer(frame: &mut Frame, area: Rect) {
    let footer_text = "(q) Exit | (k) Up | (j) Down | (l) Edit Mode";
    frame.render_widget(
        Paragraph::new(footer_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Control Guide"),
            ),
        area,
    );
}
