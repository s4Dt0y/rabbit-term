use nix::errno::Errno;
use nix::unistd::ForkResult;
use std::ffi::{CStr, CString};
use std::os::fd::{AsRawFd, OwnedFd};

fn main() {
    let fd = spawn_shell();
    let mut buf = vec![0, 255];

    let fd = fd.unwrap();
    set_nonblock(&fd);

    loop {
        let mut temp = vec![0u8; 4096];
        match nix::unistd::read(fd.as_raw_fd(), &mut temp) {
            Ok(read_size) => buf.extend_from_slice(&temp[0..read_size]),
            Err(Errno::EAGAIN) => continue,
            _ => println!("Error"),
        }

        let print_content = unsafe { std::str::from_utf8_unchecked(&buf) };
        println!("{}", print_content);
    }
}

fn spawn_shell() -> Option<OwnedFd> {
    unsafe {
        let res = nix::pty::forkpty(None, None).unwrap();

        match res.fork_result {
            ForkResult::Parent { .. } => println!("WOP"),
            ForkResult::Child => {
                println!("dop");
                let shell_name = c"/bin/bash";

                nix::unistd::execvp::<CString>(shell_name, &[]).expect("Could not spawn shell");
                std::process::exit(1);
            }
        }
        Some(res.master)
    }
}

fn set_nonblock(fd: &OwnedFd) -> () {
    let flags = nix::fcntl::fcntl(fd.as_raw_fd(), nix::fcntl::FcntlArg::F_GETFL)
        .expect("Failed to set F_GETFL");
    let mut flags = nix::fcntl::OFlag::from_bits(flags & nix::fcntl::OFlag::O_ACCMODE.bits())
        .expect("Failed to set 0_ACCMOD");
    flags.set(nix::fcntl::OFlag::O_NONBLOCK, true);

    nix::fcntl::fcntl(fd.as_raw_fd(), nix::fcntl::FcntlArg::F_SETFL(flags))
        .expect("Failed to SET_FL");
}
