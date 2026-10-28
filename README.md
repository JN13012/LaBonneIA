# LaBonneIA

Future plateforme d’assistants vocaux IA personnalisables. Maria sera l’agent
par défaut ; le nom, la voix, la personnalité et les capacités pourront ensuite
être personnalisés. La première étape produit est **LaBonneIA Studio**.

## Disponible — M0

- Page d’accueil statique React + TypeScript + Tailwind CSS.
- API Rust / Axum : `GET /health` → `{"status":"ok"}`.
- Test automatisé du endpoint et CI Rust / frontend.

La synthèse vocale, le clonage et l’export audio sont futurs. Aucun appel
ElevenLabs, audio généré, compte, base de données ou LLM dans ce jalon.
Le socle M0 est validé localement, y compris Rust et le contrôle HTTP réel de
`/health`. La CI distante reste à exécuter lors d’un push ou d’une PR autorisé.
Voir le [rapport de vérification](docs/VERIFICATION.md).

## Structure et stack

```text
apps/web/                React 19, Vite 7, TypeScript 5.9, Tailwind CSS 4
apps/api/                Rust stable, Axum 0.8, Tokio 1, Serde 1, Reqwest 0.12
  src/main.rs            Démarrage, configuration du port et erreurs
  src/lib.rs             Construction du routeur
  src/routes.rs          Route et handler de santé
  tests/health.rs        Test HTTP en mémoire
.github/workflows/ci.yml Contrôles sur pushes et pull requests
docs/                    Produit, architecture, feuille de route, vérification
```

## Prérequis

Node.js 24 et npm, Git, Rust stable avec Cargo, rustfmt et Clippy. Les outils
système ne sont pas installés automatiquement par le projet.

Si Rust manque sous Linux / WSL, suivre la [procédure officielle rustup](https://rust-lang.org/tools/install/).
À lancer soi-même après lecture du script et de ses options :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/labonneia-rustup-init.sh
sh /tmp/labonneia-rustup-init.sh --default-toolchain stable --profile minimal
. "$HOME/.cargo/env"
rustup component add rustfmt clippy
rustc --version
cargo --version
```

Si le linker C manque, installer les outils de compilation de la distribution
(par exemple `build-essential` sous Ubuntu). Si Node/npm ou Git manquent,
suivre les procédures officielles de [Node.js](https://nodejs.org/en/download)
et [Git](https://git-scm.com/downloads) pour le système utilisé. Aucune de ces
installations n’est effectuée par les scripts du dépôt.

## Développement

Terminal frontend, depuis la racine :

```bash
cd apps/web
npm ci
npm run dev
```

Ouvrir <http://127.0.0.1:5173>. Le port est strict : s’il est occupé, Vite échoue.

Terminal API, depuis la racine :

```bash
cd apps/api
cargo run
```

Puis dans un autre terminal :

```bash
curl --fail --show-error http://127.0.0.1:3000/health
# Attendu : {"status":"ok"}
```

L’API écoute uniquement sur `127.0.0.1`. Pour changer le port :
`API_PORT=3001 cargo run`. Un port invalide ou occupé fait échouer le démarrage
avec un message et un code non nul. Ctrl+C arrête le serveur.

`.env.example` documente les variables ; aucun chargement automatique de `.env`
n’est implémenté. Aucun secret n’est nécessaire pour M0. La future clé
`ELEVENLABS_API_KEY` sera réservée au backend, jamais dans `VITE_*`.

## Validation

Depuis `apps/api` :

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
```

`apps/api/Cargo.lock` est fourni et conservé pour fixer les dépendances Rust.
Les contrôles CI utilisent `--locked` afin de refuser une modification implicite
de cette résolution. Le fichier sera versionné avec le crate lors d’un commit
explicitement autorisé.

Depuis `apps/web` :

```bash
npm ci
npm run typecheck
npm run build
npm run preview
```

La prévisualisation du build est sur <http://127.0.0.1:4173>.
À la racine : `git diff --check`. La CI exécute les quatre contrôles Rust et
les trois commandes d’installation / validation frontend sur pushes et PR.
Aucun push n’est nécessaire pour travailler localement.

## Documents

- [Vision et périmètre V1, V2, V3](docs/PRODUCT.md)
- [Architecture et flux TTS futur](docs/ARCHITECTURE.md)
- [Jalons M0 à M4](docs/ROADMAP.md)
- [Rapport de vérification](docs/VERIFICATION.md)
- [Règles de contribution pour les agents](AGENTS.md)

Les sources vocales, enregistrements et exports privés sont exclus de Git.
Tout futur appel payant requiert une autorisation explicite. M1 ne démarre pas
automatiquement à la fin de M0.
