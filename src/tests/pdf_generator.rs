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
    println!("Début de la génération du PDF...");

    // Chemin absolu vers la police
    let font_path = "assets/fonts/Roboto-Regular.ttf"; // Chemin corrigé
    println!("Chargement de la police depuis : {}", font_path);

    // Création d'un nouveau document PDF
    let (doc, page1, layer1) = PdfDocument::new("Rapport de Tests RNG", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Chargement de la police
    let font = doc.add_external_font(File::open(font_path)?)?;
    println!("Police chargée avec succès.");

    // Dessin d'une bordure
    draw_border(&current_layer, Mm(10.0), Mm(10.0), Mm(200.0), Mm(287.0));

    // Titre centré
    current_layer.set_font(&font, 24.0);
    current_layer.use_text(
        "Rapport de Tests RNG",
        24.0,
        Mm(60.0),
        Mm(270.0),
        &font,
    );

    // En-tête du tableau
    current_layer.set_font(&font, 16.0);
    current_layer.use_text("Résultats des Tests RNG", 16.0, Mm(10.0), Mm(250.0), &font);

    // Ligne pour démarquer l’en-tête
    draw_line(&current_layer, Mm(10.0), Mm(248.0), Mm(200.0), Mm(248.0));

    // Affichage des résultats dans un tableau
    let mut y_offset = 240.0;
    for result in test_results.iter() {
        let color = if result.passed {
            Color::Rgb(Rgb::new(0.0, 0.5, 0.0, None)) // Vert pour réussi
        } else {
            Color::Rgb(Rgb::new(0.5, 0.0, 0.0, None)) // Rouge pour échoué
        };

        // Nom du test
        current_layer.set_font(&font, 12.0);
        current_layer.set_fill_color(color.clone());
        current_layer.use_text(&result.test_name, 12.0, Mm(12.0), Mm(y_offset), &font);

        // Résultat
        current_layer.use_text(
            if result.passed { "Réussi" } else { "Échoué" },
            12.0,
            Mm(80.0),
            Mm(y_offset),
            &font,
        );

        // Score
        current_layer.use_text(
            format!("{:.2}", result.score),
            12.0,
            Mm(150.0),
            Mm(y_offset),
            &font,
        );

        y_offset -= 10.0; // Décalage vertical pour la prochaine ligne
    }

    println!("Ajout des résultats terminé. Sauvegarde du PDF...");

    // Sauvegarde le document dans un buffer
    let mut buffer = Cursor::new(Vec::new());
    doc.save(&mut BufWriter::new(&mut buffer))?;
    println!("PDF généré avec succès !");
    Ok(buffer.into_inner())
}

/// Stub pour `generate_pdf_report` si `pdf_export` n'est pas activé.
#[cfg(not(feature = "pdf_export"))]
pub fn generate_pdf_report(_test_results: &[TestResult]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Err("La génération PDF n'est pas activée. Activez la fonctionnalité 'pdf_export' pour utiliser cette fonction.".into())
}

/// Dessine une bordure rectangulaire sur le PDF
#[cfg(feature = "pdf_export")]
fn draw_border(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y1), false),
        (Point::new(x2, y2), false),
        (Point::new(x1, y2), false),
        (Point::new(x1, y1), false),
    ];

    let line = Line {
        points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    layer.add_shape(line);
}

/// Dessine une ligne horizontale
#[cfg(feature = "pdf_export")]
fn draw_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y2), false),
    ];

    let line = Line {
        points,
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    layer.add_shape(line);
}
