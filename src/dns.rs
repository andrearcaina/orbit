// later, will be used to resolve DNS queries with root servers from the IANA root zone file
pub struct Resolver {
    server: String,
}

impl Resolver {
    pub fn new(server: &str) -> Self {
        Resolver {
            server: server.to_string(),
        }
    }

    pub fn resolve(&self, url: &str) -> String {
        format!("Resolving {} using server {}", url, self.server)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_url_input() {
        let resolver = Resolver::new("8.8.8.8");
        assert_eq!(
            resolver.resolve("https://example.com"),
            "Resolving https://example.com using server 8.8.8.8"
        );
    }
}
