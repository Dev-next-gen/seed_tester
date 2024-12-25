#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, tera::Tera};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Status};
use serde_json::json;
use chrono::{Local, Datelike};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use rand::Rng;

// Importation des modules personnalisés
use seed_tester::tests::{
    test_frequence, test_entropy, test_poker, test_periodicity, test_correlation,
    test_sequence_length, test_blocs, test_imprevisibility, test_periodicity_advanced,
    test_collisions, analyze_seed_patterns, analyze_rng_from_file, parse_csv,
};

// Importation conditionnelle pour la génération de PDF
#[cfg(feature = "pdf_export")]
use seed_tester::tests::pdf_generator::generate_pdf_report;

// Importation de la structure des résultats de test
use seed_tester::types::TestResult;

// Structure pour représenter les configurations de test envoyées par le client
#[derive(Serialize, Deserialize, Debug)]
struct ConfigForm {
    num_seeds: usize,
    test_mode: String,
}

// Route pour afficher la page d'accueil
#[get("/")]
fn index_page() -> Template {
    let mut context = HashMap::new();
    context.insert("current_year", json!(Local::now().year()));
    context.insert("app_name", json!("Seed Tester"));
    context.insert("pdf_enabled", json!(cfg!(feature = "pdf_export"))); // Vérification dynamique

    Template::render("index", &context)
}

// Route pour afficher la page de configuration
#[get("/config")]
fn config_page() -> Template {
    let mut context = HashMap::new();
    context.insert("current_year", json!(Local::now().year()));
    context.insert("default_seeds", json!(1000));
    context.insert("default_mode", json!("standard"));

    Template::render("config", &context)
}

// Route pour afficher la page d'exécution des tests RNG
#[get("/run_tests")]
fn run_tests_page() -> Template {
    let mut context = HashMap::new();
    context.insert("current_year", json!(Local::now().year()));
    context.insert("default_seeds", json!(1000));
    context.insert("default_mode", json!("standard"));
    context.insert("pdf_enabled", json!(cfg!(feature = "pdf_export"))); // Vérification dynamique

    Template::render("run_tests", &context)
}

// Route pour exécuter les tests RNG
#[post("/run_tests", data = "<input>")]
async fn run_tests_route(content_type: &ContentType, input: Data<'_>) -> Json<Vec<TestResult>> {
    let data = match input.open(128.kibibytes()).into_string().await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Erreur lors de la lecture des données : {:?}", err);
            return Json(vec![TestResult {
                test_name: "Erreur de lecture".to_string(),
                passed: false,
                score: 0.0,
                details: format!("Erreur : {:?}", err),
                thresholds: None,
            }]);
        }
    };

    let config: Option<ConfigForm> = if content_type.is_json() {
        serde_json::from_str(&data).ok()
    } else {
        None
    };

    if let Some(config) = config {
        if config.num_seeds == 0 {
            return Json(vec![TestResult {
                test_name: "Configuration invalide".to_string(),
                passed: false,
                score: 0.0,
                details: "Le nombre de seeds doit être supérieur à 0.".to_string(),
                thresholds: None,
            }]);
        }

        // Génération des seeds et exécution des tests
        let seeds = generate_seeds(config.num_seeds);
        let mut test_results = vec![
            test_frequence(&seeds),
            test_entropy(&seeds),
            test_poker(&seeds),
            test_periodicity(&seeds),
            test_correlation(&seeds),
            test_sequence_length(&seeds),
            test_blocs(&seeds, 8),
            test_imprevisibility(&seeds),
            test_periodicity_advanced(&seeds),
            test_collisions(&seeds),
        ];

        // Analyse supplémentaire des motifs
        test_results.push(analyze_seed_patterns(&seeds));
        Json(test_results)
    } else {
        Json(vec![TestResult {
            test_name: "Erreur de configuration".to_string(),
            passed: false,
            score: 0.0,
            details: "Erreur dans les données envoyées.".to_string(),
            thresholds: None,
        }])
    }
}

