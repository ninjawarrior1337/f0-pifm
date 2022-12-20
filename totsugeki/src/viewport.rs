use core::{ffi::c_void, mem};

use flipperzero_sys as sys;

use alloc::{boxed::Box};


use super::{canvas::Canvas, gui::GuiHandle, input::InputEvent};

type DrawCallback = dyn Fn(&mut Canvas) -> ();
type BoxedDrawCallback = Box<DrawCallback>;

type InputCallback = dyn Fn(InputEvent) -> ();
type BoxedInputCallback = Box<InputCallback>;

pub struct ViewPort {
    vp: *mut sys::ViewPort,
    gui_handle: Option<GuiHandle>,
    draw_callback: Option<BoxedDrawCallback>,
    input_callback: Option<BoxedInputCallback>,
}

impl ViewPort {
    pub fn new() -> Self {
        unsafe {
            let vp: ViewPort = ViewPort {
                vp: sys::view_port_alloc(),
                gui_handle: None,
                draw_callback: None,
                input_callback: None,
            };
            vp
        }
    }

    pub(super) fn raw_viewport(&self) -> *mut sys::ViewPort {
        self.vp
    }

    pub fn update(&self) {
        unsafe {
            sys::view_port_update(self.raw_viewport());
        }
    }

    pub fn attach_to_gui(&mut self, gui_handle: GuiHandle) {
        unsafe {
            sys::gui_add_view_port(
                gui_handle.inner_gui(),
                self.raw_viewport(),
                GuiHandle::FULLSCREEN,
            );
        }
        self.gui_handle = Some(gui_handle);
    }

    pub fn on_input(&mut self, f: impl Fn(InputEvent) + 'static) {
        self.input_callback = Some(Box::new(f));
        unsafe {
            sys::view_port_input_callback_set(
                self.raw_viewport(),
                Some(Self::_raw_input_callback),
                mem::transmute(&self.input_callback),
            )
        }
    }

    pub unsafe extern "C" fn _raw_input_callback(ie: *mut sys::InputEvent, ctx: *mut c_void) {
        let f: &Option<BoxedInputCallback> = mem::transmute(ctx);
        if let Some(dcb) = f {
            (dcb)(ie.into());
        }
    }

    pub fn on_draw(&mut self, f: impl Fn(&mut Canvas) + 'static + Sync) {
        self.draw_callback = Some(Box::new(f));
        unsafe {
            sys::view_port_draw_callback_set(
                self.raw_viewport(),
                Some(ViewPort::_raw_draw_callback),
                mem::transmute(&self.draw_callback),
            );
        }
    }

    pub unsafe extern "C" fn _raw_draw_callback(c: *mut sys::Canvas, ctx: *mut c_void) {
        let f: &Option<BoxedDrawCallback> = mem::transmute(ctx);
        if let Some(dcb) = f {
            (dcb)(&mut c.into());
        }
    }
}

impl Drop for ViewPort {
    fn drop(&mut self) {
        unsafe {
            if let Some(gui) = &self.gui_handle {
                sys::view_port_enabled_set(self.raw_viewport(), false);
                sys::gui_remove_view_port(gui.inner_gui(), self.raw_viewport());
            }
            sys::view_port_free(self.raw_viewport());
        }
    }
}
