use crate::types::TestResult;

// Seuils pour le test d'imprévisibilité
const IMPREVISIBILITY_THRESHOLD: f64 = 0.8;

/// Teste l'imprévisibilité des seeds.
/// `seeds` : Tableau de seeds.
/// Retourne : Une structure TestResult avec les résultats du test.
pub fn test_imprevisibility(seeds: &[u64]) -> TestResult {
    let entropy = calculate_entropy(seeds);
    let passed = entropy > IMPREVISIBILITY_THRESHOLD;

    TestResult {
        test_name: "Test d'Imprévisibilité".to_string(),
        passed,
        score: entropy,
        details: format!(
            "Entropie calculée : {:.2}, Seuil requis : {:.2}",
            entropy, IMPREVISIBILITY_THRESHOLD
        ),
        thresholds: Some((IMPREVISIBILITY_THRESHOLD, 1.0)),
    }
}

/// Calcule l'entropie des différences consécutives dans la séquence.
/// `seeds` : Tableau de seeds.
/// Retourne : La valeur de l'entropie.
fn calculate_entropy(seeds: &[u64]) -> f64 {
    use std::collections::HashSet;

    let mut consecutive_differences = Vec::new();
    for window in seeds.windows(2) {
        if let [a, b] = window {
            // Vérifie et corrige les dépassements liés à la soustraction
            if let Some(diff) = b.checked_sub(*a) {
                consecutive_differences.push(diff);
            } else {
                consecutive_differences.push(a - b); // Si overflow, soustraction inverse
            }
        }
    }

    let unique_values: HashSet<_> = consecutive_differences.iter().collect();
    unique_values.len() as f64 / seeds.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn generate_random_seeds() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| rng.gen::<u64>()).collect()
    }

    #[test]
    fn test_imprevisibility_with_random_seeds() {
        let seeds = generate_random_seeds();
        let result = test_imprevisibility(&seeds);
        println!("Score d'imprévisibilité calculé : {:?}", result.score);
        assert!(result.passed, "Échec du test d'imprévisibilité : score = {:?}", result.score);
    }
}
