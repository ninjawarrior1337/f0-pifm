#![no_std]
#![no_main]

mod utils;

extern crate alloc;
extern crate flipperzero_alloc;

use core::{borrow::BorrowMut, ffi::c_void, mem, time::Duration, cell::RefCell};

use crate::utils::input::{InputEvent, InputKey, InputType};
use alloc::{format, rc::Rc, sync::Arc, vec::Vec};

use embedded_graphics::{
    mono_font::{iso_8859_13::FONT_6X10, iso_8859_4::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Alignment, Text},
    Drawable,
};
use embedded_layout::{
    layout::linear::{spacing, ElementSpacing, LinearLayout},
    prelude::Chain,
    view_group::Views,
};
use flipperzero::furi::{
    message_queue::MessageQueue,
    sync::{Mutex, MutexGuard},
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;
use utils::{gui::GuiHandle, viewport::ViewPort};

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

fn draw_callback(cv: &mut crate::utils::canvas::Canvas, app: MutexGuard<HelloRust>) {
    use embedded_layout::prelude::*;

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let text = Text::new("Embeded Layout", Point::zero(), text_style);

    let rand_str = format!("Rand: {}", app.rand);
    let rand_disp = Text::new(rand_str.as_str(), Point::zero(), text_style);

    LinearLayout::vertical(Views::new(&mut [text, rand_disp]))
        .with_alignment(horizontal::Center)
        .arrange()
        .align_to(&cv.bounding_box(), horizontal::Center, vertical::Center)
        .with_spacing(spacing::FixedMargin(3))
        .draw(cv)
        .unwrap()
}

struct HelloRust {
    pub rand: u32,
}

fn main(_p: *mut u8) -> i32 {
    let mq: Rc<MessageQueue<InputEvent>> = Rc::new(MessageQueue::new(8));
    let app = Arc::new(Mutex::new(HelloRust { rand: 0 }));

    let mut vp = ViewPort::new();
    let gui = GuiHandle::new();

    // let vp_cell = RefCell::new(vp);

    vp.attach_to_gui(gui);

    let mq_input = mq.clone();
    vp.on_input(move |e| {
        let _ = mq_input.put(e, Duration::from_millis(u64::MAX));
    });

    let draw_app = app.clone();
    vp.on_draw(move |cv| {
        let a_lock = draw_app.lock();
        if let Ok(a) = a_lock {
            draw_callback(cv, a)
        }
    });

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
                        InputKey::Back => break 'main_loop,
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
