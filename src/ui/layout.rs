use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, List, ListItem, Paragraph, ListState},
    style::{Style, Color, Modifier},
    Frame,
};
use crate::ui::theme::Theme;

pub fn render_layout(f: &mut Frame) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .split(f.area());
    (chunks[0], chunks[1], chunks[2])
}

pub fn render_search_bar(f: &mut Frame, area: Rect, query: &str, loading: bool) {
    let spinner = if loading { "🔄 " } else { "" };
    let display = if query.is_empty() { "Type to search... [Enter] fetch API / play" } else { query };
    let block = Block::bordered().title(format!("{}🔍 Search", spinner)).style(Theme::panel());
    let paragraph = Paragraph::new(display)
        .block(block)
        .style(if query.is_empty() { Style::default().fg(Color::DarkGray) } else { Theme::accent() });
    f.render_widget(paragraph, area);
    if !query.is_empty() && !loading {
        f.set_cursor(area.x + 5 + query.len() as u16, area.y + 1);
    }
}

pub fn render_station_list(f: &mut Frame, area: Rect, stations: &[crate::api::Station], selected: usize) {
    use ratatui::widgets::{List, ListItem, Block, ListState};
    use ratatui::style::{Style, Color, Modifier};
    use crate::ui::theme::Theme;

    let items: Vec<ListItem> = stations.iter()
        .map(|s| ListItem::new(format!("{} | {} | {}", s.name, s.country, s.style()))) // ✅ Вызов метода
        .collect();
    let list = List::new(items)
        .block(Block::bordered().title("📻 Stations (↑↓ select, Enter play/search, Esc clear)"))
        .highlight_style(Style::default().bg(Color::Rgb(40, 40, 60)).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    let mut state = ListState::default();
    state.select(Some(selected));
    f.render_stateful_widget(list, area, &mut state);
}

pub fn render_info_panel(f: &mut Frame, area: Rect, status: &str, volume: u8) {
    let text = format!("{} | Vol: {}% | q:quit | +/-:vol", status, volume);
    let paragraph = Paragraph::new(text)
        .block(Block::bordered().title("ℹ️ Status"))
        .style(Theme::accent());
    f.render_widget(paragraph, area);
}