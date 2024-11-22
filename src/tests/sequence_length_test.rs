use crate::types::TestResult; // Import nécessaire pour structurer les résultats du test

// Seuils pour le test de longueur de séquence
const SEQUENCE_LENGTH_TEST_MIN: f64 = 10.0; // Valeur minimale
const SEQUENCE_LENGTH_TEST_MAX: f64 = 18.0; // Valeur maximale

/// Test de longueur de séquence pour détecter des biais dans la longueur maximale de séquences de '1'.
/// `seeds` : Tableau de seeds.
/// Retourne : Une structure TestResult avec les résultats du test.
pub fn test_sequence_length(seeds: &[u64]) -> TestResult {
    let max_length = calculate_max_sequence_length(seeds);
    let passed = max_length >= SEQUENCE_LENGTH_TEST_MIN && max_length <= SEQUENCE_LENGTH_TEST_MAX;

    TestResult {
        test_name: "Test de longueur de séquence".to_string(),
        passed,
        score: max_length,
        details: format!("Longueur maximale de séquence de '1' : {:.2}", max_length),
        thresholds: Some((SEQUENCE_LENGTH_TEST_MIN, SEQUENCE_LENGTH_TEST_MAX)),
    }
}

/// Calcule la longueur maximale de séquences consécutives de '1' dans les seeds.
/// `seeds` : Tableau de seeds.
/// Retourne : La longueur maximale de séquence de '1'.
fn calculate_max_sequence_length(seeds: &[u64]) -> f64 {
    seeds
        .iter()
        .map(|seed| {
            let mut max_len = 0;
            let mut current_len = 0;
            for i in 0..64 {
                if seed & (1 << i) != 0 {
                    current_len += 1;
                    if current_len > max_len {
                        max_len = current_len;
                    }
                } else {
                    current_len = 0;
                }
            }
            max_len as f64
        })
        .fold(0.0, f64::max) // Prend le maximum parmi toutes les séquences
}

#[cfg(test)]
mod tests {
    use super::{test_sequence_length, SEQUENCE_LENGTH_TEST_MIN, SEQUENCE_LENGTH_TEST_MAX};
    use rand::Rng;

    /// Génère des seeds aléatoires pour les tests.
    fn generate_random_seeds() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| rng.gen::<u64>()).collect()
    }

    /// Test de la fonction `test_sequence_length` avec des seeds aléatoires.
    #[test]
    fn test_sequence_length_with_random_seeds() {
        let seeds = generate_random_seeds();
        let result = test_sequence_length(&seeds);
        println!("Score de longueur de séquence calculé : {:?}", result.score);
        assert!(
            result.score >= SEQUENCE_LENGTH_TEST_MIN && result.score <= SEQUENCE_LENGTH_TEST_MAX,
            "Échec du test de longueur de séquence : score = {:?}",
            result.score
        );
    }
}
