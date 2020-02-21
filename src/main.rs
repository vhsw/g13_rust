
mod g13_consts;
mod g13_device;
mod g13_uinput;


extern crate serde_derive;
extern crate toml;
extern crate uinput;
extern crate clap;


use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::keyboard::Key;
use uinput::event::relative::Relative::Position;
use uinput::event::Event::{Controller, Relative};
use uinput::event::relative::Position::{X, Y};

use clap::App;
use clap::Arg;

use serde_derive::Deserialize;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
  g1: Option<Vec<String>>,
  g2: Option<Vec<String>>,
  g3: Option<Vec<String>>,
  g4: Option<Vec<String>>,
  g5: Option<Vec<String>>,
  g6: Option<Vec<String>>,
  g7: Option<Vec<String>>,
  g8: Option<Vec<String>>,
  g9: Option<Vec<String>>,
  g10: Option<Vec<String>>,
  g11: Option<Vec<String>>,
  g12: Option<Vec<String>>,
  g13: Option<Vec<String>>,
  g14: Option<Vec<String>>,
  g15: Option<Vec<String>>,
  g16: Option<Vec<String>>,
  g17: Option<Vec<String>>,
  g18: Option<Vec<String>>,
  g19: Option<Vec<String>>,
  g20: Option<Vec<String>>,
  g21: Option<Vec<String>>,
  g22: Option<Vec<String>>,
  m1: Option<Vec<String>>,
  m2: Option<Vec<String>>,
  m3: Option<Vec<String>>,
  mr: Option<Vec<String>>,
  d0: Option<Vec<String>>,
  d1: Option<Vec<String>>,
  d2: Option<Vec<String>>,
  d3: Option<Vec<String>>,
  d4: Option<Vec<String>>,
  bkl: Option<Vec<String>>,
  j1: Option<Vec<String>>,
  j2: Option<Vec<String>>,
  j3: Option<Vec<String>>,
}

struct G13Key{
  name: &'static str,
  byte: usize,
  mask: u8,
  action: Vec<Key>,
}

pub struct G13State {
  analog_state:  (i32, i32),
  kb_state: HashMap<Key, bool>,
  mouse_uinput: uinput::Device,
  kb_uinput:  uinput::Device,
  mapping: [G13Key; 35],
}

