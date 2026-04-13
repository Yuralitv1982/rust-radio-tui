use crate::api::{FmStreamClient, Station};
use crate::player::MpvPlayer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusMode { List, Search }

pub struct App {
    pub stations: Vec<Station>,
    pub selected: usize,
    pub search_query: String,
    pub focus: FocusMode,
    pub player: MpvPlayer,
    pub volume: u8,
    pub status: String,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            stations: FmStreamClient::mock_stations(),
            selected: 0,
            search_query: String::new(),
            focus: FocusMode::List,
            player: MpvPlayer::new(),
            volume: 50,
            status: "Нажми '/' для поиска или 'r' для загрузки".into(),
            loading: false,
        }
    }

    pub fn select_next(&mut self) {
        if !self.stations.is_empty() { self.selected = (self.selected + 1) % self.stations.len(); }
    }
    pub fn select_prev(&mut self) {
        if !self.stations.is_empty() { self.selected = self.selected.saturating_sub(1); }
    }

    pub fn type_char(&mut self, c: char) {
        self.search_query.push(c);
    }
    pub fn backspace(&mut self) {
        self.search_query.pop();
    }

    /// ✅ Умный поиск: поддерживает синтаксис "страна:запрос"
    /// Примеры: "jazz", "ua:rock", "us:news", "de:classical"
    pub async fn execute_search(&mut self) {
        if self.search_query.trim().is_empty() { return; }
        self.loading = true;
        self.status = format!("🔍 Searching: '{}'...", self.search_query);

        let (country, name) = if self.search_query.contains(':') {
            let parts: Vec<&str> = self.search_query.splitn(2, ':').collect();
            (Some(parts[0].to_uppercase()), parts[1].to_string())
        } else {
            (None, self.search_query.clone())
        };

        let client = FmStreamClient::new();
        match client.search(country.as_deref(), &name, 50).await {
            Ok(stations) => {
                self.stations = stations;
                self.selected = 0;
                let c_info = if let Some(c) = country { format!(" [{}] ", c) } else { String::new() };
                self.status = format!("✅ Found {} stations{}| '{}'", self.stations.len(), c_info, name);
            }
            Err(e) => self.status = format!("❌ {}", e),
        }
        self.loading = false;
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.focus = FocusMode::List;
    }

    pub fn play_selected(&mut self) {
        if let Some(station) = self.stations.get(self.selected) {
            if let Some(url) = station.get_best_stream() {
                self.player.stop();
                match self.player.play(&url) {
                    Ok(_) => self.status = format!("▶ Playing: {}", station.name),
                    Err(e) => self.status = format!("❌ Play error: {}", e),
                }
            } else {
                self.status = format!("⚠️ Нет ссылки: {}", station.name);
            }
        }
    }

    pub fn stop(&mut self) { self.player.stop(); self.status = "⏹ Stopped".into(); }
    pub fn volume_up(&mut self) { if let Err(e) = self.player.volume_up(5) { self.status = format!("⚠️ {}", e); } }
    pub fn volume_down(&mut self) { if let Err(e) = self.player.volume_down(5) { self.status = format!("⚠️ {}", e); } }

    pub async fn fetch_from_api(&mut self) {
        self.loading = true;
        self.status = "🔄 Fetching global top...".into();
        let client = FmStreamClient::new();
        match client.search(None, "", 50).await {
            Ok(stations) => { self.stations = stations; self.selected = 0; self.status = format!("✅ Loaded {} stations", self.stations.len()); }
            Err(e) => self.status = format!("❌ {}", e),
        }
        self.loading = false;
    }
}