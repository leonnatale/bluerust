use std::mem::{size_of, zeroed};

include!(concat!(env!("OUT_DIR"), "/bluetooth_bindings.rs"));

fn main() {
    unsafe {
        let mut device: _BLUETOOTH_DEVICE_INFO = zeroed();
        device.dwSize = size_of::<_BLUETOOTH_DEVICE_INFO>() as u32;
        
        let mut search: _BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
        search.dwSize = size_of::<_BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
        search.fReturnAuthenticated = true.into();
        search.fReturnConnected = true.into();
        search.fReturnRemembered = true.into();

        let device_handle = BluetoothFindFirstDevice(&search, &mut device);
        
        if device_handle.is_null() {
            eprintln!("No devices were found");
        } else {
            loop {
                let name_utf16 = &device.szName;
                let name = String::from_utf16_lossy(&name_utf16[..name_utf16.iter().position(|&c| c == 0).unwrap_or(name_utf16.len())]);

                println!("Found device: {:?}", name);

                let result = BluetoothFindNextDevice(device_handle, &mut device);
                if result == 0 {
                    break;
                }
            }
            // Fecha a busca de dispositivos
            BluetoothFindDeviceClose(device_handle);
        }
    }
}
