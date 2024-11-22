use crate::dtos::ticket_dto::TicketDTO;
use ratatui::{Frame, layout::Rect};

pub trait TerminalOutputPort {
    fn draw_table(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected_index: Option<usize>,
        tickets: &Vec<TicketDTO>,
    );
    fn draw_footer(&self, frame: &mut Frame, area: Rect);
    fn draw_edit_form(&self, frame: &mut Frame, area: Rect, selected_ticket: Option<&str>);
    fn next_row(&mut self, items_len: usize);
    fn previous_row(&mut self, items_len: usize);
    fn selected_index(&self) -> Option<usize>;
    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect);
}
