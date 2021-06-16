use std::prelude::*;
use socketcan::*;
use std::io::*;
fn main() {
    println!("Hello, world!");

    let canport1:socketcan::CANSocket = socketcan::CANSocket::open("vcan0")
        .expect("didnt open")
        ;

    let canport2:socketcan::CANSocket = socketcan::CANSocket::open("vcan0")
        .expect("didnt open")
        ;

    print_type_of(&canport1);
    canport1.set_read_timeout(std::time::Duration::new(2,0));
    canport2.set_read_timeout(std::time::Duration::new(2,0));
    let frame1: socketcan::CANFrame = socketcan::CANFrame::new(
        123,
        &[1,23,4,42],
        true,
        true
    ).unwrap();
    loop {
    canport1.write_frame_insist(&frame1).expect("could not send frame");
    println!("printing frame {}\r", frame1.id());
    let out = canport1.read_frame().unwrap();//.expect("cant read");
    println!("{}", out.id());
    std::io::stdout().write(out.data());
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}