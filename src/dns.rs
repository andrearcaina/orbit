use hickory_proto::{
    ProtoError,
    op::{Message, MessageType, OpCode, Query},
    rr::{Name, RecordType},
};
use std::{error::Error, io};
use url::Url;

pub struct Resolver {
    server: String,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            server: hickory_proto::ROOTS[0].to_string(),
        }
    }

    pub fn resolve(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let domain = if input.contains("://") {
            Url::parse(input)?
                .host_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "URL has no hostname"))?
                .to_owned()
        } else {
            input.trim_end_matches('.').to_owned()
        };
        let query = Self::build_a_query(&domain)?;

        Ok(format!(
            "Built a {}-byte DNS query for {} using server {}",
            query.len(),
            domain,
            self.server
        ))
    }

    fn build_a_query(domain: &str) -> Result<Vec<u8>, ProtoError> {
        let name = Name::from_ascii(domain)?;
        let mut message = Message::new(0x1234, MessageType::Query, OpCode::Query);
        message.add_query(Query::query(name, RecordType::A));

        message.to_vec()
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_query_when_resolving() {
        let resolver = Resolver::new();
        assert_eq!(
            resolver.resolve("https://example.com").unwrap(),
            "Built a 29-byte DNS query for example.com using server 198.41.0.4"
        );
    }

    #[test]
    fn builds_an_a_query_for_example_com() {
        let query = Resolver::build_a_query("example.com").unwrap();
        assert_eq!(query.len(), 29);
    }
}
