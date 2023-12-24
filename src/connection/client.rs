use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use async_std::{
    net::TcpStream,
    task::{self, JoinHandle},
};
use futures::{io::WriteHalf, AsyncReadExt};
use log::debug;

use crate::data_type::Packet;

use super::Config;

pub struct Client {
    config_arc: Arc<Config>,
    compressed_atomic: Arc<AtomicBool>,
    socket_addr: SocketAddr,
    write_stream: WriteHalf<TcpStream>,
    thread_handle: JoinHandle<()>,
}

impl Client {
    pub fn new(stream: TcpStream, socket_addr: SocketAddr, config_arc: Arc<Config>) -> Self {
        let (mut read_stream, write_stream) = stream.split();
        let compressed_atomic = Arc::new(AtomicBool::new(false));

        let thread_handle = task::spawn({
            let compressed_atomic = compressed_atomic.clone();

            async move {
                let mut buffer: Vec<u8> = Vec::new();
                let mut tmp_buffer = vec![0; 1024];

                while let Ok(length) = read_stream.read(&mut tmp_buffer).await {
                    if length == 0 {
                        break;
                    }

                    buffer.extend(&tmp_buffer[..length]);

                    while let Ok(packet) =
                        Packet::try_from(&mut buffer, compressed_atomic.load(Ordering::Relaxed))
                    {
                        debug!("{} : {:?}", socket_addr, packet);
                    }
                }
            }
        });

        Self {
            config_arc,
            compressed_atomic,
            socket_addr,
            write_stream,
            thread_handle,
        }
    }

    pub async fn disconnect(self) {
        self.thread_handle.cancel().await;
    }
}
