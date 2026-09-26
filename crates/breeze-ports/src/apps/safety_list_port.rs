use breeze_domain::SafetyList;

// La liste de sécurité de l'OS courant (§10.6), fixe : l'adaptateur de la plateforme la
// connaît, le domaine la fait primer sur tout statut choisi.
pub trait SafetyListPort {
    fn safety_list(&self) -> SafetyList;
}
