# DualSense_Minimum_Demo_Rust

This is an **unofficial** demo to try DualSense features.

## Dependencies
This project depends on [rusb](https://github.com/a1ien/rusb)

## How to run

1. Get Rust & cargo on your computer. 
https://www.rust-lang.org/

2. Clone this repository.

3. Set `buf` parameter in `src/main.rs` to active features.
Please refer below repositories.

- [UniSense](https://github.com/nullkal/UniSense/blob/master/UniSense/LowLevel/DualSenseHIDOutputReport.cs)
- [DualSenseSupport](https://github.com/Mxater/DualSenseSupport/blob/master/DualSenseSupport/DeviceInfo.cs)
- [linux drivers](https://github.com/torvalds/linux/blob/master/drivers/hid/hid-playstation.c)
- [DualSense-Windows](https://github.com/Ohjurot/DualSense-Windows/blob/main/VS19_Solution/DualSenseWindows/src/DualSenseWindows/DS5_Output.cpp)
- [DualSenseY-v2](https://github.com/WujekFoliarz/DualSenseY-v2/blob/master/app/DualSense.h)

4. Connect your pc with Dualsence, & Run below command in project root.
```
cargo run
```

If you use linux os, you may have to set [udev](https://wiki.archlinux.org/title/Udev) rule.

