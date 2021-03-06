use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Key};
use enigo::KeyboardControllable;
use crate::Config::Config;

fn set_clipboard(content: &str) {
    terminal_clipboard::set_string(content).unwrap();
}
fn send_enter_key(keyboard: &mut Enigo) {
    keyboard.key_down(Key::Return);
    keyboard.key_up(Key::Return);
}
fn send_enter_key_delay(keyboard: &mut Enigo, delay: Duration) {
    send_enter_key(keyboard);
    sleep(delay);
}
fn send_paste_clipboard(keyboard: &mut Enigo) {
    keyboard.key_sequence_parse("{+CTRL}v{-CTRL}");
}
fn send_letter(keyboard: &mut Enigo, letter: char) {
    keyboard.key_sequence_parse(&*format!("{}", letter));
}
fn edit_last_message(content: &str, keyboard: &mut Enigo) {
    keyboard.key_down(Key::UpArrow);
    keyboard.key_up(Key::UpArrow);
    set_clipboard(content);
    send_enter_key(keyboard);
}


pub trait Sender {
    fn send(&mut self, msg: String);
}



pub struct KBWriter {
    pub keyboard: Enigo,
    pub cfg: crate::Config::Config,
}
impl Sender for KBWriter {
    fn send(&mut self, msg: String) {
        self.write_slowly(msg);
        if self.cfg.is_autosend {
            send_enter_key(&mut self.keyboard);
        }
    }
}
impl KBWriter {
    pub fn new(cfg: crate::Config::Config, keyboard: Enigo) -> Self{
        Self {
            cfg,
            keyboard
        }
    }
    pub fn write_slowly(&mut self, msg: String) {
        for letter in msg.chars(){
            send_letter(&mut self.keyboard, letter);
            sleep(Duration::from_millis(self.cfg.speed as u64));
        }
    }
}



pub struct ClipboardWriter {
    pub keyboard: Enigo,
    pub cfg: crate::Config::Config,
}
impl Sender for ClipboardWriter {
    fn send(&mut self, msg: String) {
        set_clipboard(msg.as_str());
        send_paste_clipboard(&mut self.keyboard);
        if self.cfg.is_autosend {
            send_enter_key(&mut self.keyboard);
        }
    }
}
impl ClipboardWriter {
    pub fn new(cfg: crate::Config::Config, keyboard: Enigo) -> Self{
        Self {
            cfg,
            keyboard
        }
    }
}


