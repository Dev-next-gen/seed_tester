use crate::types::TestResult;

// Seuils pour le test de périodicité
const PERIODICITY_THRESHOLD: usize = 2;

/// Test avancé de périodicité pour détecter les motifs répétitifs.
/// `seeds` : Tableau de seeds.
/// Retourne : Une structure TestResult avec les résultats du test.
pub fn test_periodicity_advanced(seeds: &[u64]) -> TestResult {
    let (max_repeats, unique_patterns) = calculate_pattern_repeats(seeds);
    let passed = max_repeats <= PERIODICITY_THRESHOLD;

    TestResult {
        test_name: "Test de Périodicité Avancé".to_string(),
        passed,
        score: max_repeats as f64,
        details: format!(
            "Motifs uniques : {}, Répétitions maximales : {}, Seuil : {}",
            unique_patterns, max_repeats, PERIODICITY_THRESHOLD
        ),
        thresholds: Some((0.0, PERIODICITY_THRESHOLD as f64)),
    }
}

/// Calcule le nombre maximum de répétitions et le nombre de motifs uniques.
/// `seeds` : Tableau de seeds.
/// Retourne : Un tuple contenant le nombre maximal de répétitions et le nombre de motifs uniques.
fn calculate_pattern_repeats(seeds: &[u64]) -> (usize, usize) {
    use std::collections::HashMap;

    let mut pattern_counts = HashMap::new();
    for window in seeds.windows(3) {
        let pattern = format!("{:?}", window);
        *pattern_counts.entry(pattern).or_insert(0) += 1;
    }

    let max_repeats = *pattern_counts.values().max().unwrap_or(&0);
    (max_repeats, pattern_counts.len())
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
    fn test_periodicity_advanced_with_random_seeds() {
        let seeds = generate_random_seeds();
        let result = test_periodicity_advanced(&seeds);
        println!(
            "Score de périodicité calculé : {:?}",
            result.score
        );
        assert!(result.passed, "Échec du test de périodicité avancé : score = {:?}", result.score);
    }
}
