use crate::UdpSas;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use tokio::io::unix::AsyncFd;

pub struct UdpSocketSas {
    local_addr: SocketAddr,
    inner: AsyncFd<UdpSocket>,
}

impl UdpSocketSas {
    pub fn bind(addr: SocketAddr) -> std::io::Result<Self> {
        let udp = UdpSocket::bind_sas(addr)?;
        let local_addr = udp.local_addr()?;
        udp.set_nonblocking(true)?;
        Ok(Self {
            local_addr,
            inner: AsyncFd::new(udp)?,
        })
    }

    pub fn socket(&mut self) -> &mut UdpSocket {
        self.inner.get_mut()
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub fn local_port(&self) -> u16 {
        self.local_addr.port()
    }

    pub async fn recv_sas(&self, out: &mut [u8]) -> std::io::Result<(usize, SocketAddr, IpAddr)> {
        loop {
            let mut guard = self.inner.readable().await?;
            match guard.try_io(|inner| inner.get_ref().recv_sas(out)) {
                Ok(result) => return result,
                Err(_would_block) => continue,
            }
        }
    }

    pub async fn send_sas(
        &self,
        buf: &[u8],
        source: IpAddr,
        dest: SocketAddr,
    ) -> std::io::Result<usize> {
        loop {
            let mut guard = self.inner.writable().await?;
            match guard.try_io(|inner| inner.get_ref().send_sas(buf, &dest, &source)) {
                Ok(result) => return result,
                Err(_would_block) => continue,
            }
        }
    }
}
