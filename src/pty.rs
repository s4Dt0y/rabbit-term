use crate::log::{event, Level};
use nix::errno::Errno;
use nix::unistd::ForkResult;
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::io::FromRawFd;

pub use crate::messages::pty::{PtyActionError, PtyActionSuccess};

pub trait Pty {
    fn read(&mut self) -> Result<PtyActionSuccess, PtyActionError> {
        unimplemented!();
    }
    fn write(&mut self, content: &str) -> Result<PtyActionSuccess, PtyActionError> {
        unimplemented!()
    }
}

pub struct RtPty {
    pub buf: Vec<u8>,
    fd: OwnedFd,
    file: File,
}

impl RtPty {
    pub fn new(shell: &str) -> Self {
        let fd = spawn_shell(shell);
        let buf = vec![0, 255];

        let fd = fd.unwrap();
        set_nonblock(&fd);

        let file = unsafe { File::from_raw_fd(fd.as_raw_fd()) };

        return Self { buf, fd, file };
    }

    pub fn display_buf(&self) -> String {
        let output = String::from_utf8(self.buf.to_owned());

        if let Err(e) = output {
            event!(Level::ERROR, "{}", e);
        }
        String::from_utf8_lossy(&self.buf).to_string()
    }

    fn fd(&self) -> &OwnedFd {
        &self.fd
    }
}

impl Pty for RtPty {
    fn read(&mut self) -> Result<PtyActionSuccess, PtyActionError> {
        let mut temp = vec![0u8; 4096];
        match nix::unistd::read(self.fd().as_raw_fd(), &mut temp) {
            Ok(read_size) => {
                self.buf.extend_from_slice(&temp[0..read_size]);
                return Ok(PtyActionSuccess::ReadSuccess);
            }
            Err(Errno::EAGAIN) => Err(PtyActionError::NoContent),
            Err(e) => Err(PtyActionError::ReadFailed(e)),
        }
    }

    fn write(&mut self, content: &str) -> Result<PtyActionSuccess, PtyActionError> {
        match write!(&mut self.file, "{}", content) {
            Ok(_) => Ok(PtyActionSuccess::WriteSuccess),
            Err(e) => Err(PtyActionError::WriteFailed(e)),
        }
    }
}

pub fn spawn_shell(shell: &str) -> Option<OwnedFd> {
    unsafe {
        let res = nix::pty::forkpty(None, None).unwrap();

        match res.fork_result {
            ForkResult::Parent { .. } => (),
            ForkResult::Child => {
                let shell_name = CString::new(shell).unwrap();
                nix::unistd::execvp::<CString>(&shell_name, &[]).expect("Could not spawn shell");
                std::process::exit(1);
            }
        }
        Some(res.master)
    }
}

pub fn set_nonblock(fd: &OwnedFd) -> () {
    let flags = nix::fcntl::fcntl(fd.as_raw_fd(), nix::fcntl::FcntlArg::F_GETFL)
        .expect("Failed to set F_GETFL");
    let mut flags = nix::fcntl::OFlag::from_bits(flags & nix::fcntl::OFlag::O_ACCMODE.bits())
        .expect("Failed to set 0_ACCMOD");
    flags.set(nix::fcntl::OFlag::O_NONBLOCK, true);

    nix::fcntl::fcntl(fd.as_raw_fd(), nix::fcntl::FcntlArg::F_SETFL(flags))
        .expect("Failed to SET_FL");
}
