use crate::api::{FmStreamClient, Station};
use crate::player::MpvPlayer;

pub struct App {
    pub stations: Vec<Station>,
    pub selected: usize,
    pub query: String,
    pub player: MpvPlayer,
    pub volume: u8,
    pub status: String,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            stations: FmStreamClient::mock_stations(), // старт с моками
            selected: 0,
            query: String::new(),
            player: MpvPlayer::new(),
            volume: 50,
            status: "Ready".into(),
            loading: false,
        }
    }

    pub fn select_next(&mut self) {
        if !self.stations.is_empty() {
            self.selected = (self.selected + 1) % self.stations.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.stations.is_empty() {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    pub fn play_selected(&mut self) {
        if let Some(station) = self.stations.get(self.selected) {
            if let Some(stream) = station.streams.first() {
                match self.player.play_with_feedback(&stream.url, &station.name) {
                    Ok(msg) => self.status = msg,
                    Err(e) => self.status = e.to_string(), // Покажем ошибку в UI
                    // Show error in UI
                }
            }
        }
    }
    pub fn stop(&mut self) {
        self.player.stop();
        self.status = "⏹ Stopped".into();
    }

         pub fn volume_up(&mut self) {
             match self.player.volume_up(5) {
                 Ok(_) => self.status = format!("🔊 Volume: {}%", self.player.get_volume()),
                 Err(e) => self.status = format!("⚠️ {}", e),
             }
         }

       pub fn volume_down(&mut self) {
               match self.player.volume_down(5) {
                   Ok(_) => self.status = format!("🔉 Volume: {}%", self.player.get_volume()),
                   Err(e) => self.status = format!("⚠️ {}", e),
               }
       }


    pub async fn fetch_stations(&mut self) {
        self.loading = true;
        self.status = "🔄 Fetching...".into();
        let client = FmStreamClient::new();
        match client.search(None, None, None, 20).await {
            Ok(stations) => {
                self.stations = stations;
                self.status = format!("✅ Loaded {} stations", self.stations.len());
            }
            Err(e) => {
                self.status = format!("❌ Error: {}", e);
            }
        }
        self.loading = false;
    }
}