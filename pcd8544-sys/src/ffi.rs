
use libc::*;

pub const WHITE: uint8_t = 0;
pub const BLACK: uint8_t = 1;

extern {
    pub fn LCDInit(sclk: uint8_t, din: uint8_t, dc: uint8_t, cs: uint8_t, rst: uint8_t, contrast: uint8_t);
    pub fn LCDsetPixel(x: uint8_t, y: uint8_t, color: uint8_t);
    pub fn LCDclear();
    pub fn LCDdisplay();
}
