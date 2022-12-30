use alloc::{borrow::ToOwned, vec::Vec, string::String};
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
    pub fn raw_data(self) -> Vec<u8> {
        let rc: RawCommand = self.into();

        let b = rc.0.as_bytes().to_owned();

        b
    }
}

struct RawCommand(pub String);

impl From<Command> for RawCommand {
    fn from(value: Command) -> Self {
        let v = match value {
            Command::Play => "play".to_owned(),
            Command::Stop => "stop".to_owned(),
            Command::SetFreq(f) => {
                let mut s = "set freq ".to_owned();
                s.push_str(Buffer::new().format(f));
                s
            },
            Command::SetSong(idx) => {
                let mut s = "set song ".to_owned();
                s.push_str(Buffer::new().format(idx));
                s
            },
            Command::GetSongs => "get songs".to_owned(),
            Command::Exit => "exit".to_owned(),
        };

        RawCommand(v)
    }
}