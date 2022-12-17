use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    Play,
    Stop,
    SetSong(u32),
    SetFreq(u32)
}