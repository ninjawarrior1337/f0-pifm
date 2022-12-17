#![no_std]
#![no_main]

mod utils;

extern crate alloc;
extern crate flipperzero_alloc;

use core::{
    ffi::{c_void},
    mem,
    time::Duration,
};

use crate::utils::input::{InputEvent, InputKey, InputType};
use alloc::{format, sync::Arc, vec::Vec};

use embedded_graphics::{
    mono_font::{iso_8859_13::FONT_6X10, MonoTextStyle, iso_8859_4::FONT_6X9},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Alignment, Text},
    Drawable,
};
use embedded_layout::{layout::linear::{LinearLayout, ElementSpacing, spacing}, prelude::Chain, view_group::Views};
use flipperzero::{
    furi::{
        message_queue::MessageQueue,
        sync::{Mutex, MutexGuard},
    },
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;
use utils::{gui::Gui, viewport::ViewPort};

use prost::Message;

pub mod pifm {
    include!(concat!(env!("OUT_DIR"), "/pifm.proto.rs"));
}

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
    use embedded_layout::prelude::*;

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let text = Text::new("Embeded Layout", Point::zero(), text_style);

    let rand_str = format!("Rand: {}", app.rand);
    let rand_disp = Text::new(rand_str.as_str(), Point::zero(), text_style);

    LinearLayout::vertical(
        Views::new(&mut [
            text,
            rand_disp
        ])
    )
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&cv.bounding_box(), horizontal::Center, vertical::Center)
    .with_spacing(spacing::FixedMargin(3))
    .draw(cv)
    .unwrap()
}

fn main(_p: *mut u8) -> i32 {
    let mq: MessageQueue<InputEvent> = MessageQueue::new(8);
    let app = Arc::new(Mutex::new(HelloRust { rand: 0 }));

    let mut vp = ViewPort::new(app.clone());
    let mut gui = Gui::new();

    gui.add_viewport(&vp);

    vp.on_input(Some(input_callback), &mq);
    vp.on_draw(draw_callback);

    'main_loop: loop {
        let m = mq.get(Duration::from_millis(100));

        if let Ok(ie) = m {
            if let Ok(mut a) = app.lock() {
                match ie.get_type() {
                    InputType::Press => match ie.get_key() {
                        InputKey::Ok => {
                            unsafe {
                                a.rand = sys::furi_hal_random_get();
                            }
                            let mut sf = pifm::SetFrequency::default();
                            sf.freq = a.rand;
                            
                            utils::misc::send_over_uart(&mut sf.encode_length_delimited_to_vec());
                        }
                        InputKey::Back => {
                            break 'main_loop
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
