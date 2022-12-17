pub mod canvas;
pub mod gui;
pub mod input;
pub mod str;
pub mod viewport;

pub mod misc {
    use alloc::{vec::Vec, string::String};
    use flipperzero_sys as sys;

    pub fn send_over_uart(v: &mut Vec<u8>) {
        let mut ln: String = String::from("\n");
        unsafe {
            sys::furi_hal_uart_init(sys::FuriHalUartId_FuriHalUartIdLPUART1, 9600);
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