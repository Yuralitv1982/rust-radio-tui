use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Station {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub tags: String, // Радио-браузер возвращает жанры через запятую
    #[serde(default)]
    pub url_resolved: String, // Прямая ссылка на поток (лучший вариант)
    #[serde(default)]
    pub bitrate: u16,
}

impl Station {
    /// Возвращает жанр (первый тег) или "Unknown"
    pub fn style(&self) -> String {
        self.tags.split(',').next().unwrap_or("Unknown").trim().to_string()
    }

    /// Возвращает рабочую ссылку на поток
    pub fn get_best_stream(&self) -> Option<String> {
        if !self.url_resolved.is_empty() { Some(self.url_resolved.clone()) } else { None }
    }
}

pub struct FmStreamClient {
    client: Client,
    base_url: String,
}

impl FmStreamClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("rust-radio/0.1")
                .build()
                .unwrap_or_default(),
            // ✅ Надёжный, открытый API
            base_url: "https://de1.api.radio-browser.info/json/stations/search".to_string(),
        }
    }

    pub async fn search(
        &self,
        country: Option<&str>,
        genre: Option<&str>,
        query: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Station>> {
        let mut params: Vec<(&str, String)> = Vec::new();
        params.push(("limit", limit.to_string()));
        params.push(("hidebroken", "true".to_string()));
        params.push(("order", "votes".to_string()));
        params.push(("reverse", "true".to_string()));

        if let Some(q) = query { if !q.is_empty() { params.push(("name", q.to_string())); } }
        if let Some(c) = country { if !c.is_empty() { params.push(("countrycode", c.to_string())); } }
        if let Some(g) = genre { if !g.is_empty() { params.push(("tag", g.to_string())); } }

        let response = self.client
            .get(&self.base_url)
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP Error: {}", response.status()));
        }

        let stations: Vec<Station> = response.json().await?;
        Ok(stations)
    }

    pub fn mock_stations() -> Vec<Station> {
        vec![
            Station { name: "Radio Paradise".into(), country: "US".into(), tags: "rock,alternative".into(), url_resolved: "http://stream.radioparadise.com/rock-128".into(), bitrate: 128 },
            Station { name: "SomaFM Groove Salad".into(), country: "US".into(), tags: "ambient,electronic".into(), url_resolved: "http://ice1.somafm.com/groovesalad-128-mp3".into(), bitrate: 128 },
            Station { name: "BBC Radio 1".into(), country: "GB".into(), tags: "pop,british".into(), url_resolved: "http://stream.live.vc.bbcmedia.co.uk/bbc_radio_one".into(), bitrate: 128 },
        ]
    }
}