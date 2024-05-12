use nix::pty::openpty;
use nix::unistd::{close, read};

use std::{
    fs::File,
    io,
    os::fd::{FromRawFd, IntoRawFd},
    process::{Command, Stdio},
};

#[derive(Debug)]
struct Pty {
    fd: i32,
}

fn main() -> io::Result<()> {
    let pty = create_pty("/bin/sh");

    let f = unsafe { File::from_raw_fd(pty.fd.clone()) };

    let output_bytes = output_from_fd(pty.fd.into_raw_fd()).unwrap();
    let result = String::from_utf8(output_bytes).unwrap();

    println!("Read: {}", result);

    match close(pty.fd) {
        Ok(_) => println!("Closed"),
        Err(_) => panic!("Could not close fd"),
    };

    Ok(())
}

fn output_from_fd(fd: i32) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd, &mut read_buffer);
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_) => None,
    }
}

fn create_pty(process: &str) -> Pty {
    let ends = openpty(None, None).expect("Could not open pty");
    let master = ends.master;
    let slave = ends.slave;

    let mut builder = Command::new(process);
    builder.stdin(unsafe { Stdio::from_raw_fd(slave) });
    builder.stdout(unsafe { Stdio::from_raw_fd(slave) });
    builder.stderr(unsafe { Stdio::from_raw_fd(slave) });
    match builder.spawn() {
        Ok(_) => {
            let pty = Pty { fd: master };
            pty
        }
        Err(e) => {
            panic!("Failed to create pty: {}", e)
        }
    }
}
