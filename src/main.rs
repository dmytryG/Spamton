// sudo apt install xorg-dev libxcb-composite0-dev libxdo-dev for linux

use std::thread::sleep;
use std::time::Duration;
use enigo::Enigo;
use rand::Rng;
use crate::Autosender::{ClipboardWriter, KBWriter, Sender};

pub mod Autosender;
pub mod Config;

fn main() {
    let cfg = Config::Config::new();
    println!("{:#?}", cfg);

    //let mut writer = crate::Autosender::KBWriter::new(cfg.clone(), Enigo::new());
    let mut writer: Box<dyn Sender>;

    match std::fs::read_to_string(&cfg.source) {
        Ok(file) => {
            let file: Vec<&str> = file.lines().collect();
            println!("Source loaded successfully: ");
            for line in file.clone() {
                println!("> {}", line);
            }

            if cfg.use_clipboard {
                writer = Box::new(ClipboardWriter::new(cfg.clone(), Enigo::new()));
            } else {
                writer = Box::new(KBWriter::new(cfg.clone(), Enigo::new()));
            }

            for i in 0..5 {
                println!("Start in {}...", 5-i);
                sleep(Duration::from_secs(1));
            }
            println!("GO!");


            loop {
                let line = file[rand::thread_rng().gen_range(0..file.len())].to_string().clone();
                writer.send(line.clone());
                println!("#> {}", line);
                sleep(Duration::from_millis(cfg.delayment as u64));
            }

        }
        Err(err) => {
            println!("Can't open source file {}. Error: {}", cfg.source, err);
        }
    }

}
