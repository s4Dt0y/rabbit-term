use nix::errno::Errno;
use nix::unistd::ForkResult;
use std::ffi::CStr;
use std::os::fd::{AsRawFd, OwnedFd};

fn main() {
    let fd = spawn_shell();
    let mut buf = vec![0, 255];

    let fd = fd.unwrap();
    set_nonblock(&fd);

    loop {
        let read_res = nix::unistd::read(fd.as_raw_fd(), &mut buf);

        if let Err(Errno::EAGAIN) = read_res {
            println!("e");
            continue;
        }

        println!("{}", read_res.unwrap());
    }
}

fn spawn_shell() -> Option<OwnedFd> {
    unsafe {
        let res = nix::pty::forkpty(None, None).unwrap();

        match res.fork_result {
            ForkResult::Parent { .. } => (),
            ForkResult::Child => {
                let shell_name = c"/bin/bash";
                let args: &[&[u8]] = &[b"bash\0", b"--noprofile\0", b"--norc\0"];

                let args: Vec<&'static CStr> = args
                    .iter()
                    .map(|v| {
                        CStr::from_bytes_with_nul(v).expect("Should always have null terminator")
                    })
                    .collect::<Vec<_>>();

                nix::unistd::execvp(shell_name, &args).expect("Could not spawn shell");
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
