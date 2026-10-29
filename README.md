# LaBonneIA

Future plateforme d’assistants vocaux IA personnalisables. Maria sera l’agent
par défaut ; le premier produit est **LaBonneIA Studio**.

## Disponible — M1 : Studio simulé

- Éditeur de texte avec compteur et limite de 2 000 caractères Unicode.
- Deux voix **fictives**, vitesse, stabilité et similarité de démonstration.
- Envoi au backend, validation, chargement et erreurs compréhensibles.
- Écoute et téléchargement d’un WAV de test : **deux bips fixes, aucune parole**.
- `GET /health` conservé ; `POST /api/tts/generate` ajouté et testé.

Le texte, la voix et les réglages ne changent pas le signal. Aucun clone vocal,
appel ElevenLabs, clé API, LLM, STT, compte, base de données ou stockage audio
serveur. L’audio reste en mémoire ; le téléchargement est une action explicite
de l’utilisateur. Le portfolio reste un projet distinct.

M0 a été publié sur `main` à `3366112699970637bc92dbf151439655ddd2bc0d`.
Les résultats M1 réellement exécutés sont dans [VERIFICATION.md](docs/VERIFICATION.md).

## Structure

```text
apps/web/                React, TypeScript, Vite, Tailwind CSS
  src/App.tsx            Formulaire, états, lecteur et cycle de vie des Object URLs
  src/tts.ts             Contrat client, requête HTTP et contrôle du WAV reçu
  src/index.css          Présentation responsive et réduction des animations
apps/api/                Un seul crate Rust : Axum, Tokio, Serde
  src/main.rs            Démarrage sur la boucle locale et arrêt gracieux
  src/lib.rs             Construction du routeur
  src/routes.rs          HTTP, erreurs JSON, limite de corps et réponses audio
  src/tts.rs             Contrat typé, validation et service de génération
  src/tts/mock.rs        Générateur WAV sans réseau ni fichier
  tests/                 Tests des routes santé et TTS
.github/workflows/ci.yml Contrôles Rust et React sur pushes et pull requests
docs/                    Produit, architecture, feuille de route, vérification
```

## Démarrer localement

Prérequis : Node.js 24 et npm, Git, Rust stable avec Cargo, rustfmt et Clippy.
Aucun outil système n’est installé automatiquement.

Terminal API :

```bash
cd /home/cyber/work/LaBonneIA/apps/api
# Si nécessaire dans WSL : . "$HOME/.cargo/env"
cargo run --locked
```

Terminal frontend :

```bash
cd /home/cyber/work/LaBonneIA/apps/web
npm ci
npm run dev
```

Ouvrir <http://127.0.0.1:5173>, modifier le texte et les réglages, cliquer sur
**Tester la génération**, puis écouter les deux bips et télécharger le WAV.
Le lecteur ne démarre pas automatiquement. Modifier une entrée efface le
résultat précédent. L’interface abandonne une requête après 15 secondes.

Les serveurs écoutent exclusivement sur `127.0.0.1`. Vite relaie `/api` vers
`http://127.0.0.1:3000` sans CORS permissif. Les ports frontend sont stricts :
si 5173 est occupé, choisir `npm run dev -- --port 5174`. Ne pas arrêter un
serveur préexistant. `API_PORT=3001 cargo run --locked` change le port backend ;
il faut alors adapter la cible du proxy dans `apps/web/vite.config.ts`.
Ctrl+C arrête chaque serveur.

Aucun `.env` ni secret n’est nécessaire. `.env.example` documente les variables
futures, sans chargement automatique. Une future clé fournisseur restera
exclusivement côté backend, jamais dans `VITE_*`.

## Contrôle HTTP

```bash
curl --fail --show-error http://127.0.0.1:3000/health
# {"status":"ok"}

# Via le frontend et son proxy. Le fichier reste hors du dépôt.
curl --fail --show-error http://127.0.0.1:5173/api/tts/generate \
  -H 'Content-Type: application/json' \
  -d '{"text":"Bonjour","voice_id":"mock-alba","speed":1,"stability":0.5,"similarity":0.75}' \
  -o /tmp/labonneia-test.wav
```

Contrat, erreurs et limites détaillés dans [ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Validation

Depuis `apps/api` :

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
```

Depuis `apps/web` :

```bash
npm ci
npm run typecheck
npm run build
npm run preview
```

La prévisualisation est sur <http://127.0.0.1:4173>. Le build statique ne
fournit pas lui-même de serveur API ; un futur déploiement nécessitera une
origine commune avec routage `/api`. M1 ne livre pas de déploiement public.

À la racine : `git diff --check`. Les deux lockfiles sont conservés.
La CI exécute les contrôles Rust et frontend. Les exécutions du commit publié
sont consultables dans [GitHub Actions](https://github.com/JN13012/LaBonneIA/actions/workflows/ci.yml).
Le rapport distingue les validations locales du résultat des jobs distants.

## Documents et suite

- [Produit disponible et vision future](docs/PRODUCT.md)
- [Architecture et contrat HTTP](docs/ARCHITECTURE.md)
- [Feuille de route](docs/ROADMAP.md)
- [Rapport de vérification](docs/VERIFICATION.md)
- [Règles de travail](AGENTS.md)

M1 s’arrête au Mock. M2, tout appel payant et toute utilisation d’une voix
réelle nécessitent une demande et les autorisations adaptées. La clôture M1
comprend une publication sur `origin/main` explicitement autorisée ; elle ne
déclenche pas M2.
