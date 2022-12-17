use core::{ffi::c_void, mem};

use flipperzero_sys as sys;

use alloc::{sync::Arc, boxed::Box};
use flipperzero::furi::{sync::{Mutex, MutexGuard}, message_queue::MessageQueue};
use sys::ViewPortInputCallback;

use super::canvas::Canvas;

type DrawCallback<C> = dyn Fn(&mut Canvas, MutexGuard<C>) -> ();
type BoxedDrawCallback<C> = Box<DrawCallback<C>>;

pub struct ViewPort<C> {
    vp: *mut sys::ViewPort,
    ac: Arc<Mutex<C>>,
    draw_callback: Option<BoxedDrawCallback<C>>,
    input_callback: Option<Box<dyn Fn() -> ()>>,
}

impl<C> ViewPort<C> {
    pub fn new(ac: Arc<Mutex<C>>) -> Self {
        unsafe {
            let vp: ViewPort<C> = ViewPort {
                vp: sys::view_port_alloc(),
                ac: ac,
                draw_callback: None,
                input_callback: None,
            };
            // sys::view_port_draw_callback_set(self.raw_viewport(), f, mem::transmute(c));
            vp
        }
    }

    pub fn raw_viewport(&self) -> *mut sys::ViewPort {
        self.vp
    }

    pub fn update(&self) {
        unsafe {
            sys::view_port_update(self.raw_viewport());
        }
    }

    pub fn on_input<M>(&self, f: ViewPortInputCallback, mq: &MessageQueue<M>) {
        unsafe { sys::view_port_input_callback_set(self.raw_viewport(), f, mem::transmute(mq)) }
    }

    pub fn on_draw(&mut self, f: impl Fn(&mut Canvas, MutexGuard<C>) -> () + 'static) {
        self.draw_callback = Some(Box::new(f));
        unsafe {
            sys::view_port_draw_callback_set(
                self.raw_viewport(),
                Some(ViewPort::<C>::_raw_draw_callback),
                mem::transmute(self),
            );
        }
    }

    pub unsafe extern "C" fn _raw_draw_callback(c: *mut sys::Canvas, ctx: *mut c_void) {
        let cx: &mut ViewPort<C> = mem::transmute(ctx);
        if let Some(dcb) = &cx.draw_callback {
            if let Ok(ac) = cx.ac.clone().lock() {
                (dcb)(&mut Canvas::from(c), ac);
            }
        }
    }
}

impl<C> Drop for ViewPort<C> {
    fn drop(&mut self) {
        unsafe {
            sys::view_port_free(self.raw_viewport());
        }
    }
}