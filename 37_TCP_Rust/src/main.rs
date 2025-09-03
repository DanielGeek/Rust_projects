extern crate tun_tap;
use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let flags = u16::from_be_bytes([buf[0], buf[1]]);
    let proto = u16::from_be_bytes([buf[2], buf[3]]);

    loop {
        match nic.recv(&mut buf[..]) {
            Ok(nbytes) => eprintln!("Read {} bytes: {:x?}", nbytes - 4, &buf[4..nbytes]),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                eprintln!("No packet yet, waiting...");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
