#[derive(Clone, Copy)]
pub enum SecurityLabel {
    System,
    Trusted,
    User,
    Restricted,
}

pub struct MacPolicy;

impl MacPolicy {
    pub fn can_access(
        subject: SecurityLabel,
        object: SecurityLabel,
    ) -> bool {
        match subject {
            SecurityLabel::System => true,
            SecurityLabel::Trusted => {
                !matches!(object, SecurityLabel::System)
            }
            SecurityLabel::User => {
                matches!(
                    object,
                    SecurityLabel::User | SecurityLabel::Restricted
                )
            }
            SecurityLabel::Restricted => {
                matches!(object, SecurityLabel::Restricted)
            }
        }
    }
}
