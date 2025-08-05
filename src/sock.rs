// sock.rs - Socket abstraction for SoftEther-rust

use std::io;
use tokio::net::TcpStream;
use tokio_rustls::{TlsStream, client::TlsStream as ClientTlsStream};

// AsyncSock - Async version of Sock for SoftEther-rust  
#[derive(Debug)]
pub struct AsyncSock {
    pub tls_stream: TlsStream<TcpStream>,
    pub raw_stream: TcpStream,
    pub remote_ip: String,
    pub hostname: String,  // Store hostname for Host header
    pub insecure_skip_verify: bool,
}

impl AsyncSock {
    pub fn new(tls_stream: ClientTlsStream<TcpStream>, raw_stream: TcpStream) -> io::Result<Self> {
        // Get remote IP from the raw stream since TLS wrapper doesn't expose it directly
        let remote_ip = raw_stream.peer_addr()?.ip().to_string();
        
        // Wrap the client TLS stream in the enum
        let wrapped_stream = TlsStream::Client(tls_stream);
            
        Ok(Self {
            tls_stream: wrapped_stream,
            raw_stream,
            remote_ip: remote_ip.clone(),
            hostname: remote_ip,  // Default to IP, will be overridden with hostname if available
            insecure_skip_verify: false,
        })
    }
    
    pub fn new_with_hostname(tls_stream: ClientTlsStream<TcpStream>, raw_stream: TcpStream, hostname: String) -> io::Result<Self> {
        // Get remote IP from the raw stream since TLS wrapper doesn't expose it directly
        let remote_ip = raw_stream.peer_addr()?.ip().to_string();
        
        // Wrap the client TLS stream in the enum
        let wrapped_stream = TlsStream::Client(tls_stream);
            
        Ok(Self {
            tls_stream: wrapped_stream,
            raw_stream,
            remote_ip,
            hostname,  // Use provided hostname for Host header
            insecure_skip_verify: false,
        })
    }
    
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use tokio::io::AsyncReadExt;
        // Direct read from TLS stream (buffering handled by underlying implementation)
        self.tls_stream.read(buf).await
    }
    
    pub async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use tokio::io::AsyncWriteExt;
        self.tls_stream.write(buf).await
    }
    
    pub async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        use tokio::io::AsyncWriteExt;
        self.tls_stream.write_all(buf).await
    }
    
    pub async fn flush(&mut self) -> io::Result<()> {
        use tokio::io::AsyncWriteExt;
        self.tls_stream.flush().await
    }
    
    // WTFWriteRaw - write raw data directly to TCP stream (bypass TLS)
    pub async fn wtf_write_raw(&mut self, buf: &[u8]) -> io::Result<usize> {
        use tokio::io::AsyncWriteExt;
        self.raw_stream.write(buf).await
    }
    
    pub async fn close(mut self) -> io::Result<()> {
        use tokio::io::AsyncWriteExt;
        // Properly close the TLS connection
        self.tls_stream.shutdown().await?;
        Ok(())
    }
    
    /// Send all data, ensuring the entire buffer is sent
    pub async fn send_all(&mut self, data: &[u8]) -> io::Result<()> {
        self.write_all(data).await?;
        self.flush().await
    }
    
    /// Receive exactly the specified number of bytes
    /// If blocking is false and no data is available, returns Ok(0)
    pub async fn recv_exact(&mut self, buf: &mut [u8], blocking: bool) -> io::Result<usize> {
        use tokio::io::AsyncReadExt;
        
        if blocking {
            // Blocking read - read exact amount
            self.tls_stream.read_exact(buf).await?;
            Ok(buf.len())
        } else {
            // Non-blocking read - just read what's immediately available
            // For TLS streams, we'll use a timeout to simulate non-blocking behavior
            match tokio::time::timeout(std::time::Duration::from_millis(1), self.read(buf)).await {
                Ok(Ok(n)) => Ok(n),
                Ok(Err(e)) => Err(e),
                Err(_) => Ok(0), // Timeout = no data available
            }
        }
    }
}

// TODO: Implement exact translation of sock.go
pub struct SockStub;

impl SockStub {
    pub fn new() -> Self {
        Self
    }
}
