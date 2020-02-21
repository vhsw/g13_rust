
use std::collections::HashSet;
use uinput::event::keyboard::Key;
use uinput::event::relative::Position::{X, Y};
type G13Report = [u8; ::g13_consts::REPORT_SIZE];

pub fn process_report(report: G13Report, state: &mut ::G13State) {

  let analog_x = (report[1] as i32 - 128) / 16;
  let analog_y = (report[2] as i32 - 128) / 16;
  parse_analog(analog_x, analog_y, state);
  parse_keys(report, state);

}

fn parse_analog(x: i32, y: i32, device: &mut ::G13State) {
  println! {"x = {}, y = {}", x, y};
  if x.abs() > device.analog_state.0 {
    device.mouse_uinput.send(X, x).unwrap();
  }
  if y.abs() > device.analog_state.1 {
    device.mouse_uinput.send(Y, y).unwrap();
  }
  device.mouse_uinput.synchronize().unwrap();
  device.analog_state = (x.abs(), y.abs());
}

fn parse_keys(report: G13Report, state: &mut ::G13State) {
  let mut pressed = HashSet::<Key>::new();
  let mut released = HashSet::<Key>::new();

  for key in state.mapping.iter() {
    let is_pressed = report[key.byte] & key.mask == key.mask;
    match is_pressed {
      true => pressed.extend(key.action.iter()),
      false => released.extend(key.action.iter()),
    }
  }

  released = &released - &pressed;

  for key in &released {
    let mut key_pressed = state.kb_state.get_mut(key).unwrap();
    if *key_pressed == true {
      *key_pressed = false;
      state.kb_uinput.release(key).unwrap();
    }
  }
  for key in &pressed {
    let mut key_pressed = state.kb_state.get_mut(key).unwrap();
    if *key_pressed == false {
      *key_pressed = true;
      state.kb_uinput.press(key).unwrap();
    }
  }
  state.kb_uinput.synchronize().unwrap();

}
