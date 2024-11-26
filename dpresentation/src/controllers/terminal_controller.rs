use color_eyre::{eyre::Ok, Result};
use dapplication::input_ports::terminal_input_port::TerminalInputPort;
use ddomain::value_objects::app_mode::AppMode;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};

pub struct TerminalController<T: TerminalInputPort> {
    mode: AppMode,
    prev_mode: AppMode,
    input_port: T,
}

impl<T: TerminalInputPort> TerminalController<T> {
    pub fn new(input_port: T) -> Self {
        TerminalController {
            mode: AppMode::Normal,
            prev_mode: AppMode::Normal,
            input_port,
        }
    }

    fn transition_mode(&mut self, new_mode: AppMode) {
        if self.mode != new_mode {
            self.prev_mode = self.mode.clone();
            self.mode = new_mode;
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
                AppMode::Amend => {
                    let _ = self.input_port.mode_amend(frame);
                }
                AppMode::Raise => {
                    let _ = self.input_port.mode_raise(frame);
                }
                AppMode::Notification => {
                    let _ = self.input_port.mode_notification(frame);
                }
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
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
            return Ok(true);
        }

        match (&self.mode, key.code) {
            // 終了
            (_, KeyCode::Char('q')) if !matches!(self.mode, AppMode::Amend | AppMode::Raise) => {
                return Ok(true)
            }

            // 通常時
            (AppMode::Normal, KeyCode::Char('g')) => self.input_port.switch_display_guide(),
            (AppMode::Normal, KeyCode::Char('l')) => {
                self.transition_mode(AppMode::Inquery);
            }
            (AppMode::Normal, KeyCode::Char('+')) => {
                self.transition_mode(AppMode::Raise);
            }
            (AppMode::Normal, KeyCode::Char('j') | KeyCode::Down) => self.input_port.next_row()?,
            (AppMode::Normal, KeyCode::Char('k') | KeyCode::Up) => {
                self.input_port.previous_row()?
            }

            // 照会
            (AppMode::Inquery, KeyCode::Char('l')) => {
                self.transition_mode(AppMode::Amend);
            }
            (AppMode::Inquery, KeyCode::Char('h') | KeyCode::Left) => {
                self.transition_mode(AppMode::Normal);
            }
            (AppMode::Inquery, KeyCode::Char('j') | KeyCode::Down) => self.input_port.next_row()?,
            (AppMode::Inquery, KeyCode::Char('k') | KeyCode::Up) => {
                self.input_port.previous_row()?
            }

            // 訂正
            (AppMode::Amend, KeyCode::Esc) => {
                self.transition_mode(AppMode::Inquery);
            }
            (AppMode::Amend, KeyCode::Char('q')) => {
                self.input_port.submit()?;
                self.transition_mode(AppMode::Notification);
            }
            (AppMode::Amend, _) => self.input_port.handle_input(key.code)?,

            // 起票
            (AppMode::Raise, KeyCode::Esc) => {
                self.transition_mode(AppMode::Normal);
            }
            (AppMode::Raise, _) => self.input_port.handle_input(key.code)?,

            // 通知
            (AppMode::Notification, _) => self.transition_mode(self.prev_mode.clone()),

            // その他のキー入力
            _ => {}
        }

        Ok(false)
    }
}
