use core::ffi::c_char;

use flipperzero_sys as sys;

use super::viewport::ViewPort;

pub struct Gui {
    gui: *mut sys::Gui,
    vp: Option<*mut sys::ViewPort>
}

impl Gui {
    const RECORD_GUI: *const c_char = sys::c_string!("gui");
    const FULLSCREEN: sys::GuiLayer = sys::GuiLayer_GuiLayerFullscreen;

    pub fn new() -> Self {
        unsafe {
            let gui = sys::furi_record_open(Self::RECORD_GUI) as *mut sys::Gui;
            Gui {
                gui,
                vp: None
            }
        }
    }

    fn inner_gui(&self) -> *mut sys::Gui {
        self.gui
    }

    pub fn add_viewport<T>(&mut self, vp: &ViewPort<T>) {
        unsafe {
            sys::gui_add_view_port(self.inner_gui(), vp.raw_viewport(), Self::FULLSCREEN);
        }
        self.vp = Some(vp.raw_viewport());
    }
}

impl Drop for Gui {
    fn drop(&mut self) {
        unsafe {
            if let Some(vp) = self.vp {
                sys::view_port_enabled_set(vp, false);
                sys::gui_remove_view_port(self.gui, vp);
            }
            sys::furi_record_close(Self::RECORD_GUI);
        }
    }
}