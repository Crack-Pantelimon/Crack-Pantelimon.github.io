#[cfg(feature = "web")]
pub const DATA_BASE_URL: &str = "https://pantelimon.alt-f4.ro/";
#[cfg(not(feature = "web"))]
pub const DATA_BASE_URL: &str = "http://127.0.0.1:1973/";
// pub const DATA_BASE_URL: &str = "https://pantelimon.alt-f4.ro/";

/// Returns the base URL of the current page at runtime (wasm only).
/// E.g. if the page is at `https://example.com/crack/index.html`,
/// this returns `https://example.com/crack/`.
/// Falls back to "/" if detection fails.
#[cfg(feature = "web")]
pub fn get_page_base_url() -> String {
    web_sys::window()
        .and_then(|w| w.location().href().ok())
        .map(|href| {
            // Strip filename (e.g., "index.html") to get the directory
            if let Some(last_slash) = href.rfind('/') {
                href[..=last_slash].to_string()
            } else {
                href
            }
        })
        .unwrap_or_else(|| "/".to_string())
}

#[cfg(not(feature = "web"))]
pub fn get_page_base_url() -> String {
    DATA_BASE_URL.to_string()
}
