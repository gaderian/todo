#![allow(dead_code)]
pub static RED: &'static str = "\x1b[0;31m";
pub static GREY: &'static str = "\x1b[1;30m";
pub static BLUE: &'static str = "\x1b[0;34m";
pub static NONE: &'static str = "\x1b[0m";
pub static GREEN: &'static str = "\x1b[0;32m";
pub static CYAN: &'static str = "\x1b[0;36m";

pub trait Colorizer<T> {
    fn color(&self, s: T) -> String;
}