// Route pour analyser un fichier RNG
#[post("/upload_file", data = "<file>")]
async fn upload_file(content_type: &ContentType, file: Data<'_>) -> Json<Vec<TestResult>> {
    let file_content = match file.open(2.mebibytes()).into_string().await {
        Ok(capped) => capped.into_inner(),
        Err(err) => {
            eprintln!("Erreur lors de la lecture du fichier : {:?}", err);
            return Json(vec![TestResult {
                test_name: "Erreur".to_string(),
                passed: false,
                score: 0.0,
                details: format!("Erreur lors de la lecture du fichier : {:?}", err),
                thresholds: None,
            }]);
        }
    };

    if content_type.is_json() {
        let data: Vec<u64> = serde_json::from_str(&file_content).unwrap_or_default();
        Json(analyze_rng_from_file(data))
    } else if content_type.is_plain() || content_type.is_csv() {
        let data = parse_csv(file_content);
        Json(analyze_rng_from_file(data))
    } else {
        Json(vec![TestResult {
            test_name: "Erreur".to_string(),
            passed: false,
            score: 0.0,
            details: "Type de fichier non supporté.".to_string(),
            thresholds: None,
        }])
    }
}

// Route pour exporter les résultats en PDF
#[cfg(feature = "pdf_export")]
#[get("/export_pdf")]
fn export_pdf() -> Result<(ContentType, Vec<u8>), (Status, String)> {
    let test_results = vec![
        TestResult {
            test_name: "Test de Fréquence".to_string(),
            passed: true,
            score: 0.85,
            details: "Détails du test de fréquence".to_string(),
            thresholds: Some((0.5, 1.0)),
        },
        TestResult {
            test_name: "Test d'Entropie".to_string(),
            passed: false,
            score: 0.45,
            details: "Détails du test d'entropie".to_string(),
            thresholds: Some((0.4, 0.6)),
        },
    ];

    match generate_pdf_report(&test_results) {
        Ok(pdf_data) => Ok((ContentType::PDF, pdf_data)),
        Err(err) => {
            eprintln!("Erreur lors de la génération du PDF : {:?}", err);
            Err((Status::InternalServerError, "Impossible de générer le fichier PDF.".to_string()))
        }
    }
}

// Route pour exporter les résultats en CSV
#[get("/export_csv")]
fn export_csv() -> (ContentType, Vec<u8>) {
    let mut wtr = csv::Writer::from_writer(vec![]);

    let test_results = vec![
        TestResult {
            test_name: "Test de Fréquence".to_string(),
            passed: true,
            score: 0.85,
            details: "Détails du test de fréquence".to_string(),
            thresholds: Some((0.5, 1.0)),
        },
        TestResult {
            test_name: "Test d'Entropie".to_string(),
            passed: false,
            score: 0.45,
            details: "Détails du test d'entropie".to_string(),
            thresholds: Some((0.4, 0.6)),
        },
    ];

    wtr.write_record(&["Nom du test", "Résultat", "Score", "Détails", "Seuils Bas", "Seuils Haut"])
        .expect("Erreur d'écriture du CSV");

    for result in test_results {
        wtr.write_record(&[
            result.test_name,
            result.passed.to_string(),
            format!("{:.2}", result.score),
            result.details,
            result.thresholds.map_or("".to_string(), |(low, _)| format!("{:.2}", low)),
            result.thresholds.map_or("".to_string(), |(_, high)| format!("{:.2}", high)),
        ])
        .expect("Erreur d'écriture dans le CSV");
    }

    let data = wtr.into_inner().expect("Erreur lors de la conversion en CSV");
    (ContentType::CSV, data)
}

// Génération de seeds aléatoires
fn generate_seeds(num_seeds: usize) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    (0..num_seeds).map(|_| rng.gen::<u64>()).collect()
}

// Chargement des templates
fn force_load_templates() -> Tera {
    let mut tera = Tera::default();
    tera.add_template_files(vec![
        ("templates/layout_base.html.tera", Some("layout_base")),
        ("templates/index.html.tera", Some("index")),
        ("templates/config.html.tera", Some("config")),
        ("templates/run_tests.html.tera", Some("run_tests")),
    ])
    .expect("Erreur de chargement des templates.");

    tera.autoescape_on(vec!["html.tera"]);
    tera
}

// Lancement de l'application Rocket
#[launch]
fn rocket() -> _ {
    let mut app = rocket::build()
        .attach(Template::custom(|engines| {
            engines.tera = force_load_templates();
        }))
        .mount(
            "/",
            routes![
                index_page,
                config_page,
                run_tests_page,
                run_tests_route,
                upload_file,
                export_csv,
            ],
        )
        .mount("/static", rocket::fs::FileServer::from("./static"));

    #[cfg(feature = "pdf_export")]
    {
        app = app.mount("/", routes![export_pdf]);
    }

    app
}
