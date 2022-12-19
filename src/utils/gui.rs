use core::{ffi::c_char};

use flipperzero_sys as sys;

pub struct GuiHandle {
    gui: *mut sys::Gui,
}

impl GuiHandle {
    pub const RECORD_GUI: *const c_char = sys::c_string!("gui");
    pub const FULLSCREEN: sys::GuiLayer = sys::GuiLayer_GuiLayerFullscreen;

    pub fn new() -> Self {
        unsafe {
            let gui = sys::furi_record_open(Self::RECORD_GUI) as *mut sys::Gui;
            GuiHandle { gui }
        }
    }

    pub(super) fn inner_gui(&self) -> *mut sys::Gui {
        self.gui
    }
}

impl Drop for GuiHandle {
    fn drop(&mut self) {
        unsafe {
            sys::furi_record_close(Self::RECORD_GUI);
        }
    }
}
