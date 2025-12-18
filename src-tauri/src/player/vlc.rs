#[cfg(feature = "vlc")]
use vlc_rs::{Instance, Media, MediaPlayer};
#[cfg(feature = "vlc")]
use std::sync::Mutex;
#[cfg(feature = "vlc")]
use std::sync::Arc;

#[cfg(feature = "vlc")]
pub struct VlcPlayer {
    instance: Instance,
    player: Mutex<Option<MediaPlayer>>,
}

#[cfg(feature = "vlc")]
impl VlcPlayer {
    pub fn new() -> Option<Self> {
        let instance = Instance::new().ok()?;
        Some(Self {
            instance,
            player: Mutex::new(None),
        })
    }

    pub fn play_file(&self, path: &str) -> Result<(), String> {
        let md = Media::new_path(&self.instance, path).ok_or("Failed to create media")?;
        let mp = MediaPlayer::new(&self.instance).ok_or("Failed to create media player")?;

        mp.set_media(&md);
        mp.play().map_err(|_| "Failed to start playback")?;

        let mut player_lock = self.player.lock().map_err(|_| "Failed to lock player")?;
        *player_lock = Some(mp);

        Ok(())
    }

    pub fn stop(&self) {
        if let Ok(player_lock) = self.player.lock() {
            if let Some(mp) = player_lock.as_ref() {
                mp.stop();
            }
        }
    }

    pub fn pause(&self) {
        if let Ok(player_lock) = self.player.lock() {
            if let Some(mp) = player_lock.as_ref() {
                mp.pause();
            }
        }
    }

    pub fn set_audio_track(&self, track_id: i32) {
        if let Ok(player_lock) = self.player.lock() {
            if let Some(mp) = player_lock.as_ref() {
                mp.set_audio_track(track_id);
            }
        }
    }
}

// Stub implementation when feature is disabled
#[cfg(not(feature = "vlc"))]
#[allow(dead_code)]
pub struct VlcPlayer;

#[cfg(not(feature = "vlc"))]
#[allow(dead_code)]
impl VlcPlayer {
    pub fn new() -> Option<Self> {
        None
    }
}
