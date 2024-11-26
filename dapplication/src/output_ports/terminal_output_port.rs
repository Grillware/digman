use std::collections::HashMap;

use crate::dtos::ticket_dto::TicketDTO;
use ddomain::value_objects::app_mode::AppMode;
use ratatui::{layout::Rect, Frame};
use tui_textarea::TextArea;

pub trait TerminalOutputPort {
    fn draw_table(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected_index: Option<usize>,
        tickets: &Vec<TicketDTO>,
    );
    fn draw_ticket_detail(&self, frame: &mut Frame, area: Rect, selected_ticket: TicketDTO);
    fn draw_ticket_form(
        &self,
        frame: &mut Frame,
        text_areas: HashMap<String, TextArea>,
        selected_ticket: TicketDTO,
    );

    fn draw_guide(&self, frame: &mut Frame, area: Rect, mode: AppMode);

    fn next_row(&mut self, items_len: usize);
    fn previous_row(&mut self, items_len: usize);
    fn selected_index(&self) -> Option<usize>;
    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect);
    fn notify(&self, frame: &mut Frame, message: String);
}
