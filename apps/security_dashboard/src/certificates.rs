use std::collections::HashMap;

#[derive(Clone)]
pub struct Certificate {
    pub name: String,
    pub issuer: String,
    pub fingerprint: String,
    pub trusted: bool,
}

pub struct CertificateManager {
    certificates: HashMap<String, Certificate>,
}

impl CertificateManager {
    pub fn new() -> Self {
        Self {
            certificates: HashMap::new(),
        }
    }

    pub fn add_certificate(&mut self, cert: Certificate) {
        self.certificates
            .insert(cert.fingerprint.clone(), cert);
    }

    pub fn revoke_certificate(&mut self, fingerprint: &str) {
        self.certificates.remove(fingerprint);
    }

    pub fn is_trusted(&self, fingerprint: &str) -> bool {
        self.certificates
            .get(fingerprint)
            .map(|c| c.trusted)
            .unwrap_or(false)
    }

    pub fn list_certificates(&self) -> Vec<&Certificate> {
        self.certificates.values().collect()
    }
}
