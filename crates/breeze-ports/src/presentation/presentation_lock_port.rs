// Verrou de présentation d'une pause Hardcore (§8.5) : ce que la plateforme retire autour des
// surfaces (barre de menus, Dock, changement d'application) et le premier plan qu'elle rend à
// Breeze. Une capacité de l'enveloppe, pas du domaine : l'Enforcer dit seulement quand.
pub trait PresentationLockPort {
    // Entrée dans la pause Hardcore : retire ce qui peut l'être et amène Breeze devant.
    fn lock(&mut self);
    // À chaque poll pendant la pause : Breeze repris au premier plan s'il l'a perdu.
    fn hold(&mut self);
    // Sortie de la pause, une seule fois : présentation rendue telle qu'avant.
    fn release(&mut self);
}
