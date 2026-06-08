use super::secure_dns::{DnsQuery, SecureDnsResolver, DnsResponse};

pub struct Resolver {
    dns: SecureDnsResolver,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            dns: SecureDnsResolver::new(),
        }
    }

    pub fn lookup(&self, domain: &'static str) -> Option<DnsResponse> {
        let query = DnsQuery { domain };

        self.dns.resolve(query)
    }
}
