use std::borrow::Borrow;
use dotenv::dotenv;
use std::env;



#[derive(Clone, Debug)]
pub struct Config {
    pub use_clipboard: bool,
    pub source: String,
    pub delayment: u16,
    pub speed: u16,
    pub is_autosend: bool
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            use_clipboard: dotenv::var("use_clipboard").unwrap().parse::<bool>().unwrap(),
            source: dotenv::var("source").unwrap(),
            delayment: dotenv::var("delayment").unwrap().parse::<u16>().unwrap(),
            speed: dotenv::var("speed").unwrap().parse::<u16>().unwrap(),
            is_autosend: dotenv::var("is_autosend").unwrap().parse::<bool>().unwrap(),
        }
    }
}