impl Default for G13State {
  fn default() -> G13State{
    #[rustfmt::skip]
    let kb = uinput::default().unwrap()
                        .name("g13_keys").unwrap()
                        .event(uinput::event::Keyboard::All).unwrap()
                        .create().unwrap();
    #[rustfmt::skip]
    let mouse = uinput::default().unwrap()
                           .name("g13_mouse").unwrap()
                           .event(Controller(Mouse(Left))).unwrap()
                           .event(Relative(Position(X))).unwrap()
                           .event(Relative(Position(Y))).unwrap()
                           .create().unwrap();


    #[rustfmt::skip]
    let mut buttons = [
      G13Key{name: "G1",  byte: 3, mask: 0b00000001, action: vec![]},
      G13Key{name: "G2",  byte: 3, mask: 0b00000010, action: vec![]},
      G13Key{name: "G3",  byte: 3, mask: 0b00000100, action: vec![]},
      G13Key{name: "G4",  byte: 3, mask: 0b00001000, action: vec![]},
      G13Key{name: "G5",  byte: 3, mask: 0b00010000, action: vec![]},
      G13Key{name: "G6",  byte: 3, mask: 0b00100000, action: vec![]},
      G13Key{name: "G7",  byte: 3, mask: 0b01000000, action: vec![]},
      G13Key{name: "G8",  byte: 3, mask: 0b10000000, action: vec![]},
      G13Key{name: "G9",  byte: 4, mask: 0b00000001, action: vec![]},
      G13Key{name: "G10", byte: 4, mask: 0b00000010, action: vec![]},
      G13Key{name: "G11", byte: 4, mask: 0b00000100, action: vec![]},
      G13Key{name: "G12", byte: 4, mask: 0b00001000, action: vec![]},
      G13Key{name: "G13", byte: 4, mask: 0b00010000, action: vec![]},
      G13Key{name: "G14", byte: 4, mask: 0b00100000, action: vec![]},
      G13Key{name: "G15", byte: 4, mask: 0b01000000, action: vec![]},
      G13Key{name: "G16", byte: 4, mask: 0b10000000, action: vec![]},
      G13Key{name: "G17", byte: 5, mask: 0b00000001, action: vec![]},
      G13Key{name: "G18", byte: 5, mask: 0b00000010, action: vec![]},
      G13Key{name: "G19", byte: 5, mask: 0b00000100, action: vec![]},
      G13Key{name: "G20", byte: 5, mask: 0b00001000, action: vec![]},
      G13Key{name: "G21", byte: 5, mask: 0b00010000, action: vec![]},
      G13Key{name: "G22", byte: 5, mask: 0b00100000, action: vec![]},
      G13Key{name: "D0",  byte: 6, mask: 0b00000001, action: vec![]},
      G13Key{name: "D1",  byte: 6, mask: 0b00000010, action: vec![]},
      G13Key{name: "D2",  byte: 6, mask: 0b00000100, action: vec![]},
      G13Key{name: "D3",  byte: 6, mask: 0b00001000, action: vec![]},
      G13Key{name: "D4",  byte: 6, mask: 0b00010000, action: vec![]},
      G13Key{name: "M1",  byte: 6, mask: 0b00100000, action: vec![]},
      G13Key{name: "M2",  byte: 6, mask: 0b01000000, action: vec![]},
      G13Key{name: "M3",  byte: 6, mask: 0b10000000, action: vec![]},
      G13Key{name: "MR",  byte: 7, mask: 0b00000001, action: vec![]},
      G13Key{name: "J1",  byte: 7, mask: 0b00000010, action: vec![]},
      G13Key{name: "J2",  byte: 7, mask: 0b00000100, action: vec![]},
      G13Key{name: "J3",  byte: 7, mask: 0b00001000, action: vec![]},
      G13Key{name: "BK",  byte: 7, mask: 0b00100000, action: vec![]},
    ];

    parse_config(g13_consts::DEFAULT_MAP.to_string(), &mut buttons);

    let kb_state = HashMap::<Key, bool>::from_iter(Key::iter_variants().map(|v| (v, false)));

    G13State{
      analog_state: (0, 0),
      kb_state: kb_state,
      mouse_uinput: mouse,
      kb_uinput: kb,
      mapping: buttons,
      }
  }
}

