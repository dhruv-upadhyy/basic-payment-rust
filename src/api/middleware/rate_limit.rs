use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: usize,
    reset_at: Instant,
}

#[derive(Clone)]
pub struct RateLimiter {
    entries: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    fn is_rate_limited(&self, ip: &str) -> bool {
        let mut entries = self.entries.lock().unwrap();
        
        let now = Instant::now();
        
        // Clean up old entries
        entries.retain(|_, entry| entry.reset_at > now);
        
        match entries.get_mut(ip) {
            Some(entry) => {
                if entry.reset_at <= now {
                    // Reset if window expired
                    entry.count = 1;
                    entry.reset_at = now + self.window;
                    false
                } else if entry.count >= self.max_requests {
                    // Rate limited
                    true
                } else {
                    entry.count += 1;
                    false
                }
            }
            None => {
                // First request
                entries.insert(
                    ip.to_string(),
                    RateLimitEntry {
                        count: 1,
                        reset_at: now + self.window,
                    },
                );
                false
            }
        }
    }
}

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    limiter: axum::extract::Extension<RateLimiter>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = addr.ip().to_string();
    
    if limiter.is_rate_limited(&ip) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
} 