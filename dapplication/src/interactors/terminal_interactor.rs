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

use std::collections::HashMap;

pub struct TerminalInteractor<'a, R: TicketRepository, O: TerminalOutputPort> {
    selected_ticket_index: Option<usize>,
    items: Vec<Ticket>,
    repository: R,
    output_port: O,
    text_areas: HashMap<String, TextArea<'a>>,
    active_text_area: String,
    need_guide: bool,
    notification_message: String,
}

impl<'a, R: TicketRepository, O: TerminalOutputPort> TerminalInteractor<'a, R, O> {
    pub fn new(repository: R, output_port: O) -> Result<Self> {
        let items = repository.fetch_tickets()?;
        let mut text_areas = HashMap::new();
        text_areas.insert("title".to_string(), TextArea::new(vec![]));
        text_areas.insert("completion_condition".to_string(), TextArea::new(vec![]));
        text_areas.insert("level".to_string(), TextArea::new(vec![]));
        text_areas.insert("status".to_string(), TextArea::new(vec![]));

        Ok(Self {
            selected_ticket_index: Some(0),
            items,
            repository,
            output_port,
            text_areas,
            active_text_area: "title".to_string(),
            need_guide: true,
            notification_message: "".to_string(),
        })
    }

    fn cycle_active_text_area(&mut self) {
        // サイクルでアクティブなテキストエリアを切り替え
        let keys: Vec<&String> = self.text_areas.keys().collect();
        if let Some(pos) = keys.iter().position(|key| **key == self.active_text_area) {
            self.active_text_area = keys[(pos + 1) % keys.len()].clone().to_string();
        }
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

        if self.need_guide {
            self.output_port
                .draw_guide(frame, rects[1], AppMode::Inquery);
        }

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
        if self.need_guide {
            self.output_port
                .draw_guide(frame, rects[1], AppMode::Normal);
        }

        Ok(())
    }
    fn mode_amend(&mut self, frame: &mut Frame) -> Result<()> {
        let selected_ticket = self
            .selected_ticket_index
            .and_then(|i| self.items.get(i).cloned())
            .unwrap();

        // 各テキストエリアを描画
        // レイアウト分割
        let rects = Layout::vertical([
            Constraint::Percentage(90),
            Constraint::Percentage(10), // ガイドエリア
        ])
        .split(frame.area());

        self.output_port.draw_ticket_form(
            frame,
            self.text_areas.clone(),
            selected_ticket.clone().into(),
        );

        if self.need_guide {
            self.output_port.draw_guide(frame, rects[1], AppMode::Amend);
        }

        Ok(())
    }

    fn mode_raise(&mut self, frame: &mut Frame) -> Result<()> {
        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char(c) => {
                if let Some(text_area) = self.text_areas.get_mut(&self.active_text_area) {
                    let input = Input {
                        key: Key::Char(c),
                        ctrl: false,
                        alt: false,
                        shift: false,
                    };
                    text_area.input(input);
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                if let Some(text_area) = self.text_areas.get_mut(&self.active_text_area) {
                    text_area.delete_char();
                }
            }
            KeyCode::Tab => {
                // タブキーでアクティブエリアを切り替え
                self.cycle_active_text_area();
            }
            _ => {}
        }
        Ok(())
    }

    fn submit(&mut self) -> Result<()> {
        // 現在選択されているチケットを取得
        let mut selected_ticket = self
            .selected_ticket_index
            .and_then(|i| self.items.get(i).cloned())
            .ok_or_else(|| color_eyre::eyre::eyre!("No ticket selected"))?;

        // DTOの生成
        let mut dto =
            self.text_areas
                .iter()
                .fold(TicketDTO::default(), |mut dto, (key, text_area)| {
                    let content = text_area.lines().join("\n");
                    match key.as_str() {
                        "title" => dto.title = content,
                        "completion_condition" => dto.completion_condition = content,
                        "level" => dto.level = content,
                        "status" => dto.status = content,
                        _ => {}
                    }
                    dto
                });

        selected_ticket.substitute(|ticket_mut| {
            dto.id = ticket_mut.id.clone();
            dto.created_at = *ticket_mut.created_at;
            dto.resolved_at = *ticket_mut.resolved_at;
        });

        match self.repository.save(dto.into()) {
            std::result::Result::Ok(_) => {
                self.notification_message =
                    format!("The save operation was completed successfully.")
            }
            Err(err) => self.notification_message = format!("The save operation failed: {}", err),
        }

        Ok(())
    }

    fn switch_display_guide(&mut self) {
        self.need_guide = !self.need_guide;
    }

    fn mode_notification(&mut self, frame: &mut Frame) -> Result<()> {
        self.output_port
            .notify(frame, self.notification_message.clone());
        Ok(())
    }
}
