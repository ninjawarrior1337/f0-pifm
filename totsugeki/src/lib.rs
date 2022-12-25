#![no_std]

pub mod canvas;
pub mod gui;
pub mod input;
pub mod str;
pub mod viewport;

extern crate alloc;
extern crate flipperzero_alloc;

pub mod misc {
    use flipperzero_sys as sys;

    pub fn send_over_uart(v: &mut [u8]) {
        let mut ln  = ['\n' as u8];
        unsafe {
            sys::furi_hal_uart_init(sys::FuriHalUartId_FuriHalUartIdLPUART1, 115200);
            sys::furi_hal_uart_tx(
                sys::FuriHalUartId_FuriHalUartIdLPUART1,
                v.as_mut_ptr(),
                v.len(),
            );
            sys::furi_hal_uart_tx(
                sys::FuriHalUartId_FuriHalUartIdLPUART1,
                ln.as_mut_ptr(),
                ln.len(),
            );
            sys::furi_delay_ms(100);
            sys::furi_hal_uart_deinit(sys::FuriHalUartId_FuriHalUartIdLPUART1)
        }
    }    
}