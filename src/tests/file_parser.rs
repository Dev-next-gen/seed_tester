/// Parse un fichier CSV en un vecteur de valeurs RNG.
/// 
/// # Arguments
/// * `file_content` : Une chaîne de caractères représentant le contenu du fichier CSV.
///
/// # Retourne
/// Un vecteur de `u64` contenant les valeurs RNG parsées.
///
/// # Remarques
/// * Ignore les lignes vides ou mal formatées.
/// * Supprime les espaces et caractères inutiles autour des valeurs.
pub fn parse_csv(file_content: String) -> Vec<u64> {
    file_content
        .lines() // Sépare le contenu du fichier par lignes.
        .filter_map(|line| {
            let trimmed = line.trim(); // Supprime les espaces inutiles.
            if trimmed.is_empty() {
                return None; // Ignore les lignes vides.
            }
            match trimmed.parse::<u64>() {
                Ok(value) => Some(value), // Ajoute la valeur si le parsing réussit.
                Err(_) => {
                    eprintln!("Ligne ignorée : '{}', parsing échoué.", trimmed); // Journalise les erreurs.
                    None // Ignore les lignes invalides.
                }
            }
        })
        .collect() // Collecte les valeurs valides dans un vecteur.
}
