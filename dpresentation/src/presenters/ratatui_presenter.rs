use dapplication::{
    dtos::ticket_dto::TicketDTO, output_ports::terminal_output_port::TerminalOutputPort,
};
use ddomain::entites::table_colors::TableColors;
use ratatui::{
    Frame,
    layout::{Constraint, Margin, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table, TableState,
    },
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
        let header_style = Style::default().fg(Color::White).bg(Color::Blue);
        let header = Row::new(
            [
                "ID",
                "Level",
                "Title",
                "Status",
                "Created At",
                "Resolved At",
            ]
            .iter()
            .map(|&s| Cell::from(s)),
        )
        .style(header_style)
        .height(1);

        let rows: Vec<Row> = tickets
            .iter()
            .enumerate()
            .map(|(i, ticket)| {
                let row_style = if selected_index == Some(i) {
                    Style::default()
                        .fg(self.table_colors.selected_row_style_fg)
                        .bg(self.table_colors.header_bg)
                } else if i % 2 == 0 {
                    Style::default()
                        .fg(self.table_colors.row_fg)
                        .bg(self.table_colors.normal_row_color)
                } else {
                    Style::default()
                        .fg(self.table_colors.row_fg)
                        .bg(self.table_colors.alt_row_color)
                };

                Row::new([
                    Cell::from(ticket.id.as_str()),
                    Cell::from(ticket.level.as_str()),
                    Cell::from(ticket.title.as_str()),
                    Cell::from(ticket.status.as_str()),
                    Cell::from(ticket.created_at.to_rfc3339()),
                    Cell::from(
                        ticket
                            .resolved_at
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_else(|| "".to_string()),
                    ),
                ])
                .style(row_style)
            })
            .collect();

        let widths = vec![
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(30),
            Constraint::Length(15),
            Constraint::Length(25),
            Constraint::Length(25),
        ];

        frame.render_stateful_widget(
            Table::new(std::iter::once(header).chain(rows), &widths)
                .block(Block::default().borders(Borders::ALL).title("Ticket List")),
            area,
            &mut self.state
        );
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
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

    fn draw_edit_form(&self, frame: &mut Frame, area: Rect, selected_ticket: Option<&str>) {
        let form_text = match selected_ticket {
            Some(ticket) => format!("Selected Ticket: {}", ticket),
            None => "Edit Mode: No ticket selected.".to_string(),
        };

        let paragraph = Paragraph::new(form_text)
            .block(Block::default().borders(Borders::ALL).title("Edit Screen"));
        frame.render_widget(paragraph, area);
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
