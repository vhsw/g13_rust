
mod g13_consts;
mod g13_device;
mod g13_keyboard;
extern crate uinput;


fn main() {
    let vid: u16 = g13_consts::VENDOR_ID;
    let pid: u16 = g13_consts::PRODUCT_ID;

    g13_device::connect(vid, pid);
}
