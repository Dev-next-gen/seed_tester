#[cfg(feature = "pdf_export")]
use printpdf::*;
#[cfg(feature = "pdf_export")]
use std::fs::File;
#[cfg(feature = "pdf_export")]
use std::io::{BufWriter, Cursor};
use crate::types::TestResult;

/// Génère un rapport PDF basé sur les résultats des tests RNG.
/// `test_results` : Une référence à un vecteur contenant les résultats des tests.
/// Retourne : Un `Vec<u8>` représentant les données du fichier PDF ou une erreur.
#[cfg(feature = "pdf_export")]
pub fn generate_pdf_report(test_results: &[TestResult]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Création d'un nouveau document PDF
    let (doc, page1, layer1) = PdfDocument::new("Rapport de Tests RNG", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Ajout d'une police
    let font_path = "assets/fonts/Roboto-Regular.ttf"; // Chemin vers la police
    let font = doc.add_external_font(File::open(font_path)?)?;

    // Ajout du titre
    current_layer.use_text("Rapport de Tests RNG", 20.0, Mm(10.0), Mm(280.0), &font);

    // Ajout des résultats des tests
    for (i, result) in test_results.iter().enumerate() {
        let y_offset = 260.0 - (i as f64 * 10.0); // Décalage vertical
        current_layer.use_text(
            format!(
                "{} - Résultat : {} - Score : {:.2}",
                result.test_name,
                if result.passed { "Réussi" } else { "Échoué" },
                result.score
            ),
            12.0,
            Mm(10.0),
            Mm(y_offset),
            &font,
        );
    }

    // Sauvegarde le document dans un buffer
    let mut buffer = Cursor::new(Vec::new());
    doc.save(&mut BufWriter::new(&mut buffer))?;
    Ok(buffer.into_inner()) // Retourne les données PDF
}

/// Stub pour `generate_pdf_report` si `pdf_export` n'est pas activé.
#[cfg(not(feature = "pdf_export"))]
pub fn generate_pdf_report(_test_results: &[TestResult]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Err("La génération PDF n'est pas activée. Activez la fonctionnalité 'pdf_export' pour utiliser cette fonction.".into())
}
