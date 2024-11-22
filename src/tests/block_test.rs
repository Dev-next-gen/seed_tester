use crate::types::TestResult;

// Seuils pour le test de blocs
const CHI_SQUARE_THRESHOLD: f64 = 25.0; // Seuil ajusté pour tolérer plus de variations
const BLOCK_SIZE: usize = 4; // Taille des blocs pour l'analyse

/// Test de blocs pour vérifier la répartition des combinaisons.
/// `seeds` : Tableau de seeds.
/// `block_size` : Taille des blocs.
/// Retourne : Une structure TestResult avec les résultats du test.
pub fn test_blocs(seeds: &[u64], block_size: usize) -> TestResult {
    let chi_square_score = calculate_chi_square(seeds, block_size);
    let passed = chi_square_score <= CHI_SQUARE_THRESHOLD;

    TestResult {
        test_name: "Test de blocs".to_string(),
        passed,
        score: chi_square_score,
        details: format!(
            "Score Chi-square : {:.2}, Seuil requis : {:.2}",
            chi_square_score, CHI_SQUARE_THRESHOLD
        ),
        thresholds: Some((0.0, CHI_SQUARE_THRESHOLD)),
    }
}

/// Calcule le score Chi-square pour les blocs.
/// `seeds` : Tableau de seeds.
/// `block_size` : Taille des blocs.
/// Retourne : Le score Chi-square.
fn calculate_chi_square(seeds: &[u64], block_size: usize) -> f64 {
    let mut counts = vec![0; 2_usize.pow(block_size as u32)];

    for &seed in seeds {
        let mut value = seed;
        for _ in 0..(64 / block_size) {
            let block = (value & ((1 << block_size) - 1)) as usize;
            counts[block] += 1;
            value >>= block_size;
        }
    }

    let total_blocks = counts.iter().sum::<usize>() as f64;
    let expected_count = total_blocks / counts.len() as f64;

    counts
        .iter()
        .map(|&count| {
            let diff = count as f64 - expected_count;
            (diff * diff) / expected_count
        })
        .sum()
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
    fn test_block_with_random_seeds() {
        let seeds = generate_random_seeds();
        let result = test_blocs(&seeds, BLOCK_SIZE);
        println!("Score Chi-square calculé : {:?}", result.score);
        assert!(
            result.passed,
            "Échec du test de blocs : score = {:?}",
            result.score
        );
    }
}
