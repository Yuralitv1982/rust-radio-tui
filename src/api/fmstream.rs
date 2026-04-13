// src/api/fmstream.rs — ПРАВИЛЬНЫЕ АТРИБУТЫ
// Correct attributes

use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Stream {
    #[allow(dead_code)]  // ✅ Внешний атрибут для поля
    pub bitrate: u16,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]  // ✅ Внешний атрибут для всей структуры
pub struct Station {
    pub name: String,
    pub country: String,
    pub style: String,
    pub streams: Vec<Stream>,
}

pub struct FmStreamClient {
    client: Client,
    base_url: String,
}

impl FmStreamClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://fmstream.org/api.php".to_string(),
        }
    }

    pub async fn search(
        &self,
        country: Option<&str>,
        style: Option<&str>,
        query: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Station>> {
        let mut params = Vec::new();
        if let Some(c) = country { params.push(("c", c)); }
        if let Some(s) = style { params.push(("style", s)); }
        if let Some(q) = query { params.push(("s", q)); }
        params.push(("hq", "1")); // только высокое качество
        params.push(("n", "0"));

        let url = format!("{}?{}", self.base_url, serde_urlencoded::to_string(params)?);
        let response = self.client.get(&url).send().await?.json::<Vec<Station>>().await?;
        Ok(response.into_iter().take(limit).collect())
    }

    // Для тестов: вернуть моковые данные без сети
    // src/api/fmstream.rs — замени функцию mock_stations() на эту:
    // Replace the mock_stations() function with this:

    pub fn mock_stations() -> Vec<Station> {
        vec![
            Station {
                name: "Radio Paradise — Rock".into(),
                country: "USA".into(),
                style: "Rock".into(),
                streams: vec![Stream {
                    bitrate: 128,
                    url: "http://stream.radioparadise.com/rock-128".into()
                }],
            },
            Station {
                name: "SomaFM — Groove Salad".into(),
                country: "USA".into(),
                style: "Ambient".into(),
                streams: vec![Stream {
                    bitrate: 128,
                    url: "http://ice1.somafm.com/groovesalad-128-mp3".into()
                }],
            },
            Station {
                name: "BBC Radio 1".into(),
                country: "UK".into(),
                style: "Pop".into(),
                streams: vec![Stream {
                    bitrate: 128,
                    url: "http://stream.live.vc.bbcmedia.co.uk/bbc_radio_one".into()
                }],
            },
            Station {
                name: "Radio Swiss Jazz".into(),
                country: "CH".into(),
                style: "Jazz".into(),
                streams: vec![Stream {
                    bitrate: 128,
                    url: "http://stream.srg-ssr.ch/m/rsj/mp3_128".into()
                }],
            },
        ]
    }
}