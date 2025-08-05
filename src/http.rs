// http.rs - HTTP client functionality for SoftEther protocol

use std::io;
use crate::{Pack, AsyncSock};

// HTTP constants - exactly matching Go version
pub const HTTP_CONTENT_TYPE: &str = "application/octet-stream";
pub const HTTP_CONTENT_TYPE2: &str = "image/jpeg"; 
pub const HTTP_KEEP_ALIVE: &str = "timeout=15; max=19";
pub const HTTP_VPN_TARGET: &str = "/vpnsvc/vpn.cgi";
pub const HTTP_VPN_TARGET2: &str = "/vpnsvc/connect.cgi";
pub const HTTP_PACK_RAND_SIZE_MAX: u32 = 1000;

// HttpClientSend sends a Pack via HTTP POST - exact same as Go
pub async fn http_client_send(sock: &mut AsyncSock, pack: &Pack) -> io::Result<Vec<u8>> {
    // Serialize the pack
    let pack_data = pack.to_buf().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Pack serialization failed: {}", e))
    })?;
    
    // Create HTTP POST request
    let content_length = pack_data.len();
    
    let request = format!(
        "POST {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Content-Length: {}\r\n\
         Content-Type: application/octet-stream\r\n\
         \r\n",
        HTTP_VPN_TARGET,
        sock.remote_ip,
        content_length
    );
    
    // Send HTTP headers
    sock.write_all(request.as_bytes()).await?;
    
    // Send pack data
    sock.write_all(&pack_data).await?;
    sock.flush().await?;
    
    // Read HTTP response
    read_http_response(sock).await
}

// Read HTTP response and extract body
async fn read_http_response(sock: &mut AsyncSock) -> io::Result<Vec<u8>> {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    
    let mut reader = BufReader::new(&mut sock.tls_stream);
    let mut response_line = String::new();
    
    // Read status line
    reader.read_line(&mut response_line).await?;
    println!("[DEBUG] HTTP status line: {}", response_line.trim());
    if !response_line.starts_with("HTTP/1.1 200") && !response_line.starts_with("HTTP/1.0 200") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("HTTP error: {}", response_line.trim())
        ));
    }
    
    // Read headers until empty line
    let mut content_length = 0;
    loop {
        let mut header_line = String::new();
        reader.read_line(&mut header_line).await?;
        
        if header_line.trim().is_empty() {
            break; // End of headers
        }
        
        if header_line.to_lowercase().starts_with("content-length:") {
            if let Some(len_str) = header_line.split(':').nth(1) {
                content_length = len_str.trim().parse().unwrap_or(0);
            }
        }
        if header_line.to_lowercase().starts_with("content-type:") {
            println!("[DEBUG] HTTP Content-Type: {}", header_line.trim());
        }
    }
    
    // Read body
    if content_length > 0 {
        let mut body = vec![0u8; content_length];
        use tokio::io::AsyncReadExt;
        reader.read_exact(&mut body).await?;
        println!("[DEBUG] HTTP response body length: {} bytes", body.len());
        // If it looks like HTML, show the first part
        if body.len() > 10 && body.starts_with(b"<!DOCTYPE") {
            let html_preview = String::from_utf8_lossy(&body[..std::cmp::min(200, body.len())]);
            println!("[DEBUG] HTML response detected: {}", html_preview);
        }
        Ok(body)
    } else {
        // Read until connection closes if no content-length
        let mut body = Vec::new();
        use tokio::io::AsyncReadExt;
        reader.read_to_end(&mut body).await?;
        Ok(body)
    }
}
