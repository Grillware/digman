use crate::dtos::ticket_dto::TicketDTO;
use crate::input_ports::terminal_input_port::TerminalInputPort;
use crate::output_ports::terminal_output_port::TerminalOutputPort;
use color_eyre::{Result, eyre::Ok};
use ddomain::repositories::ticket_repository::TicketRepository;
use ddomain::{entites::ticket::Ticket, value_objects::app_mode::AppMode};
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
};
use tui_textarea::{Input, Key, TextArea};

pub struct TerminalInteractor<'a, R: TicketRepository, O: TerminalOutputPort> {
    selected_ticket_index: Option<usize>,
    items: Vec<Ticket>,
    repository: R,
    output_port: O,
    text_area: TextArea<'a>,
}

impl<'a, R: TicketRepository, O: TerminalOutputPort> TerminalInteractor<'a, R, O> {
    pub fn new(repository: R, output_port: O) -> Result<Self> {
        let items = repository.fetch_tickets()?;
        Ok(Self {
            selected_ticket_index: Some(0),
            items,
            repository,
            output_port,
            text_area: TextArea::new(vec![]),
        })
    }
}

impl<'a, R: TicketRepository, O: TerminalOutputPort> TerminalInputPort
    for TerminalInteractor<'a, R, O>
{
    fn read_key(&self) -> Result<Option<KeyCode>> {
        if let Event::Key(key_event) = event::read()? {
            Ok(Some(key_event.code))
        } else {
            Ok(None)
        }
    }

    fn next_row(&mut self) -> Result<()> {
        if let Some(index) = self.selected_ticket_index {
            if index < self.items.len() - 1 {
                self.selected_ticket_index = Some(index + 1);
            }
        } else {
            self.selected_ticket_index = Some(0);
        }
        self.output_port.next_row(self.items.len());
        Ok(())
    }

    fn previous_row(&mut self) -> Result<()> {
        if let Some(index) = self.selected_ticket_index {
            if index > 0 {
                self.selected_ticket_index = Some(index - 1);
            }
        } else {
            self.selected_ticket_index = Some(self.items.len().saturating_sub(1));
        }
        self.output_port.previous_row(self.items.len());
        Ok(())
    }

    fn mode_inquery(&mut self, frame: &mut Frame) -> Result<()> {
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(4)]).split(frame.area());

        let selected_ticket = self
            .selected_ticket_index
            .and_then(|i| self.items.get(i).cloned())
            .unwrap();

        self.output_port
            .draw_ticket_detail(frame, rects[0], selected_ticket.into());

        self.output_port
            .draw_footer(frame, rects[1], AppMode::Inquery.to_string());

        Ok(())
    }

    fn mode_normal(&mut self, frame: &mut Frame) -> Result<()> {
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(4)]).split(frame.area());
        let ticket_dtos: Vec<TicketDTO> = self.items.iter().cloned().map(Into::into).collect();

        self.output_port.render_scrollbar(frame, rects[0]);
        self.output_port.draw_table(
            frame,
            rects[0],
            self.output_port.selected_index(),
            &ticket_dtos,
        );
        self.output_port
            .draw_footer(frame, rects[1], AppMode::Normal.to_string());

        Ok(())
    }

    fn mode_register(&mut self, frame: &mut Frame) -> Result<()> {
        // レイアウトを設定
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(4)]).split(frame.area());
        let selected_ticket = self
            .selected_ticket_index
            .and_then(|i| self.items.get(i).cloned())
            .unwrap();
        // テキストエリアをフォームの一部として描画
        self.output_port.draw_ticket_form(
            frame,
            rects[0],
            &mut self.text_area,
            selected_ticket.into(),
        );

        // フッターの描画
        self.output_port
            .draw_footer(frame, rects[1], AppMode::Register.to_string());

        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char(c) => {
                // Construct the Input with Key::Char and no modifiers (no Ctrl, Alt, or Shift)
                let input = Input {
                    key: Key::Char(c),
                    ctrl: false,
                    alt: false,
                    shift: false,
                };
                self.text_area.input(input);
            }
            KeyCode::Backspace | KeyCode::Delete => {
                self.text_area.delete_char();
            }
            _ => {}
        }
        Ok(())
    }
}
