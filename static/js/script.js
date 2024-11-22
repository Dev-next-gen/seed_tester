document.addEventListener('DOMContentLoaded', function () {
    // Boutons et formulaires
    const refreshButton = document.getElementById('refresh-button');
    const downloadButton = document.getElementById('download-btn');
    const configForm = document.getElementById('config-form');
    const analyzeForm = document.getElementById('analyze-form');
    const uploadFileForm = document.getElementById('upload-file-form');
    const loader = document.getElementById('loader');
    const alertBox = document.getElementById('alert');

    // Conteneurs de résultats
    const resultsTable = document.getElementById('results-table');
    const fileResultsContainer = document.getElementById('file-results-container');

    /**
     * Affiche une alerte utilisateur.
     * @param {string} message - Message à afficher.
     * @param {string} type - Type d'alerte ('success', 'info', 'error').
     */
    const showAlert = (message, type) => {
        if (alertBox) {
            alertBox.textContent = message;
            alertBox.className = `alert ${
                type === 'success'
                    ? 'bg-green-600'
                    : type === 'info'
                    ? 'bg-blue-600'
                    : 'bg-red-600'
            } text-white fixed top-4 right-4 p-4 max-w-xs rounded-lg shadow-lg`;
            alertBox.classList.remove('hidden');
            setTimeout(() => alertBox.classList.add('hidden'), 3000);
        } else {
            console.warn("Alerte : élément 'alertBox' introuvable.");
        }
    };

    /**
     * Active ou désactive le loader.
     * @param {boolean} show - `true` pour afficher le loader, `false` pour le cacher.
     */
    const toggleLoader = (show) => {
        if (loader) {
            loader.style.display = show ? 'flex' : 'none';
        } else {
            console.warn("Alerte : élément 'loader' introuvable.");
        }
    };

    /**
     * Met à jour un conteneur avec les résultats des tests RNG.
     * @param {Array} results - Résultats à afficher.
     * @param {string} containerId - ID du conteneur de destination.
     */
    const updateResults = (results, containerId = 'results-table') => {
        const container = document.getElementById(containerId);
        if (!container) {
            console.warn(`Alerte : conteneur des résultats '${containerId}' introuvable.`);
            return;
        }

        if (!results || results.length === 0) {
            container.innerHTML = `<p class="text-gray-400">Aucun résultat disponible.</p>`;
            return;
        }

        const tableRows = results
            .map(
                (result) => `
                <tr class="${result.passed ? 'bg-green-900' : 'bg-red-900'} hover:bg-gray-700">
                    <td class="p-2 border-b border-gray-700">${result.test_name || 'Inconnu'}</td>
                    <td class="p-2 border-b border-gray-700">${result.passed ? 'Réussi' : 'Échoué'}</td>
                    <td class="p-2 border-b border-gray-700">${parseFloat(result.score || 0).toFixed(2)}</td>
                </tr>`
            )
            .join('');

        container.innerHTML = `
            <table class="w-full border-collapse mt-4">
                <thead>
                    <tr>
                        <th class="border-b-2 border-gray-700 p-2">Nom du test</th>
                        <th class="border-b-2 border-gray-700 p-2">Résultat</th>
                        <th class="border-b-2 border-gray-700 p-2">Score</th>
                    </tr>
                </thead>
                <tbody>${tableRows}</tbody>
            </table>
        `;
    };

    // Gestion du formulaire de configuration pour lancer les tests
    if (configForm) {
        configForm.addEventListener('submit', async (event) => {
            event.preventDefault();
            const formData = new FormData(configForm);
            const json = Object.fromEntries(formData.entries());

            if (!json.num_seeds || isNaN(json.num_seeds) || parseInt(json.num_seeds) <= 0) {
                showAlert('Le nombre de seeds est invalide. Veuillez entrer un nombre valide.', 'error');
                return;
            }

            if (!['standard', 'advanced', 'expert'].includes(json.test_mode)) {
                showAlert('Le mode de test est invalide. Veuillez sélectionner un mode valide.', 'error');
                return;
            }

            toggleLoader(true);

            try {
                const response = await fetch('/run_tests', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(json),
                });

                if (!response.ok) throw new Error(`Erreur HTTP : ${response.status}`);
                const results = await response.json();
                updateResults(results);
                showAlert('Tests exécutés avec succès.', 'success');
            } catch (error) {
                console.error('Erreur lors de l\'exécution des tests :', error);
                showAlert('Erreur lors de l\'exécution des tests.', 'error');
            } finally {
                toggleLoader(false);
            }
        });
    }

    // Gestion du formulaire pour l'analyse de fichier
    if (uploadFileForm) {
        uploadFileForm.addEventListener('submit', async (event) => {
            event.preventDefault();
            const fileInput = document.getElementById('rng_file');

            if (!fileInput || !fileInput.files.length) {
                showAlert('Veuillez sélectionner un fichier.', 'error');
                return;
            }

            const formData = new FormData();
            formData.append('file', fileInput.files[0]);

            toggleLoader(true);

            try {
                const response = await fetch('/upload_file', {
                    method: 'POST',
                    body: formData,
                });

                if (!response.ok) throw new Error(`Erreur HTTP : ${response.status}`);
                const results = await response.json();
                updateResults(results, 'file-results-container');
                showAlert('Analyse du fichier réussie.', 'success');
            } catch (error) {
                console.error('Erreur lors de l\'analyse du fichier :', error);
                showAlert('Erreur lors de l\'analyse du fichier.', 'error');
            } finally {
                toggleLoader(false);
            }
        });
    }

    // Gestion du téléchargement CSV
    if (downloadButton) {
        downloadButton.addEventListener('click', async () => {
            try {
                const response = await fetch('/export_csv');
                if (!response.ok) throw new Error(`Erreur HTTP : ${response.status}`);
                const data = await response.text();

                const blob = new Blob([data], { type: 'text/csv' });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'resultats.csv';
                document.body.appendChild(a);
                a.click();
                document.body.removeChild(a);

                showAlert('Téléchargement du CSV réussi.', 'success');
            } catch (error) {
                console.error('Erreur lors du téléchargement CSV :', error);
                showAlert('Erreur lors du téléchargement CSV.', 'error');
            }
        });
    }

    // Gestion du rafraîchissement des résultats
    if (refreshButton) {
        refreshButton.addEventListener('click', async () => {
            toggleLoader(true);

            try {
                const response = await fetch('/refresh_results');
                if (!response.ok) throw new Error(`Erreur HTTP : ${response.status}`);
                const results = await response.json();
                updateResults(results);
                showAlert('Résultats rafraîchis avec succès.', 'success');
            } catch (error) {
                console.error('Erreur lors du rafraîchissement :', error);
                showAlert('Erreur lors du rafraîchissement des résultats.', 'error');
            } finally {
                toggleLoader(false);
            }
        });
    }
});
