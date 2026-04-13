// src/ui/visualizer.rs
use ratatui::{
    layout::{Rect, Margin},
    widgets::{Widget, Block, Borders},
    buffer::Buffer,
};
use crate::ui::theme::Theme;

#[derive(Clone)]
pub struct AudioVisualizer {
    pub bars: Vec<u16>,
    pub frame_count: u64,
}

impl AudioVisualizer {
    pub fn new(bar_count: usize) -> Self {
        Self {
            bars: vec![0; bar_count],
            frame_count: 0,
        }
    }

    pub fn update(&mut self, is_playing: bool) {
        self.frame_count = self.frame_count.wrapping_add(1);
        for (i, bar) in self.bars.iter_mut().enumerate() {
            if !is_playing {
                *bar = bar.saturating_sub(2);
            } else {
                let base = ((self.frame_count as f64 + i as f64 * 0.5) * 0.3).sin() * 30.0 + 40.0;
                let noise = (i * 7 + self.frame_count as usize * 3) % 20;
                *bar = (base as u16 + noise as u16).min(100);
            }
        }
    }
}

impl Widget for &AudioVisualizer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .title("🎵 Spectrum")
            .borders(Borders::ALL)
            .style(Theme::panel())
            .render(area, buf);

        let inner = area.inner(Margin { horizontal: 1, vertical: 1 });
        if inner.width == 0 || inner.height == 0 { return; }

        let bar_count = self.bars.len().max(1);
        let bar_width = inner.width as usize / bar_count;
        if bar_width == 0 { return; }

        for (i, &height) in self.bars.iter().enumerate() {
            let x = inner.left() + (i * bar_width) as u16;
            let max_h = inner.height;
            let filled_h = (height as f32 / 100.0 * max_h as f32) as u16;

            for y in 0..filled_h {
                let percent = ((y + 1) * 100 / max_h.max(1)) as u16;
                let style = Theme::gradient_bar(percent);
                let symbol = if y == filled_h - 1 { "▄" } else { "█" };
                buf.set_string(x, inner.bottom() - y - 1, symbol, style);
            }
        }
    }
}