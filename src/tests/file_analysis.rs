use crate::types::TestResult;
use crate::tests::{
    test_frequence, test_entropy, test_poker, test_periodicity, test_correlation,
    test_sequence_length, test_collisions,
};

/// Analyse les données RNG issues d'un fichier.
///
/// # Arguments
/// * `data` : Un vecteur contenant les valeurs RNG lues dans un fichier.
///
/// # Retourne
/// Une liste de `TestResult` contenant les résultats des tests.
///
/// # Remarques
/// * Si le vecteur `data` est vide, retourne un `TestResult` avec une erreur.
pub fn analyze_rng_from_file(data: Vec<u64>) -> Vec<TestResult> {
    if data.is_empty() {
        return vec![TestResult {
            test_name: "Analyse RNG".to_string(),
            passed: false,
            score: 0.0,
            details: "Les données fournies sont vides.".to_string(),
            thresholds: None,
        }];
    }

    let tests: Vec<fn(&[u64]) -> TestResult> = vec![
        test_frequence,
        test_entropy,
        test_poker,
        test_periodicity,
        test_correlation,
        test_sequence_length,
        test_collisions,
    ];

    tests
        .iter()
        .map(|test| test(&data)) // Applique chaque test à `data`
        .collect() // Collecte les résultats dans un vecteur
}
