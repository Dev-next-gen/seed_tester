use serde::{Serialize, Deserialize}; // Assurez-vous que `Deserialize` est inclus

#[derive(Serialize, Deserialize, Debug)] // Ajout de `Debug` pour le débogage
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub score: f64,
    pub details: String,
    pub thresholds: Option<(f64, f64)>, // Ajout du champ thresholds
}
