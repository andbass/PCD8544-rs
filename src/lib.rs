
extern crate pcd8544_sys;
extern crate wiringpi;

use wiringpi::WiringPi;
use wiringpi::pin::Pin;

// We take a WiringPi as a parameter to ensure that wiringpi was setup prior to this function call
#[allow(unused_variables)]
pub fn init<P: Pin>(pi: &WiringPi<P>, sclk: u8, din: u8, dc: u8, cs: u8, rst: u8, contrast: u8) {
    unsafe {
        pcd8544_sys::LCDInit(sclk, din, dc, cs, rst, contrast);
    }
}

pub fn set_pixel(x: u8, y: u8, on: bool) {
    unsafe {
        pcd8544_sys::LCDsetPixel(x, y, if on { pcd8544_sys::WHITE } else { pcd8544_sys::BLACK } );
    }
}

pub fn display() {
    unsafe {
        pcd8544_sys::LCDdisplay();
    }
}

pub fn clear() {
    unsafe {
        pcd8544_sys::LCDclear();
    }
}
