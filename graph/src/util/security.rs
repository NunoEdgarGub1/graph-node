use std::net::SocketAddr;
use url::Url;

pub fn display_url(url: &str) -> String {
    let mut url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return String::from(url),
    };

    if url.password().is_some() {
        url.set_password(Some("HIDDEN_PASSWORD"))
            .expect("failed to redact password");
    }

    return url.into_string();
}

pub fn display_socket_addr(addr: &SocketAddr) -> String {
    display_url(addr.to_string().as_str())
}
