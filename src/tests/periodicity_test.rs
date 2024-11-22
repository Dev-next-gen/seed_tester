use crate::types::TestResult; // Importation de TestResult pour structurer les résultats des tests

// Constantes pour le Test de Périodicité, définissant les seuils de réussite
const PERIODICITY_TEST_MIN: f64 = 0.3; // Taux de répétition minimal acceptable
const PERIODICITY_TEST_MAX: f64 = 0.7; // Taux de répétition maximal acceptable

/// Exécute le test de périodicité des bits dans les seeds.
/// `seeds` : Un tableau de seeds de type u64.
/// Retourne : Une structure TestResult avec les résultats du test de périodicité.
pub fn test_periodicity(seeds: &[u64]) -> TestResult {
    let periodicity = calculate_periodicity(seeds); // Calcul de la périodicité des bits
    let passed = periodicity >= PERIODICITY_TEST_MIN && periodicity <= PERIODICITY_TEST_MAX;

    TestResult {
        test_name: "Test de périodicité des bits".to_string(),
        passed,
        score: periodicity,
        details: format!("Taux de répétition : {:.2}%", periodicity * 100.0),
        thresholds: Some((PERIODICITY_TEST_MIN, PERIODICITY_TEST_MAX)),
    }
}

/// Calcule la périodicité des bits '1' dans les seeds.
/// `seeds` : Tableau de seeds.
/// Retourne : Le taux de répétition des bits entre les seeds successifs.
fn calculate_periodicity(seeds: &[u64]) -> f64 {
    let mut repeats = 0;
    let mut total = 0;

    for i in 0..seeds.len() - 1 {
        let bits_in_common = (seeds[i] ^ seeds[i + 1]).count_zeros();
        repeats += bits_in_common;
        total += 64;
    }

    if total == 0 {
        0.0
    } else {
        repeats as f64 / total as f64
    }
}
