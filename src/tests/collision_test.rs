use crate::types::TestResult; // Import nécessaire

/// Seuil de collision maximal acceptable (en pourcentage)
const COLLISION_THRESHOLD: f64 = 1.0; // 1 % de collisions maximum

/// Test de collision pour vérifier si le RNG produit des valeurs uniques.
/// # Arguments
/// * `seeds` - Tableau de seeds générés par le RNG.
/// 
/// # Retourne
/// Une structure `TestResult` avec :
/// - `test_name` : Le nom du test.
/// - `passed` : Un booléen indiquant si le test est réussi.
/// - `score` : Le taux de collision calculé.
/// - `details` : Une chaîne contenant les informations détaillées.
/// - `thresholds` : Les seuils minimum et maximum pour le test.
pub fn test_collisions(seeds: &[u64]) -> TestResult {
    use std::collections::HashSet;

    if seeds.is_empty() {
        return TestResult {
            test_name: "Test de Collision".to_string(),
            passed: false,
            score: 100.0, // 100 % de collision dans le cas d'une entrée vide
            details: "Erreur : Aucun seed fourni pour le test.".to_string(),
            thresholds: Some((0.0, COLLISION_THRESHOLD)),
        };
    }

    // Calcul des collisions
    let unique_values: HashSet<_> = seeds.iter().cloned().collect();
    let num_unique = unique_values.len();
    let num_total = seeds.len();

    let collision_rate = 100.0 - (num_unique as f64 / num_total as f64) * 100.0;
    let passed = collision_rate <= COLLISION_THRESHOLD;

    TestResult {
        test_name: "Test de Collision".to_string(),
        passed,
        score: collision_rate,
        details: format!(
            "Taux de collision : {:.2} %, Nombre total : {}, Uniques : {}, Seuil maximal : {:.2} %",
            collision_rate, num_total, num_unique, COLLISION_THRESHOLD
        ),
        thresholds: Some((0.0, COLLISION_THRESHOLD)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collisions_with_no_collision() {
        let seeds = vec![1, 2, 3, 4, 5];
        let result = test_collisions(&seeds);
        assert!(result.passed);
        assert_eq!(result.score, 0.0);
        println!("{}", result.details);
    }

    #[test]
    fn test_collisions_with_some_collisions() {
        let seeds = vec![1, 2, 2, 3, 4, 5, 5];
        let result = test_collisions(&seeds);
        assert!(!result.passed);
        assert!(result.score > 0.0);
        println!("{}", result.details);
    }

    #[test]
    fn test_collisions_with_empty_input() {
        let seeds: Vec<u64> = vec![];
        let result = test_collisions(&seeds);
        assert!(!result.passed);
        assert_eq!(result.score, 100.0);
        println!("{}", result.details);
    }
}
