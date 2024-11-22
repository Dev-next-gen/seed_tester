use crate::types::TestResult; // Importation de TestResult pour structurer les résultats des tests

// Constantes pour le Test d'Entropie, définissant les seuils de réussite
const ENTROPY_TEST_MIN: f64 = 0.99; // Valeur minimale pour une entropie acceptable
const ENTROPY_TEST_MAX: f64 = 1.01; // Valeur maximale pour une entropie acceptable

/// Exécute le test d'entropie de Shannon en analysant les seeds.
/// `seeds` : Un tableau de seeds de type u64.
/// Retourne : Une structure TestResult contenant les résultats du test d'entropie.
pub fn test_entropy(seeds: &[u64]) -> TestResult {
    let entropy = calculate_entropy(seeds); // Calcul de l'entropie
    let passed = entropy >= ENTROPY_TEST_MIN && entropy <= ENTROPY_TEST_MAX;

    TestResult {
        test_name: "Test d'entropie de Shannon".to_string(),
        passed,
        score: entropy,
        details: format!("Entropie calculée : {:.2}", entropy),
        thresholds: Some((ENTROPY_TEST_MIN, ENTROPY_TEST_MAX)),
    }
}

/// Calcule l'entropie de Shannon basée sur les bits dans les seeds.
/// `seeds` : Tableau de seeds.
/// Retourne : L'entropie de Shannon calculée.
fn calculate_entropy(seeds: &[u64]) -> f64 {
    let mut count_ones = 0; // Compteur de bits à 1
    let mut total_bits = 0; // Compteur total de bits

    for seed in seeds {
        count_ones += seed.count_ones(); // Comptabilise les bits à 1 dans chaque seed
        total_bits += 64; // Chaque seed est un u64, donc 64 bits
    }

    let p = count_ones as f64 / total_bits as f64; // Probabilité d'observer un bit à 1
    if p == 0.0 || p == 1.0 {
        0.0 // Cas limite pour éviter log2(0)
    } else {
        // Calcul de l'entropie de Shannon
        -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
    }
}

#[cfg(test)]
mod tests {
    use super::{test_entropy, ENTROPY_TEST_MIN, ENTROPY_TEST_MAX}; // Import des éléments du module parent
    use super::*;
    use rand::Rng;

    /// Génère une série de seeds aléatoires pour les tests.
    fn generate_random_seeds() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| rng.gen::<u64>()).collect()
    }

    /// Teste la fonction d'entropie avec des seeds aléatoires.
    /// Vérifie si le score d'entropie est dans les limites définies par ENTROPY_TEST_MIN et ENTROPY_TEST_MAX.
    #[test]
    fn test_entropy_with_random_seeds() {
        let seeds = generate_random_seeds(); // Génère des seeds pour le test
        let result: TestResult = test_entropy(&seeds); // Exécute le test d'entropie
        println!("Score d'entropie calculé : {:?}", result.score); // Affiche le score pour référence
        assert!(
            result.score >= ENTROPY_TEST_MIN && result.score <= ENTROPY_TEST_MAX,
            "Échec du test d'entropie : score = {:?}",
            result.score
        );
    }
}
