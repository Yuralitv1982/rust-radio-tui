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
            Constraint::Min(12),
            Constraint::Length(4),
        ])
        .split(f.area());
    (chunks[0], chunks[1], chunks[2])
}

pub fn render_search_bar(f: &mut Frame, area: Rect, query: &str, is_focused: bool, loading: bool) {
    let icon = if is_focused { "🔎" } else { "📻" };
    let spinner = if loading { " 🔄" } else { "" };
    let placeholder = if query.is_empty() { "type: jazz | ua:rock | us:news | press / to search" } else { query };

    let style = if is_focused { Theme::accent() } else { Theme::panel() };
    let block = Block::bordered().title(format!("{}{}{}", icon, if query.is_empty() { " Search" } else { "" }, spinner)).style(style);

    let paragraph = Paragraph::new(placeholder)
        .block(block)
        .style(if query.is_empty() && !is_focused { Style::default().fg(Color::DarkGray) } else { Theme::accent() });
    f.render_widget(paragraph, area);

    if is_focused && !query.is_empty() && !loading {
        f.set_cursor(area.x + 3 + query.len() as u16, area.y + 1);
    }
}

pub fn render_station_list(f: &mut Frame, area: Rect, stations: &[crate::api::Station], selected: usize) {
    let items: Vec<ListItem> = stations.iter()
        .map(|s| ListItem::new(format!("{} | {} | {}", s.name, s.country, s.style())))
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("📻 Stations (↑↓ navigate, Enter play, / search, r refresh)"))
        .highlight_style(Style::default().bg(Color::Rgb(40, 40, 60)).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    let mut state = ListState::default();
    state.select(Some(selected));
    f.render_stateful_widget(list, area, &mut state);
}

pub fn render_info_panel(f: &mut Frame, area: Rect, status: &str, volume: u8) {
    let text = format!("{} | Vol: {}%\n[/] search | [Esc] clear | [Enter] play | [r] global refresh", status, volume);
    let paragraph = Paragraph::new(text)
        .block(Block::bordered().title("ℹ️ Controls"))
        .style(Theme::accent());
    f.render_widget(paragraph, area);
}