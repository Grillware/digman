use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn view_edit_form(frame: &mut Frame, area: Rect, selected_ticket: Option<&str>) {
    let form_text = match selected_ticket {
        Some(ticket) => format!("Selected Ticket: {}", ticket),
        None => "Edit Mode: No ticket selected.".to_string(),
    };

    let paragraph = Paragraph::new(form_text)
        .block(Block::default().borders(Borders::ALL).title("Edit Screen"));
    frame.render_widget(paragraph, area);
}
