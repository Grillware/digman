use dapplication::{
    dtos::ticket_dto::TicketDTO, output_ports::terminal_output_port::TerminalOutputPort,
};
use ratatui::{
    Frame,
    layout::{Margin, Rect},
    widgets::{Scrollbar, ScrollbarOrientation, TableState},
};

use crate::{
    table_colors::TableColors,
    views::{view_edit_form::view_edit_form, view_footer::view_footer, view_table::view_table},
};

pub struct RatatuiPresenter {
    table_colors: TableColors,
    state: TableState,
    scroll_state: ratatui::widgets::ScrollbarState,
}

impl RatatuiPresenter {
    pub fn new(table_colors: TableColors) -> Self {
        Self {
            table_colors,
            state: TableState::default().with_selected(0),
            scroll_state: ratatui::widgets::ScrollbarState::new((23 - 1) * 4),
        }
    }
}

impl TerminalOutputPort for RatatuiPresenter {
    fn draw_table(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected_index: Option<usize>,
        tickets: &Vec<TicketDTO>,
    ) {
        view_table(
            frame,
            area,
            selected_index,
            tickets,
            &self.table_colors,
            &mut self.state,
        );
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        view_footer(frame, area);
    }

    fn draw_edit_form(&self, frame: &mut Frame, area: Rect, selected_ticket: Option<&str>) {
        view_edit_form(frame, area, selected_ticket);
    }

    fn next_row(&mut self, items_len: usize) {
        let i = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(if i >= items_len - 1 { 0 } else { i + 1 }));

        self.scroll_state = self
            .scroll_state
            .position(self.state.selected().unwrap_or(0) * 4);
    }

    fn previous_row(&mut self, items_len: usize) {
        let i = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(if i == 0 { items_len - 1 } else { i - 1 }));

        self.scroll_state = self
            .scroll_state
            .position(self.state.selected().unwrap_or(0) * 4);
    }

    fn selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }
}
