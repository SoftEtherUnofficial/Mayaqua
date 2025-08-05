use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use rustls::{ClientConfig, RootCertStore};
use webpki_roots;
use std::io;
use std::sync::Arc;
use crate::sock::AsyncSock;

pub async fn tcp_connect(hostname: &str, port: u16) -> io::Result<AsyncSock> {
    tcp_connect_with_config(hostname, port, false).await
}

pub async fn tcp_connect_with_config(hostname: &str, port: u16, insecure_skip_verify: bool) -> io::Result<AsyncSock> {
    // Create TLS config with configurable certificate verification
    let config = if insecure_skip_verify {
        // Create config that accepts all certificates (for testing)
        ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(InsecureServerCertVerifier))
            .with_no_client_auth()
    } else {
        // Standard certificate verification
        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        
        ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
    };
    
    let config = Arc::new(config);
    let connector = TlsConnector::from(config);
    
    // Connect TCP first
    let tcp_stream = TcpStream::connect(format!("{}:{}", hostname, port)).await?;
    
    // We can't clone tokio TcpStream, so we'll connect again for raw access
    let raw_stream = TcpStream::connect(format!("{}:{}", hostname, port)).await?;
    
    // Create TLS connection
    let domain = rustls::pki_types::ServerName::try_from(hostname.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    
    let tls_stream = connector.connect(domain, tcp_stream).await
        .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, e))?;
    
    let mut sock = AsyncSock::new(tls_stream, raw_stream)?;
    sock.insecure_skip_verify = insecure_skip_verify;
    Ok(sock)
}

// Custom certificate verifier for insecure connections (testing only)
#[derive(Debug)]
struct InsecureServerCertVerifier;

impl rustls::client::danger::ServerCertVerifier for InsecureServerCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> std::result::Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        // Accept all certificates - use only for testing
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA1,
            rustls::SignatureScheme::ECDSA_SHA1_Legacy,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}
