use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn view_popup(frame: &mut Frame, message: String) {
    let pop_area = center(
        frame.area(),
        Constraint::Percentage(20),
        Constraint::Length(3), // top and bottom border + content
    );

    // ポップアップブロックのスタイルを設定
    let block = Block::default()
        .title("Popup")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .title_style(Style::default().fg(Color::Yellow));

    // メッセージを表示するパラグラフを作成
    let paragraph = Paragraph::new(message)
        .block(block)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }); // テキストが幅に合わせて改行されるように設定

    // 背景をクリアしてポップアップをレンダリング
    frame.render_widget(Clear, frame.area());
    frame.render_widget(paragraph, pop_area);
}
