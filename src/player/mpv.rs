use std::process::{Command, Child, Stdio};
use anyhow::Result;

pub struct MpvPlayer {
    child: Option<Child>,
    current_url: Option<String>,
    volume: u8,
}

impl MpvPlayer {
    pub fn new() -> Self {
        Self { child: None, current_url: None, volume: 50 }
    }

    pub fn play(&mut self, url: &str) -> Result<()> {
        self.stop();
        let child = Command::new("mpv")
            .arg("--no-video")
            .arg("--quiet")
            .arg("--no-terminal")
            .arg("--msg-level=all=no")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()?;
        self.child = Some(child);
        self.current_url = Some(url.to_string());
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.current_url = None;
    }

    pub fn set_volume(&mut self, volume: u8) -> Result<()> {
        self.volume = volume;
        let output = Command::new("wpctl")
            .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &format!("{}%", volume)])
            .output()?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("wpctl failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(())
    }

    pub fn adjust_volume(&mut self, delta: i8) -> Result<()> {
        let val = delta.abs() as u8;
        let sign = if delta >= 0 { "+" } else { "-" };
        let vol_str = format!("{}%{}", val, sign);
        let output = Command::new("wpctl")
            .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &vol_str])
            .output()?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("wpctl failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
        if delta > 0 { self.volume = (self.volume + val).min(100); }
        else { self.volume = self.volume.saturating_sub(val); }
        Ok(())
    }

    pub fn volume_up(&mut self, step: u8) -> Result<()> { self.adjust_volume(step as i8) }
    pub fn volume_down(&mut self, step: u8) -> Result<()> { self.adjust_volume(-(step as i8)) }
    pub fn get_volume(&self) -> u8 { self.volume }
    pub fn is_playing(&self) -> bool { self.child.is_some() }

    pub fn play_with_feedback(&mut self, url: &str, station_name: &str) -> Result<String> {
        match self.play(url) {
            Ok(_) => Ok(format!("▶ Playing: {}", station_name)),
            Err(e) => Err(anyhow::anyhow!("❌ Failed: {}", e)),
        }
    }
}

impl Drop for MpvPlayer {
    fn drop(&mut self) { self.stop(); }
}