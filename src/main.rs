mod api;
mod player;
mod ui;
mod app;

use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
// #[allow(unused_imports)]  // ✅ Раскомментируй, если оставляешь Frame
// use ratatui::Frame;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let (search_area, list_area, info_area) = ui::render_layout(f);
            ui::render_search_bar(f, search_area, &app.query);
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
                    KeyCode::Enter => app.play_selected(),
                    KeyCode::Char(' ') => app.stop(),
                    KeyCode::Char('+') => app.volume_up(),
                    KeyCode::Char('-') => app.volume_down(),

                    // Упрощённая версия для MVP:
                    // Simplified version for MVP:
                    KeyCode::Char('r') if !app.loading => {
                        app.loading = true;
                        app.status = "🔄 Fetching...".into();

                        // Блокирующий вызов, но с флагом
                        // Blocking call, but with flag
                        let client = api::FmStreamClient::new();
                        match tokio::task::block_in_place(|| {
                            tokio::runtime::Handle::current().block_on(
                                client.search(None, None, None, 20)
                            )
                        }) {
                            Ok(stations) => {
                                app.stations = stations;
                                app.selected = 0;
                                app.status = format!("✅ Loaded {}", app.stations.len());
                            }
                            Err(e) => app.status = format!("❌ {}", e),
                        }
                        app.loading = false;
                    }

                    KeyCode::Char(c) => app.query.push(c),
                    KeyCode::Esc => app.query.clear(),
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}