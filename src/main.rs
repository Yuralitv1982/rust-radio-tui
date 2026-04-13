mod api;
mod player;
mod ui;
mod app;

use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let (search_area, list_area, info_area) = ui::render_layout(f);
            ui::render_search_bar(f, search_area, &app.search_query, app.loading);
            ui::render_station_list(f, list_area, &app.stations, app.selected);
            ui::render_info_panel(f, info_area, &app.status, app.volume);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
                    KeyCode::Enter => {
                        if !app.loading {
                            if !app.search_query.is_empty() {
                                tokio::task::block_in_place(|| {
                                    tokio::runtime::Handle::current().block_on(app.fetch_from_api());
                                });
                            } else {
                                app.play_selected();
                            }
                        }
                    }
                    KeyCode::Char(' ') => app.stop(),
                    KeyCode::Char('+') => app.volume_up(),
                    KeyCode::Char('-') => app.volume_down(),
                    KeyCode::Char(c) if c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '.' => {
                        app.search_query.push(c);
                        app.apply_filter();
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                        app.apply_filter();
                    }
                    KeyCode::Esc => {
                        app.search_query.clear();
                        app.apply_filter();
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}