use crate::types::TestResult; // Importation de TestResult pour structurer les résultats des tests
use std::collections::HashMap; // Utilisé pour calculer la fréquence des motifs dans les seeds

// Constantes de seuil pour le test de Poker
const POKER_TEST_MIN: f64 = 10.0;
const POKER_TEST_MAX: f64 = 22.0; // Seuil ajusté

/// Exécute le test de Poker sur les seeds donnés pour vérifier leur distribution.
/// `seeds`: Un tableau d'entiers `u64` représentant les seeds.
/// Retourne : Une structure `TestResult` avec les résultats du test de Poker.
pub fn test_poker(seeds: &[u64]) -> TestResult {
    let poker_stat = calculate_poker_stat(seeds); // Calcul du score du test de Poker
    let passed = poker_stat >= POKER_TEST_MIN && poker_stat <= POKER_TEST_MAX;

    TestResult {
        test_name: "Test de Poker".to_string(),
        passed,
        score: poker_stat,
        details: format!("Statistique de Poker calculée : {:.2}", poker_stat),
        thresholds: Some((POKER_TEST_MIN, POKER_TEST_MAX)),
    }
}

/// Calcule la statistique de Poker en utilisant les 4 derniers bits de chaque seed.
/// `seeds`: Tableau d'entiers `u64` représentant les seeds.
/// Retourne : Le score de la statistique de Poker.
fn calculate_poker_stat(seeds: &[u64]) -> f64 {
    let mut frequency_map = HashMap::new(); // Dictionnaire pour stocker les fréquences des motifs

    // Comptabilisation des occurrences des 4 derniers bits de chaque seed
    for seed in seeds {
        let pattern = seed & 0xF; // Conserve les 4 derniers bits
        *frequency_map.entry(pattern).or_insert(0) += 1; // Compte le motif
    }

    // Calcul de la statistique de Poker basée sur les fréquences observées
    let n = seeds.len() as f64;
    let sum_frequencies: f64 = frequency_map.values().map(|&count| (count as f64).powi(2)).sum();
    (16.0 / n) * sum_frequencies - n // Calcul ajusté pour la statistique de Poker
}

#[cfg(test)]
mod tests {
    use super::{test_poker, POKER_TEST_MIN, POKER_TEST_MAX};
    use crate::types::TestResult;
    use rand::Rng;

    /// Génère une série de seeds aléatoires pour les tests.
    fn generate_random_seeds() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| rng.gen::<u64>()).collect()
    }

    /// Test de la fonction `test_poker` avec des seeds aléatoires.
    #[test]
    fn test_poker_with_random_seeds() {
        let seeds = generate_random_seeds(); // Génère des seeds pour le test
        let result: TestResult = test_poker(&seeds); // Exécute le test de Poker
        println!("Score de poker calculé : {:?}", result.score); // Affiche le score pour référence
        assert!(
            result.score >= POKER_TEST_MIN && result.score <= POKER_TEST_MAX,
            "Échec du test de poker : score = {:?}",
            result.score
        );
    }
}
