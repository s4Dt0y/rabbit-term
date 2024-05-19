use nix::errno::Errno;
use std::io::Error;

#[derive(Debug)]
pub enum PtyActionSuccess {
    ReadSuccess,
    WriteSuccess,
}

#[derive(Debug)]
pub enum PtyActionError {
    NoContent,
    WriteFailed(Error),
    ReadFailed(Errno),
}
