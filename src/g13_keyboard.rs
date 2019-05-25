use uinput::event::keyboard;

pub fn create_device() -> uinput::Device{
  uinput::default().unwrap()
        .name("g13_keys").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap()
}

pub fn parse_report(report: [u8; ::g13_consts::REPORT_SIZE], device: &mut uinput::Device) {
    for num in &report {
        print!("{:08b} ", num);
    }
    println!();

	device.click(&keyboard::Key::F).unwrap();
	device.synchronize().unwrap();
}
