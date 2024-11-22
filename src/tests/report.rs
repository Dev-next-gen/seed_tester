use super::TestResult;

pub fn generate_report(test_results: &[TestResult]) {
    let mut passed_tests = 0;
    let mut total_score = 0.0;

    println!("===== Rapport des Tests de Seed =====");
    for result in test_results {
        println!("\nTest : {}", result.test_name);
        println!("Passé : {}", if result.passed { "Oui" } else { "Non" });
        println!("Score : {:.2}", result.score);
        println!("Détails : {}", result.details);

        if result.passed {
            passed_tests += 1;
        }
        total_score += result.score;
    }

    let average_score = total_score / test_results.len() as f64;
    let success_rate = (passed_tests as f64 / test_results.len() as f64) * 100.0;

    println!("\n===== Statistiques Globales =====");
    println!("Nombre de tests réussis : {}", passed_tests);
    println!("Taux de réussite : {:.2}%", success_rate);
    println!("Score moyen : {:.2}", average_score);
}
