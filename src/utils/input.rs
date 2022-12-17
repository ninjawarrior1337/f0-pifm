use flipperzero_sys as sys;

#[derive(PartialEq)]
pub enum InputKey {
    Ok,
    Back,
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

impl From<u8> for InputKey {
    fn from(value: u8) -> Self {
        match value {
            sys::InputKey_InputKeyOk => Self::Ok,
            sys::InputKey_InputKeyBack => Self::Back,
            sys::InputKey_InputKeyDown => Self::Down,
            sys::InputKey_InputKeyUp => Self::Up,
            sys::InputKey_InputKeyLeft => Self::Left,
            sys::InputKey_InputKeyRight => Self::Right,

            _ => Self::Unknown,
        }
    }
}

#[derive(PartialEq)]
pub enum InputType {
    Long,
    Press,
    Release,
    Repeat,
    Short,
    Unknown,
}

impl From<u8> for InputType {
    fn from(value: u8) -> Self {
        match value {
            sys::InputType_InputTypeLong => Self::Long,
            sys::InputType_InputTypePress => Self::Press,
            sys::InputType_InputTypeRelease => Self::Release,
            sys::InputType_InputTypeRepeat => Self::Repeat,
            sys::InputType_InputTypeShort => Self::Short,
            _ => Self::Unknown,
        }
    }
}

pub struct InputEvent {
    raw: *mut sys::InputEvent,
}

impl From<*mut sys::InputEvent> for InputEvent {
    fn from(value: *mut sys::InputEvent) -> Self {
        InputEvent { raw: value }
    }
}

impl InputEvent {
    pub fn get_key(&self) -> InputKey {
        unsafe { (*self.raw).key.into() }
    }

    pub fn get_type(&self) -> InputType {
        unsafe { (*self.raw).type_.into() }
    }
}