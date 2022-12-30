#[cfg(feature = "embedded-graphics")]
use embedded_graphics::{prelude::{DrawTarget, OriginDimensions, Size, Point}, pixelcolor::BinaryColor, Pixel, primitives::{rectangle, Rectangle}};

use flipperzero_sys as sys;

pub struct Canvas(pub *mut sys::Canvas);

impl Canvas {
    pub fn width(&self) -> u8 {
        unsafe {
            sys::canvas_width(self.0)
        }
    }

    pub fn height(&self) -> u8 {
        unsafe {
            sys::canvas_height(self.0)
        }
    }
}

#[cfg(feature = "embedded-graphics")]
impl Canvas {
    pub fn bounding_box(&self) -> Rectangle {
        rectangle::Rectangle::new(Point::zero(), self.size())
    }
}

impl From<*mut sys::Canvas> for Canvas {
    fn from(value: *mut sys::Canvas) -> Self {
        Canvas(value)
    }
}

#[cfg(feature = "embedded-graphics")]
impl DrawTarget for Canvas {
    type Color = embedded_graphics::pixelcolor::BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(cord, color) in pixels.into_iter() {
            unsafe {
                match color {
                    BinaryColor::Off => sys::canvas_set_color(self.0, sys::Color_ColorWhite),
                    BinaryColor::On => sys::canvas_set_color(self.0, sys::Color_ColorBlack),
                }
                sys::canvas_draw_dot(self.0, cord.x as u8, cord.y as u8);
            }
        }

        Ok(())
    }
}

#[cfg(feature = "embedded-graphics")]
impl OriginDimensions for Canvas {
    fn size(&self) -> embedded_graphics::prelude::Size {
        unsafe {
            Size::new(
                self.width().into(),
                self.height().into(),
            )
        }
    }
}