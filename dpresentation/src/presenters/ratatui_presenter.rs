use std::collections::HashMap;

use crate::views::view_amend_guide::view_amend_guide;
use crate::views::view_inquery_guide::view_inquery_guide;
use crate::views::view_normal_guide::view_normal_guide;
use crate::views::view_popup::view_popup;
use crate::views::view_raise_guide::view_raise_guide;
use crate::views::{view_ticket_detail::view_ticket_detail, view_ticket_form::view_ticket_form};
use crate::{table_colors::TableColors, views::view_table::view_table};
use dapplication::{
    dtos::ticket_dto::TicketDTO, output_ports::terminal_output_port::TerminalOutputPort,
};
use ddomain::value_objects::app_mode::AppMode;
use ratatui::{
    layout::{Margin, Rect},
    widgets::{Scrollbar, ScrollbarOrientation, TableState},
    Frame,
};
use tui_textarea::TextArea;

pub struct RatatuiPresenter {
    table_colors: TableColors,
    state: TableState,
    scroll_state: ratatui::widgets::ScrollbarState,
}

impl RatatuiPresenter {
    pub fn new(table_colors: TableColors, content_length: usize) -> Self {
        Self {
            table_colors,
            state: TableState::default().with_selected(0),
            scroll_state: ratatui::widgets::ScrollbarState::new(content_length * 4),
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

    fn draw_guide(&self, frame: &mut Frame, area: Rect, mode: AppMode) {
        match mode {
            AppMode::Normal => view_normal_guide(frame, area),
            AppMode::Inquery => view_inquery_guide(frame, area),
            AppMode::Amend => view_amend_guide(frame, area),
            AppMode::Raise => view_raise_guide(frame, area),
            AppMode::Notification => {}
        }
    }

    fn draw_ticket_detail(&self, frame: &mut Frame, area: Rect, selected_ticket: TicketDTO) {
        view_ticket_detail(frame, area, selected_ticket);
    }

    fn draw_ticket_form(
        &self,
        frame: &mut Frame,

        text_areas: HashMap<String, TextArea>,
        selected_ticket: TicketDTO,
    ) {
        view_ticket_form(frame, text_areas, selected_ticket);
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

    fn notify(&self, frame: &mut Frame, message: String) {
        view_popup(frame, message);
    }
}
