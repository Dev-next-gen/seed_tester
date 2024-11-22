pub mod frequency_test;
pub mod periodicity_test;
pub mod sequence_length_test;
pub mod entropy_test;
pub mod correlation_test;
pub mod poker_test;
pub mod block_test;
pub mod imprevisibility_test;
pub mod periodicity_advanced_test;
pub mod collision_test;
pub mod pdf_generator;
pub mod analysis;
pub mod file_analysis;
pub mod file_parser;

// Regroupement logique des exports pour une meilleure lisibilité
// Export des fonctions de tests RNG
pub use self::frequency_test::test_frequence;
pub use self::periodicity_test::test_periodicity;
pub use self::sequence_length_test::test_sequence_length;
pub use self::entropy_test::test_entropy;
pub use self::correlation_test::test_correlation;
pub use self::poker_test::test_poker;
pub use self::block_test::test_blocs;
pub use self::imprevisibility_test::test_imprevisibility;
pub use self::periodicity_advanced_test::test_periodicity_advanced;
pub use self::collision_test::test_collisions;

// Export des outils supplémentaires
pub use self::pdf_generator::generate_pdf_report;
pub use self::analysis::analyze_seed_patterns;
pub use self::file_analysis::analyze_rng_from_file;
pub use self::file_parser::parse_csv;
