use breeze_domain::SafetyList;
use breeze_ports::SafetyListPort;

// Sur un OS sans adaptateur, aucune identité n'est connue : la liste est vide, jamais
// remplie par une devinette (§10.6).
pub struct NullSafetyList;

impl SafetyListPort for NullSafetyList {
    fn safety_list(&self) -> SafetyList {
        SafetyList::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_is_empty() {
        assert_eq!(NullSafetyList.safety_list(), SafetyList::default());
    }
}
