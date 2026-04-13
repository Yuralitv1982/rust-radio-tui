use ratatui::style::{Color, Style, Modifier};

pub struct Theme;

impl Theme {
    pub const BG: Color = Color::Rgb(18, 18, 24);
    pub const FG: Color = Color::Rgb(220, 220, 230);
    pub const ACCENT: Color = Color::Rgb(100, 200, 255);
    pub const SUCCESS: Color = Color::Rgb(100, 255, 150);

    pub fn panel() -> Style { Style::default().bg(Self::BG).fg(Self::FG) }
    pub fn accent() -> Style { Style::default().fg(Self::ACCENT).add_modifier(Modifier::BOLD) }
    pub fn success() -> Style { Style::default().fg(Self::SUCCESS) }
}