use std::io;

#[cfg(all(target_os = "linux", feature = "iouring"))]
use io_uring::opcode;

#[cfg(any(feature = "legacy", feature = "poll-io"))]
use super::MaybeFd;
use super::{Op, OpAble};

/// Socket creation operation
pub(crate) struct Socket {
    pub(crate) domain: libc::c_int,
    pub(crate) socket_type: libc::c_int,
    pub(crate) protocol: libc::c_int,
}

impl Op<Socket> {
    /// Create a new socket
    pub(crate) fn socket(
        domain: libc::c_int,
        socket_type: libc::c_int,
        protocol: libc::c_int,
    ) -> io::Result<Self> {
        Op::submit_with(Socket {
            domain,
            socket_type,
            protocol,
        })
    }
}

impl OpAble for Socket {
    #[cfg(all(target_os = "linux", feature = "iouring"))]
    const RET_IS_FD: bool = true;

    #[cfg(all(target_os = "linux", feature = "iouring"))]
    fn uring_op(&mut self) -> io_uring::squeue::Entry {
        opcode::Socket::new(self.domain, self.socket_type, self.protocol).build()
    }

    #[cfg(any(feature = "legacy", feature = "poll-io"))]
    #[inline]
    fn legacy_interest(&self) -> Option<(super::super::ready::Direction, usize)> {
        None
    }

    #[cfg(any(feature = "legacy", feature = "poll-io"))]
    fn legacy_call(&mut self) -> io::Result<MaybeFd> {
        crate::syscall!(socket@FD(self.domain, self.socket_type, self.protocol))
    }
}
