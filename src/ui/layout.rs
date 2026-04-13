use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render_layout(f: &mut Frame) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Поиск / фильтры
            Constraint::Min(10),     // Список станций
            Constraint::Length(5),   // Инфо-панель
        ])
        .split(f.area());
    (chunks[0], chunks[1], chunks[2])
}

pub fn render_search_bar(f: &mut Frame, area: Rect, query: &str) {
    use ratatui::widgets::{Block, Paragraph};
    let block = Block::bordered().title("🔍 Search (type to filter, ESC to clear)");
    let paragraph = Paragraph::new(query).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_station_list(
    f: &mut Frame,
    area: Rect,
    stations: &[crate::api::Station],
    selected: usize,
) {
    use ratatui::{
        widgets::{List, ListItem, Block},
        style::{Style, Color},
    };
    let items: Vec<ListItem> = stations
        .iter()
        .map(|s| {
            let content = format!("{} | {} | {}", s.name, s.country, s.style);
            ListItem::new(content)
        })
        .collect();
    let list = List::new(items)
        .block(Block::bordered().title("📻 Stations (↑↓ to select, Enter to play)"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(Some(selected)));
}

pub fn render_info_panel(f: &mut Frame, area: Rect, status: &str, volume: u8) {
    use ratatui::widgets::{Block, Paragraph};
    let text = format!("{}\nVolume: {}% | q: quit | f: favorite", status, volume);
    let paragraph = Paragraph::new(text).block(Block::bordered().title("ℹ️ Info"));
    f.render_widget(paragraph, area);
}