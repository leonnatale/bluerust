use bluerust::{ BluetoothDevice, find_first_bluetooth_device };

#[test]
fn find_first_device() {
    let device = find_first_bluetooth_device(
        true,
        false
    );
    dbg!(device);
}