use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct GameInfo {
    game_title: [u8; 12],
    game_code: [u8; 4],
    maker_code: [u8; 2],
    fixed_value: [u8; 1], // Must be 96h.
    unit_code: [u8; 1],
    device_type: [u8; 1],
    reserved_area1: [u8; 7],
    version_number: [u8; 1],
    check_sum: [u8; 1],
    reserved_area2: [u8; 2],
}

impl GameInfo {
    fn new() -> GameInfo {
        let mut game_info = GameInfo {
            game_title: [0; 12],
            game_code: [0; 4],
            maker_code: [0; 2],
            fixed_value: [0x96],
            unit_code: [0],
            device_type: [0],
            reserved_area1: [0; 7],
            version_number: [0],
            check_sum: [0],
            reserved_area2: [0; 2],
        };
        game_info.check_sum = [game_info.calc_check_sum()];
        game_info
    }

    fn calc_check_sum(&mut self) -> u8 {
        let mut chk: u8 = 0;
        for i in &self.game_title {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.game_code {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.maker_code {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.fixed_value {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.unit_code {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.device_type {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.reserved_area1 {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        for i in &self.version_number {
            chk = chk - i;
            chk=(chk-0x19) & 0xFF;
        }
        chk
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("game_info.rs");
    let mut f = File::create(&dest_path).unwrap();

    let game_info = GameInfo::new();

    let encoded: Vec<u8> = bincode::serialize(&game_info).unwrap();

    let s = format!("{:?}", encoded);
    let s = s.as_str();
    let slice = &s[1..s.len()-1];

    f.write_all(format!(r#"asm!(".byte {}")"#, slice).as_bytes()).unwrap();
}
