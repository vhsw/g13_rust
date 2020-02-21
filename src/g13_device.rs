extern crate libusb;

use g13_uinput;
use std::time::Duration;

struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

pub fn connect(vid: u16, pid: u16, g13_state: &mut ::G13State) {
    match libusb::Context::new() {
        Ok(mut context) => match open_device(&mut context, vid, pid) {
            // Some((mut device, device_desc, mut handle)) => {
            //     read_device(&mut device, &device_desc, &mut handle).unwrap()
            // }
            Some((_, _, mut handle)) => read_device(&mut handle, g13_state).unwrap(),
            None => panic!("Could not find device"),
        },
        Err(e) => panic!("Could not initialize libusb: {}", e),
    }
}

fn open_device(
    context: &mut libusb::Context,
    vid: u16,
    pid: u16,
) -> Option<(
    libusb::Device,
    libusb::DeviceDescriptor,
    libusb::DeviceHandle,
)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

fn read_device(
    handle: &mut libusb::DeviceHandle,
    g13_state:  &mut ::G13State
) -> libusb::Result<()> {
    try!(handle.reset());

    let endpoint = Endpoint {
        config:  ::g13_consts::KEY_ENDPOINT,
        iface:   ::g13_consts::INTERFACE,
        setting: ::g13_consts::SETTING,
        address: ::g13_consts::ADDRESS,
    };
    read_endpoint(handle, endpoint, g13_state);
    Ok(())
}

fn read_endpoint(handle: &mut libusb::DeviceHandle, endpoint: Endpoint, g13_state: &mut ::G13State) {
    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface).ok();
            true
        }
        _ => false,
    };
    match configure_endpoint(handle, &endpoint) {
        Ok(_) => {
            let mut report = [0; ::g13_consts::REPORT_SIZE];
            let timeout = Duration::from_micros(::g13_consts::KEY_READ_TIMEOUT);
            loop {
                match handle.read_interrupt(endpoint.address, &mut report, timeout) {
                    Ok(_) => {
                        g13_uinput::process_report(report, g13_state);
                    }
                    Err(err) => match err {
                        libusb::Error::Timeout => continue,
                        _ => {
                            eprintln! {"{}", err};
                            break;
                        }
                    },
                }
            }
        }
        Err(err) => eprintln!("could not configure endpoint: {}", err),
    }

    if has_kernel_driver {
        handle.attach_kernel_driver(endpoint.iface).ok();
    }
}

fn configure_endpoint<'a>(
    handle: &'a mut libusb::DeviceHandle,
    endpoint: &Endpoint,
) -> libusb::Result<()> {
    try!(handle.set_active_configuration(endpoint.config));
    try!(handle.claim_interface(endpoint.iface));
    try!(handle.set_alternate_setting(endpoint.iface, endpoint.setting));
    Ok(())
}
