use crate::types::TestResult; // Importation de TestResult pour structurer les résultats des tests

// Constantes de seuil pour le Test de Corrélation
const CORRELATION_TEST_MIN: f64 = 0.45; // Valeur minimale acceptée pour le passage du test
const CORRELATION_TEST_MAX: f64 = 0.55; // Valeur maximale acceptée pour le passage du test

/// Exécute le test de corrélation en analysant les seeds.
/// `seeds` : Un tableau de nombres de seeds pour le calcul.
/// Retourne : Une structure TestResult contenant les résultats du test.
pub fn test_correlation(seeds: &[u64]) -> TestResult {
    let correlation = calculate_correlation(seeds); // Calcul de la corrélation
    let passed = correlation >= CORRELATION_TEST_MIN && correlation <= CORRELATION_TEST_MAX;

    TestResult {
        test_name: "Test de corrélation des bits".to_string(),
        passed,
        score: correlation,
        details: format!("Taux de corrélation : {:.2}%", correlation * 100.0),
        thresholds: Some((CORRELATION_TEST_MIN, CORRELATION_TEST_MAX)),
    }
}

/// Calcule le taux de corrélation entre les seeds successifs.
/// `seeds` : Tableau de seeds.
/// Retourne : La valeur moyenne de corrélation entre les bits des seeds adjacents.
fn calculate_correlation(seeds: &[u64]) -> f64 {
    let mut pairs = seeds.windows(2); // Génère des paires de seeds consécutifs
    let mut correlation_sum = 0.0;
    let mut count = 0;

    while let Some(&[seed1, seed2]) = pairs.next() {
        let correlation = (seed1 ^ seed2).count_ones() as f64 / 64.0; // Calcul de la différence binaire normalisée
        correlation_sum += correlation;
        count += 1;
    }

    correlation_sum / count as f64 // Moyenne de la corrélation
}

#[cfg(test)]
mod tests {
    use super::{test_correlation, CORRELATION_TEST_MIN, CORRELATION_TEST_MAX}; // Importation des fonctions et constantes du module parent
    use super::*;
    use rand::Rng;

    /// Génère une série de seeds aléatoires pour les tests.
    fn generate_random_seeds() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| rng.gen::<u64>()).collect()
    }

    /// Teste la fonction de corrélation avec des seeds aléatoires.
    /// Vérifie si le score de corrélation est dans les limites définies par CORRELATION_TEST_MIN et CORRELATION_TEST_MAX.
    #[test]
    fn test_correlation_with_random_seeds() {
        let seeds = generate_random_seeds(); // Génère des seeds pour le test
        let result: TestResult = test_correlation(&seeds); // Exécute le test de corrélation
        println!("Score de corrélation calculé : {:?}", result.score); // Affiche le score pour référence
        assert!(
            result.score >= CORRELATION_TEST_MIN && result.score <= CORRELATION_TEST_MAX,
            "Échec du test de corrélation : score = {:?}",
            result.score
        );
    }
}
