use crate::types::TestResult;

pub fn generate_report(results: &[TestResult]) {
    for result in results {
        println!("Test: {}, Passed: {}, Score: {:.2}, Details: {}", 
                 result.test_name, result.passed, result.score, result.details);
    }
}
