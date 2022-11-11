
use std::{thread, time::Duration};

fn main() {
    use modbus::tcp;
    use modbus::{Client};

    let mut cfg = tcp::Config::default();
    cfg.tcp_port = 502;
    cfg.modbus_uid = 1;
    cfg.tcp_read_timeout = Option::from(Duration::from_secs(10));
    cfg.tcp_connect_timeout = Option::from(Duration::from_secs(10));
    // change cfg port to 502 // 0x7D42 = 32066
    let mut client = tcp::Transport::new_with_cfg("192.168.8.101", cfg).unwrap();
    println!("Connected");
    thread::sleep(Duration::from_secs(2));
    //println!("{:?}", client.    #Temperature 32087 0x7D57 1);
    let data = client.read_holding_registers(0x7D57, 20); // 7D57
    match data {
        Ok(data) => {
            println!("data: {:?}", data);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}