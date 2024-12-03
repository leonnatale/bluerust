use std::mem::{size_of, zeroed};

use binding::{ _BLUETOOTH_DEVICE_INFO, _BLUETOOTH_DEVICE_SEARCH_PARAMS, BluetoothFindFirstDevice, BluetoothFindDeviceClose };

mod binding {
    include!(concat!(env!("OUT_DIR"), "/bluetooth_bindings.rs"));
}

#[derive(Debug)]
pub struct BluetoothDevice {
    name: String,
    address: u64,
    class: u32
}

fn utf16_to_utf8(utf16: &[u16]) -> String {
    String::from_utf16_lossy(&utf16[..utf16.iter().position(|&c| c == 0).unwrap_or(utf16.len())])
}

pub fn find_first_bluetooth_device(connected: bool, remembered: bool) -> Option<BluetoothDevice> {
    unsafe {
        let mut device: _BLUETOOTH_DEVICE_INFO = zeroed();
        device.dwSize = size_of::<_BLUETOOTH_DEVICE_INFO>() as u32;
        let mut search: _BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
        search.dwSize = size_of::<_BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
        search.fReturnConnected = connected.into();
        search.fReturnRemembered = remembered.into();

        let first_device_handle = BluetoothFindFirstDevice(&search, &mut device);
        if !first_device_handle.is_null() {
            let device_name = utf16_to_utf8(&device.szName);

            let device_info = BluetoothDevice {
                name: device_name,
                address: device.Address.__bindgen_anon_1.ullLong,
                class: device.ulClassofDevice
            };
            return Some(device_info);
        }
        BluetoothFindDeviceClose(first_device_handle);
        None
    }
}