fn decode(s: &str) -> Result<Key, String> {
  match s {
    "esc" => Ok(Key::Esc),
    "1" => Ok(Key::_1),
    "2" => Ok(Key::_2),
    "3" => Ok(Key::_3),
    "4" => Ok(Key::_4),
    "5" => Ok(Key::_5),
    "6" => Ok(Key::_6),
    "7" => Ok(Key::_7),
    "8" => Ok(Key::_8),
    "9" => Ok(Key::_9),
    "0" => Ok(Key::_0),
    "-" => Ok(Key::Minus),
    "=" => Ok(Key::Equal),
    "backspace" => Ok(Key::BackSpace),
    "tab" => Ok(Key::Tab),
    "q" => Ok(Key::Q),
    "w" => Ok(Key::W),
    "e" => Ok(Key::E),
    "r" => Ok(Key::R),
    "t" => Ok(Key::T),
    "y" => Ok(Key::Y),
    "u" => Ok(Key::U),
    "i" => Ok(Key::I),
    "o" => Ok(Key::O),
    "p" => Ok(Key::P),
    "(" => Ok(Key::LeftBrace),
    "_" => Ok(Key::RightBrace),
    "enter" => Ok(Key::Enter),
    "lctrl" => Ok(Key::LeftControl),
    "a" => Ok(Key::A),
    "s" => Ok(Key::S),
    "d" => Ok(Key::D),
    "f" => Ok(Key::F),
    "g" => Ok(Key::G),
    "h" => Ok(Key::H),
    "j" => Ok(Key::J),
    "k" => Ok(Key::K),
    "l" => Ok(Key::L),
    ";" => Ok(Key::SemiColon),
    "'" => Ok(Key::Apostrophe),
    "`" => Ok(Key::Grave),
    "lshift" => Ok(Key::LeftShift),
    "\\" => Ok(Key::BackSlash),
    "z" => Ok(Key::Z),
    "x" => Ok(Key::X),
    "c" => Ok(Key::C),
    "v" => Ok(Key::V),
    "b" => Ok(Key::B),
    "n" => Ok(Key::N),
    "m" => Ok(Key::M),
    "," => Ok(Key::Comma),
    "." => Ok(Key::Dot),
    "/" => Ok(Key::Slash),
    "rshift" => Ok(Key::RightShift),
    "lalt" => Ok(Key::LeftAlt),
    "space" => Ok(Key::Space),
    "capslock" => Ok(Key::CapsLock),
    "f1" => Ok(Key::F1),
    "f2" => Ok(Key::F2),
    "f3" => Ok(Key::F3),
    "f4" => Ok(Key::F4),
    "f5" => Ok(Key::F5),
    "f6" => Ok(Key::F6),
    "f7" => Ok(Key::F7),
    "f8" => Ok(Key::F8),
    "f9" => Ok(Key::F9),
    "f10" => Ok(Key::F10),
    "numlock" => Ok(Key::NumLock),
    "scrolllock" => Ok(Key::ScrollLock),
    "f11" => Ok(Key::F11),
    "f12" => Ok(Key::F12),
    "rctrl" => Ok(Key::RightControl),
    "sysrq" => Ok(Key::SysRq),
    "ralt" => Ok(Key::RightAlt),
    "lf" => Ok(Key::LineFeed),
    "home" => Ok(Key::Home),
    "up" => Ok(Key::Up),
    "pgup" => Ok(Key::PageUp),
    "left" => Ok(Key::Left),
    "right" => Ok(Key::Right),
    "end" => Ok(Key::End),
    "down" => Ok(Key::Down),
    "pgdn" => Ok(Key::PageDown),
    "insert" => Ok(Key::Insert),
    "delete" => Ok(Key::Delete),
    "lmeta" => Ok(Key::LeftMeta),
    "rmeta" => Ok(Key::RightMeta),
    "scrollup" => Ok(Key::ScrollUp),
    "scrolldown" => Ok(Key::ScrollDown),
    "f13" => Ok(Key::F13),
    "f14" => Ok(Key::F14),
    "f15" => Ok(Key::F15),
    "f16" => Ok(Key::F16),
    "f17" => Ok(Key::F17),
    "f18" => Ok(Key::F18),
    "f19" => Ok(Key::F19),
    "f20" => Ok(Key::F20),
    "f21" => Ok(Key::F21),
    "f22" => Ok(Key::F22),
    "f23" => Ok(Key::F23),
    "f24" => Ok(Key::F24),
    _ => Err(format!{"Unknown value in config: {}", s}),
  }
}

fn parse_config(config_str: String, buttons: &mut [G13Key; 35]){
  let config: HashMap<String, Vec<String>> = toml::from_str(&config_str).unwrap();
      for button in buttons.iter_mut() {
        match config.get(button.name) {
          Some(actions) => {
            button.action = actions.iter().map(|a| decode(a).expect("wrong value for key {} in config")).collect()
        },
          None => continue,
        }
    }
}

fn main(){
  let vid: u16 = g13_consts::VENDOR_ID;
  let pid: u16 = g13_consts::PRODUCT_ID;

  let mut device: G13State = G13State::default();

  #[rustfmt::skip]
  let matches = App::new("G13 Keyboard Driver")
                .version("0.1.0")
                .author("Alexey Dubrov <nevermind1025@gmail.com>")
                .about("Maps USB data to /dev/input")
                .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                .get_matches();



  match matches.value_of("config") {
    Some(config) => {
        let mut file = File::open(config).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        parse_config(contents, &mut device.mapping);
    },
    None => (),
  }
  g13_device::connect(vid, pid, &mut device);
}
