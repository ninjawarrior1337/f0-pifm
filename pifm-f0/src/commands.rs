use alloc::{borrow::ToOwned, vec::Vec};
use itoa::Buffer;
use arrform::{arrform, ArrForm};
pub enum Command {
    Play,
    Stop,
    SetFreq(u32),
    SetSong(u32),
    GetSongs,
    Exit
}

impl Command {
    pub fn raw_data<'a>(self) -> Vec<u8> {
        let rc: RawCommand = self.into();

        let b = rc.0.as_bytes().to_owned();

        b
    }
}

struct RawCommand(pub ArrForm::<32>);

impl From<Command> for RawCommand {
    fn from(value: Command) -> Self {
        let v = match value {
            Command::Play => arrform!(32, "play"),
            Command::Stop => arrform!(32, "stop"),
            Command::SetFreq(f) => arrform!(32, "set freq {}", f),
            Command::SetSong(idx) => arrform!(32, "set song {}", idx),
            Command::GetSongs => arrform!(32, "get songs"),
            Command::Exit => arrform!(32, "exit"),
        };

        RawCommand(v)
    }
}