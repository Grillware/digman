use color_eyre::{Result, eyre::Ok};
use dapplication::input_ports::terminal_input_port::TerminalInputPort;
use ddomain::value_objects::app_mode::AppMode;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

pub struct TerminalController<T: TerminalInputPort> {
    mode: AppMode,
    input_port: T,
}

impl<T: TerminalInputPort> TerminalController<T> {
    pub fn new(input_port: T) -> Self {
        TerminalController {
            mode: AppMode::Normal,
            input_port,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            let _ = terminal.draw(|frame| match self.mode {
                AppMode::Normal => {
                    let _ = self.input_port.mode_normal(frame);
                }
                AppMode::Inquery => {
                    let _ = self.input_port.mode_inquery(frame);
                }
                AppMode::Register => {
                    let _ = self.input_port.mode_register(frame);
                },
            });

            if self.handle_event(event::read()?)? {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<bool> {
        let Event::Key(key) = event else {
            return Ok(false);
        };
        if key.kind != KeyEventKind::Press {
            return Ok(false);
        }

        match (&self.mode, key.code) {
            (_, KeyCode::Char('q')) if self.mode != AppMode::Register => return Ok(true),

            // Normalモード
            (AppMode::Normal, KeyCode::Char('l')) => self.mode = AppMode::Inquery,
            (AppMode::Normal, KeyCode::Char('j') | KeyCode::Down) => self.input_port.next_row()?,
            (AppMode::Normal, KeyCode::Char('k') | KeyCode::Up) => {
                self.input_port.previous_row()?
            }

            // Inqueryモード
            (AppMode::Inquery, KeyCode::Char('l')) => self.mode = AppMode::Register,
            (AppMode::Inquery, KeyCode::Char('h') | KeyCode::Left) => self.mode = AppMode::Normal,
            (AppMode::Inquery, KeyCode::Char('j') | KeyCode::Down) => self.input_port.next_row()?,
            (AppMode::Inquery, KeyCode::Char('k') | KeyCode::Up) => {
                self.input_port.previous_row()?
            }

            // Registerモード
            (AppMode::Register, KeyCode::Esc) => self.mode = AppMode::Inquery,
            (AppMode::Register, _) => self.input_port.handle_input(key.code)?,

            // その他のキー入力
            _ => {}
        }

        Ok(false)
    }
}
