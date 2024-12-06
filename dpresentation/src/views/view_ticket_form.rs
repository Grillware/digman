use std::collections::HashMap;

use dapplication::dtos::ticket_dto::TicketDTO;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;

pub fn view_ticket_form(
    frame: &mut Frame,
    mut text_areas: HashMap<String, TextArea>,
    selected_ticket: TicketDTO,
) {
    // 各フィールドの初期値を TextArea に設定
    if let Some(title_area) = text_areas.get_mut("title") {
        title_area.insert_str(selected_ticket.title.clone());
    }
    if let Some(condition_area) = text_areas.get_mut("completion_condition") {
        condition_area.insert_str(selected_ticket.completion_condition.clone());
    }
    if let Some(level_area) = text_areas.get_mut("level") {
        level_area.insert_str(selected_ticket.level.clone());
    }
    if let Some(status_area) = text_areas.get_mut("status") {
        status_area.insert_str(selected_ticket.status.clone());
    }

    // レイアウト分割
    let rects = Layout::vertical([
        Constraint::Percentage(20), // タイトルエリア
        Constraint::Percentage(70),
    ])
    .split(frame.area());

    let main_row = Layout::horizontal([
        Constraint::Percentage(62), // 完了条件エリア
        Constraint::Percentage(38), // レベル・ステータスエリア
    ])
    .split(rects[1]);

    let sub_row = Layout::vertical([
        Constraint::Percentage(50), // レベルエリア
        Constraint::Percentage(50), // ステータスエリア
    ])
    .split(main_row[1]);

    // 各エリアの矩形
    let title_area = rects[0];
    let condition_area = main_row[0];
    let level_area = sub_row[0];
    let status_area = sub_row[1];

    // 共通のスタイル
    let block_style = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black));
    let text_style = Style::default().fg(Color::White);

    // 各テキストエリアを Paragraph として描画
    if let Some(title_text_area) = text_areas.get("title") {
        let content = title_text_area.lines().join("\n");
        let paragraph = Paragraph::new(content)
            .block(
                block_style
                    .clone()
                    .title(Span::styled("Title", Style::default().fg(Color::Cyan))),
            )
            .style(text_style);
        frame.render_widget(paragraph, title_area);
    }

    if let Some(condition_text_area) = text_areas.get("completion_condition") {
        let content = condition_text_area.lines().join("\n");
        let paragraph = Paragraph::new(content)
            .block(block_style.clone().title(Span::styled(
                "Completion Condition",
                Style::default().fg(Color::Cyan),
            )))
            .style(text_style);
        frame.render_widget(paragraph, condition_area);
    }

    if let Some(level_text_area) = text_areas.get("level") {
        let content = level_text_area.lines().join("\n");
        let paragraph = Paragraph::new(content)
            .block(
                block_style
                    .clone()
                    .title(Span::styled("Level", Style::default().fg(Color::Cyan))),
            )
            .style(text_style);
        frame.render_widget(paragraph, level_area);
    }

    if let Some(status_text_area) = text_areas.get("status") {
        let content = status_text_area.lines().join("\n");
        let paragraph = Paragraph::new(content)
            .block(
                block_style
                    .clone()
                    .title(Span::styled("Status", Style::default().fg(Color::Cyan))),
            )
            .style(text_style);
        frame.render_widget(paragraph, status_area);
    }
}
