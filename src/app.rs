use crate::api::{FmStreamClient, Station};
use crate::player::MpvPlayer;

pub struct App {
    pub stations: Vec<Station>,
    pub all_stations: Vec<Station>,
    pub selected: usize,
    pub search_query: String,
    pub player: MpvPlayer,
    pub volume: u8,
    pub status: String,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            stations: FmStreamClient::mock_stations(),
            all_stations: FmStreamClient::mock_stations(),
            selected: 0,
            search_query: String::new(),
            player: MpvPlayer::new(),
            volume: 50,
            status: "Ready".into(),
            loading: false,
        }
    }

    pub fn select_next(&mut self) {
        if !self.stations.is_empty() { self.selected = (self.selected + 1) % self.stations.len(); }
    }
    pub fn select_prev(&mut self) {
        if !self.stations.is_empty() { self.selected = self.selected.saturating_sub(1); }
    }

            pub fn play_selected(&mut self) {
                if let Some(station) = self.stations.get(self.selected) {
                    if let Some(url) = station.get_best_stream() {
                        match self.player.play_with_feedback(&url, &station.name) {
                            Ok(msg) => self.status = msg,
                            Err(e) => self.status = e.to_string(),
                        }
                    } else {
                        self.status = format!("⚠️ No stream URL for {}", station.name);
                    }
                }
            }

    pub fn stop(&mut self) { self.status = "⏹ Stopped".into(); self.player.stop(); }
    pub fn volume_up(&mut self) { if let Err(e) = self.player.volume_up(5) { self.status = format!("⚠️ {}", e); } }
    pub fn volume_down(&mut self) { if let Err(e) = self.player.volume_down(5) { self.status = format!("⚠️ {}", e); } }

    pub fn apply_filter(&mut self) {
        let q = self.search_query.to_lowercase();
        self.stations = self.all_stations.iter()
            .filter(|s| self.search_query.is_empty() || s.name.to_lowercase().contains(&q))
            .cloned()
            .collect();
        self.selected = 0;
        self.status = format!("📋 Local filter: {} results", self.stations.len());
    }

    pub async fn fetch_from_api(&mut self) {
        self.loading = true;
        self.status = "🔄 Fetching from fmstream.org...".into();
        let client = FmStreamClient::new();
        match client.search(None, None, Some(&self.search_query), 30).await {
            Ok(stations) => {
                self.all_stations = stations.clone();
                self.stations = stations;
                self.selected = 0;
                self.status = format!("✅ API loaded: {} stations", self.stations.len());
            }
            Err(e) => self.status = format!("❌ API Error: {}", e),
        }
        self.loading = false;
    }
}