// Seul endroit non sûr de l'adaptateur. objc2-core-foundation ne sait pas lire un
// conteneur Core Foundation non typé (`CFArray`, `CFDictionary` sans paramètres) sans
// qu'on lui affirme le type de ses éléments ; il n'offre pour cela que `cast_unchecked`.
//
// Invariant tenu : on n'affirme que `CFType` — « ceci est un objet Core Foundation » —
// jamais un type concret. C'est vrai de tout conteneur rendu par Core Graphics, qui ne
// stocke que des objets CF (tableaux et dictionnaires créés avec les callbacks
// `kCFType*`). Chaque valeur lue est ensuite vérifiée à l'exécution par `downcast`, et
// aucune clé n'est jamais relue depuis le dictionnaire : les clés ne servent qu'à la
// recherche, par égalité CF.
#![allow(unsafe_code)]

use objc2_core_foundation::{CFArray, CFDictionary, CFRetained, CFType};

pub fn objects_of(array: CFRetained<CFArray>) -> CFRetained<CFArray<CFType>> {
    // SAFETY: voir l'invariant du module — un CFArray de Core Graphics ne contient que
    // des objets Core Foundation.
    unsafe { CFRetained::cast_unchecked(array) }
}

pub fn entries_of(
    dictionary: CFRetained<CFDictionary>,
) -> CFRetained<CFDictionary<CFType, CFType>> {
    // SAFETY: voir l'invariant du module — clés et valeurs d'un CFDictionary de Core
    // Graphics sont des objets Core Foundation.
    unsafe { CFRetained::cast_unchecked(dictionary) }
}
