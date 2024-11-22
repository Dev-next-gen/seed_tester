use crate::types::TestResult; // Importation de TestResult pour structurer les résultats des tests

// Constantes pour le Test de Fréquence, définissant les seuils de réussite
const FREQUENCY_TEST_MIN: f64 = 0.45; // Fréquence minimale acceptable des '1'
const FREQUENCY_TEST_MAX: f64 = 0.55; // Fréquence maximale acceptable des '1'

/// Exécute le test de fréquence des bits dans les seeds.
/// `seeds` : Un tableau de seeds de type u64.
/// Retourne : Une structure TestResult avec les résultats du test de fréquence.
pub fn test_frequence(seeds: &[u64]) -> TestResult {
    let frequency = calculate_frequency(seeds); // Calcul de la fréquence des '1'
    let passed = frequency >= FREQUENCY_TEST_MIN && frequency <= FREQUENCY_TEST_MAX;

    TestResult {
        test_name: "Test de fréquence des bits".to_string(),
        passed,
        score: frequency,
        details: format!("Fréquence des '1' : {:.2}%", frequency * 100.0),
        thresholds: Some((FREQUENCY_TEST_MIN, FREQUENCY_TEST_MAX)),
    }
}

/// Calcule la fréquence des bits '1' dans les seeds.
/// `seeds` : Tableau de seeds.
/// Retourne : La fréquence des '1' calculée.
fn calculate_frequency(seeds: &[u64]) -> f64 {
    // Somme des bits '1' dans chaque seed, divisé par le nombre total de bits
    let count = seeds.iter().map(|seed| seed.count_ones() as u64).sum::<u64>();
    count as f64 / (seeds.len() * 64) as f64
}

#[cfg(test)]
mod tests {
    use super::*; // Importe tous les éléments nécessaires du module parent
    use rand::Rng; // Import utilisé uniquement pour les tests

    /// Génère une série de seeds aléatoires pour les tests.
    fn generate_random_seeds() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| rng.gen::<u64>()).collect()
    }

    /// Teste la fonction de fréquence avec des seeds aléatoires.
    /// Vérifie que le score de fréquence est dans les limites définies par FREQUENCY_TEST_MIN et FREQUENCY_TEST_MAX.
    #[test]
    fn test_frequency_with_random_seeds() {
        let seeds = generate_random_seeds(); // Génère des seeds pour le test
        let result = test_frequence(&seeds); // Exécute le test de fréquence
        println!("Score de fréquence calculé : {:?}", result.score); // Affiche le score pour référence
        assert!(result.passed, "Échec du test de fréquence : score = {:?}", result.score);
    }
}
