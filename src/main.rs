#![no_std]
#![no_main]

mod proto;
mod utils;

extern crate alloc;
extern crate flipperzero_alloc;

use core::{
    cell::{Cell, RefCell},
    ffi::{c_char, c_void, CStr},
    mem, ptr,
    time::Duration,
};

use crate::utils::input::{InputEvent, InputKey, InputType};
use alloc::{ffi::CString, format, rc::Rc, string::String, sync::Arc};
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{iso_8859_13::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::{Circle, Primitive, PrimitiveStyle},
    text::{renderer::CharacterStyle, Alignment, Text},
    Drawable,
};
use flipperzero::{
    furi::{
        message_queue::MessageQueue,
        sync::{Mutex, MutexGuard},
        thread::sleep,
    },
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;
use serde_json_core::heapless::Vec;
use utils::{gui::Gui, viewport::ViewPort};

manifest!(
    name = "Hello, Rust!",
    has_icon = true,
    icon = "rustacean-10x10.icon"
);

entry!(main);

pub extern "C" fn input_callback(e: *mut sys::InputEvent, ctx: *mut c_void) {
    let mq: &MessageQueue<InputEvent> = unsafe { mem::transmute(ctx) };

    let _ = mq.put(InputEvent::from(e), Duration::from_secs(1));
}

struct HelloRust {
    pub rand: u32,
}
fn draw_callback(cv: &mut crate::utils::canvas::Canvas, app: MutexGuard<HelloRust>) {
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::with_alignment(
        "Hello World",
        Point::new(128 / 2, 64 / 2),
        text_style,
        Alignment::Center,
    )
    .draw(cv)
    .unwrap();
    Text::with_alignment(
        format!("{}", app.rand).as_str(),
        Point::new(128 / 2, 64 / 2 + 10),
        text_style,
        Alignment::Center,
    )
    .draw(cv)
    .unwrap();
}

fn main(_p: *mut u8) -> i32 {
    let mq: MessageQueue<InputEvent> = MessageQueue::new(8);
    let app = Arc::new(Mutex::new(HelloRust { rand: 0 }));

    let mut vp = ViewPort::new(app.clone());
    let mut gui = Gui::new();

    gui.add_viewport(&vp);

    vp.on_input(Some(input_callback), &mq);
    vp.on_draw(draw_callback);

    let mut processing: bool = true;
    while processing {
        let m = mq.get(Duration::from_millis(100));

        if let Ok(ie) = m {
            if let Ok(mut a) = app.lock() {
                match ie.get_type() {
                    InputType::Press => match ie.get_key() {
                        InputKey::Ok => {
                            unsafe {
                                a.rand = sys::furi_hal_random_get();
                            }
                            let mut v: Vec<u8, 32> =
                                serde_json_core::to_vec(&proto::Message::SetFreq(a.rand)).unwrap();

                            crate::utils::misc::send_over_uart(&mut v);

                            v = serde_json_core::to_vec(&proto::Message::Play).unwrap();

                            crate::utils::misc::send_over_uart(&mut v);
                        }
                        InputKey::Back => {
                            processing = false;
                        }
                        _ => {}
                    },

                    _ => {}
                }
            }
        }
        vp.update();
    }

    0
}
