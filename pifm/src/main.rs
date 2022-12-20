#![no_std]
#![no_main]

extern crate alloc;
extern crate flipperzero_alloc;

use core::{time::Duration};

use alloc::{rc::Rc, sync::Arc};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Text}, Drawable,
};
use embedded_layout::{
    layout::linear::{spacing, LinearLayout},
};
use flipperzero::furi::{
    message_queue::MessageQueue,
    sync::{Mutex, MutexGuard},
};

use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

use statig::{StateMachine, Response::{self, Transition}, StateMachineSharedStorage, InitializedStatemachine};

use prost::Message;
use totsugeki::{canvas::Canvas, input::{InputEvent, InputType, InputKey}, viewport::ViewPort, gui::GuiHandle};

pub mod pifm {
    include!(concat!(env!("OUT_DIR"), "/pifm.proto.rs"));
}

manifest!(
    name = "Hello, Rust!",
    has_icon = true,
    icon = "rustacean-10x10.icon"
);

entry!(main);

fn draw_callback(cv: &mut Canvas, app: MutexGuard<InitializedStatemachine<AppState>>) {
    use embedded_layout::prelude::*;

    let mut text_style_sel = MonoTextStyle::new(&FONT_6X9, BinaryColor::Off);
    text_style_sel.background_color = Some(BinaryColor::On);
    let mut text_style_desel = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    text_style_desel.background_color = Some(BinaryColor::Off);

    let mut ui = match app.state() {
        State::Start {  } => {
            alloc::vec![
                Text::new("Start", Point::zero(), text_style_sel),
                Text::new("Stop", Point::zero(), text_style_desel),
            ]
        },
        State::Stop {  } => {
            alloc::vec![
                Text::new("Start", Point::zero(), text_style_desel),
                Text::new("Stop", Point::zero(), text_style_sel),
            ]
        },
    };

    LinearLayout::vertical(Views::new(&mut ui))
        .with_alignment(horizontal::Center)
        .with_spacing(spacing::FixedMargin(16))
        .arrange()
        .align_to(&cv.bounding_box(), horizontal::Center, vertical::Center)
        .draw(cv)
        .unwrap()
}

#[derive(Default)]
pub struct AppState {
    rand: u32,
    home_selected: u8
}

pub struct Event;

#[statig::state_machine(initial = "State::start()", state(derive(Debug)))]
impl AppState {
    #[state]
    fn start(&mut self, event: &Event) -> Response<State> {
        self.home_selected = (self.home_selected+1)%2;
        Transition(State::stop())
    }

    #[state]
    fn stop(&mut self, event: &Event) -> Response<State> {
        self.home_selected = (self.home_selected+1)%2;
        Transition(State::start())
    }
}

fn main(_p: *mut u8) -> i32 {
    let app_state = AppState::default().state_machine().init();
    let app = Arc::new(Mutex::new(app_state));

    let mq: Rc<MessageQueue<InputEvent>> = Rc::new(MessageQueue::new(8));
    let mut vp = ViewPort::new();
    let gui = GuiHandle::new();

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

    loop {
        let m = mq.get(Duration::from_millis(100));

        if let Ok(ie) = m {
            if let Ok(mut state) = app.lock() {
                match ie.get_type() {
                    InputType::Press => match ie.get_key() {
                        InputKey::Ok => {
                            unsafe {
                                state.rand = sys::furi_hal_random_get();
                            }
                            let mut sf = pifm::SetFrequency::default();
                            sf.freq = state.rand;

                            totsugeki::misc::send_over_uart(&mut sf.encode_length_delimited_to_vec());
                        }
                        InputKey::Up | InputKey::Down => {
                            state.handle(&Event)
                        },
                        InputKey::Back => break,
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
