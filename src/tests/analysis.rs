use crate::types::TestResult;

/// Analyse des motifs dans les seeds.
/// `seeds` : Un tableau de seeds à analyser.
/// Retourne : Une structure `TestResult` décrivant les motifs trouvés.
pub fn analyze_seed_patterns(seeds: &[u64]) -> TestResult {
    let mut pattern_counts = std::collections::HashMap::new();
    for &seed in seeds {
        let pattern = seed & 0xF; // Analyse sur les 4 derniers bits
        *pattern_counts.entry(pattern).or_insert(0) += 1;
    }

    let unique_patterns = pattern_counts.len();
    let max_count = pattern_counts.values().cloned().max().unwrap_or(0);
    let bias_score = max_count as f64 / seeds.len() as f64;

    TestResult {
        test_name: "Analyse des motifs".to_string(),
        passed: unique_patterns > 10 && bias_score < 0.2,
        score: bias_score,
        details: format!(
            "Motifs uniques : {}, Score de biais : {:.2}",
            unique_patterns, bias_score
        ),
        thresholds: Some((0.0, 0.2)),
    }
}
