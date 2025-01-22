use rusb::{Context, DeviceHandle, Error, UsbContext};
fn main() {
    match exe() {
        Ok(_) => println!("ok"),
        Err(e) => println!("Error: {}", e),
    }
}
fn exe() -> Result<(), Error> {
    let context = Context::new()?;

    let vendor_id = 0x054c;
    let product_id = 0x0ce6;

    let mut handle: Option<DeviceHandle<Context>> = None;

    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
            handle = Some(device.open()?);
            break;
        }
    }

    let handle = handle.expect("Device not found");
    if handle.kernel_driver_active(3)? {
        let _ = handle.detach_kernel_driver(3);
    }
    handle.claim_interface(3)?;

    let mut buf: [u8; 48] = [0; 48];
    buf[0] = 0x02;
    buf[1] = 0xff;
    buf[2] = 0x57;
    // buf[11]: RightTriggerMode
    // buf[12:21]: RightTriggerParams
    // buf[22]: LeftTriggerMode
    // buf[23:32]: LeftTriggerParams

    let _ = handle.write_bulk(3, &buf, std::time::Duration::from_millis(100));

    handle.release_interface(3)?;

    Ok(())
}
