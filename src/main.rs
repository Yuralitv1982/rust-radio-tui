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
            ui::render_search_bar(f, search_area, &app.search_query, app.focus == app::FocusMode::Search, app.loading);
            ui::render_station_list(f, list_area, &app.stations, app.selected);
            ui::render_info_panel(f, info_area, &app.status, app.volume);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                // src/main.rs — исправленный match key.code

                match key.code {
                    KeyCode::Char('q') => break,

                    // 🎯 Навигация (работает всегда)
                    KeyCode::Down => app.select_next(),
                    KeyCode::Up => app.select_prev(),

                    // 🎯 Enter: контекстное действие
                    KeyCode::Enter => match app.focus {
                        app::FocusMode::Search if !app.loading => {
                            tokio::task::block_in_place(|| {
                                tokio::runtime::Handle::current().block_on(app.execute_search());
                            });
                        }
                        app::FocusMode::List if !app.loading => {
                            app.play_selected();
                        }
                        _ => {}
                    },

                    // 🎯 Громкость и управление
                    KeyCode::Char(' ') => app.stop(),
                    KeyCode::Char('+') => app.volume_up(),
                    KeyCode::Char('-') => app.volume_down(),

                    // 🎯 Ctrl+R: обновить список (а не просто 'r')
                    KeyCode::Char('r') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) && !app.loading => {
                        tokio::task::block_in_place(|| {
                            tokio::runtime::Handle::current().block_on(app.fetch_from_api());
                        });
                    }

                    // 🔍 Поиск: вход ТОЛЬКО по '/'
                    KeyCode::Char('/') => { app.focus = app::FocusMode::Search; }

                    // 🔍 Выход из поиска
                    KeyCode::Esc => app.clear_search(),

                    // 🔍 Ввод в режиме поиска: ВСЕ печатные символы (включая s, r, цифры, пробел)
                    KeyCode::Char(c) if app.focus == app::FocusMode::Search => {
                        if c.is_alphanumeric() || c == ' ' || c == '-' || c == '.' || c == ':' || c == '_' {
                            app.type_char(c);
                        }
                    }

                    // 🔍 Backspace в поиске
                    KeyCode::Backspace if app.focus == app::FocusMode::Search => app.backspace(),

                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}