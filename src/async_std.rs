use async_io::Async;

use crate::UdpSas;
use std::net::{IpAddr, SocketAddr, UdpSocket};

pub struct UdpSocketSas {
    local_addr: SocketAddr,
    inner: Async<UdpSocket>,
}

impl UdpSocketSas {
    pub fn bind(addr: SocketAddr) -> std::io::Result<Self> {
        let raw = UdpSocket::bind_sas(addr)?;
        let local_addr = raw.local_addr()?;
        raw.set_nonblocking(true)?;
        Ok(Self {
            local_addr,
            inner: Async::new(raw).expect("Should be able to create async UdpSocket from raw")
        })
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub fn local_port(&self) -> u16 {
        self.local_addr.port()
    }

    pub async fn recv_sas(&self, out: &mut [u8]) -> std::io::Result<(usize, SocketAddr, IpAddr)> {
        loop {
            self.inner.readable().await?;
            match self.inner.get_ref().recv_sas(out) {
                Ok(result) => return Ok(result),
                Err(_would_block) => continue,
            }
        }
    }

    pub async fn recv_from(&self, out: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        loop {
            self.inner.readable().await?;
            match self.inner.get_ref().recv_from(out) {
                Ok(result) => return Ok(result),
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
            self.inner.writable().await?;
            match self.inner.get_ref().send_sas(buf, &dest, &source) {
                Ok(result) => return Ok(result),
                Err(_would_block) => continue,
            }
        }
    }

    pub async fn send_to(
        &self,
        buf: &[u8],
        dest: &SocketAddr
    ) -> std::io::Result<usize> {
        loop {
            self.inner.writable().await?;
            match self.inner.get_ref().send_to(buf, dest) {
                Ok(result) => return Ok(result),
                Err(_would_block) => continue,
            }
        }
    }
}