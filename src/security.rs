use url::Url;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub allow_localhost: bool,
    pub allow_loopback: bool,
    pub allowed_ports: Vec<u16>,
    pub strict_mode: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            allow_localhost: true,
            allow_loopback: true,
            allowed_ports: vec![80, 443, 3000, 8080, 5173],
            strict_mode: true,
        }
    }
}

pub struct SecurityFilter {
    policy: SecurityPolicy,
}

impl SecurityFilter {
    pub fn new(policy: &SecurityPolicy) -> Self {
        Self {
            policy: policy.clone(),
        }
    }

    pub fn is_allowed(&self, request_url: &str) -> bool {
        if request_url.is_empty() {
            return true;
        }

        let url = match Url::parse(request_url) {
            Ok(url) => url,
            Err(_) => return false,
        };

        if url.scheme() != "http" && url.scheme() != "https" {
            return false;
        }

        let host = match url.host_str() {
            Some(h) => h,
            None => return false,
        };

        if self.is_local_host(host) {
            self.is_allowed_port(url.port().unwrap_or(80))
        } else {
            false
        }
    }

    fn is_local_host(&self, host: &str) -> bool {
        if self.policy.allow_localhost && host == "localhost" {
            return true;
        }

        if self.policy.allow_loopback {
            if host == "127.0.0.1" || host == "::1" {
                return true;
            }

            if let Ok(ip_addr) = host.parse::<IpAddr>() {
                return match ip_addr {
                    IpAddr::V4(ipv4) => ipv4.is_loopback(),
                    IpAddr::V6(ipv6) => ipv6.is_loopback(),
                };
            }
        }

        false
    }

    fn is_allowed_port(&self, port: u16) -> bool {
        if self.policy.strict_mode {
            self.policy.allowed_ports.contains(&port)
        } else {
            true
        }
    }

    pub fn filter_urls(&self, urls: Vec<String>) -> Vec<String> {
        urls.into_iter()
            .filter(|url| self.is_allowed(url))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localhost_allowed() {
        let policy = SecurityPolicy::default();
        let filter = SecurityFilter::new(&policy);
        
        assert!(filter.is_allowed("http://localhost:3000"));
        assert!(filter.is_allowed("https://localhost:8080"));
        assert!(filter.is_allowed("http://127.0.0.1:80"));
        assert!(!filter.is_allowed("http://example.com"));
        assert!(!filter.is_allowed("ftp://localhost"));
    }
}
