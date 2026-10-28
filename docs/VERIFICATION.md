# Vérification M0.1 — 28 octobre 2026

## État

**PASS : tous les critères M0 vérifiables localement sont validés.** Le blocage
Rust du premier passage M0 est levé : compilation, qualité, test automatisé et
requête HTTP réelle sur l’API démarrée passent. Le frontend est revérifié.
La configuration CI est vérifiée statiquement et ses commandes passent en local ;
son exécution sur GitHub reste non vérifiée, sans push ni PR autorisé.

Aucune fonctionnalité M1, aucun appel ElevenLabs, audio, commit ou push.

## Inspection et versions

- Branche `main`, remote `git@github.com:JN13012/LaBonneIA.git`.
- Les fichiers M0 étaient déjà modifiés / non suivis avant M0.1 ; ce travail est
  conservé. Aucun changement de branche ni ajout à l’index Git.
- Rust `1.99.1 (58cf8f9c3 2026-10-13)` et Cargo
  `1.99.1 (5f94df478 2026-08-27)`, canal `stable-x86_64-unknown-linux-gnu`.
- Le shell de l’agent n’avait pas hérité du nouveau PATH. L’ajout temporaire
  `export PATH="/home/cyber/.cargo/bin:$PATH"` a suffi. Aucun outil installé ni
  configuration système modifiée pendant M0.1.
- Node.js `24.18.0`, npm `12.2.0`.

Versions résolues dans les lockfiles conservés :

| Composant | Version |
| --- | --- |
| Axum | 0.8.9 |
| Tokio | 1.54.1 |
| Serde | 1.0.229 |
| Reqwest | 0.12.28 |
| Tower (tests) | 0.5.3 |
| Serde JSON (tests) | 1.0.154 |
| React / React DOM | 19.3.1 |
| Vite | 7.3.7 |
| TypeScript | 5.9.3 |
| Tailwind CSS / plugin Vite | 4.3.3 |
| Plugin React Vite | 5.2.0 |

## Modifications M0.1

- `cargo fmt --all` exécuté ; aucune correction de code nécessaire.
- `apps/api/Cargo.lock` généré et conservé, non ignoré par Git. Pas de mise à
  jour des plages de dépendances du manifeste.
- CI : ajout de `--locked` à Clippy, aux tests et au build Rust pour empêcher
  une résolution implicite différente du lockfile.
- README, architecture et roadmap actualisés pour supprimer les mentions de
  blocage Rust et de lockfile absent.
- Ce rapport remplace les résultats initiaux bloqués par les résultats M0.1.

Le socle reste constitué d’un frontend statique et d’un seul crate Rust.
Aucun module métier, fournisseur ou autre composant M1 n’est ajouté.

## Contrôles Rust

Commandes exécutées depuis `apps/api`, avec Rust stable dans le PATH :

| Commande | État | Résultat |
| --- | --- | --- |
| `cargo fmt --all` | PASS | Formatage exécuté. |
| `cargo fmt --all -- --check` | PASS | Aucun écart. |
| `cargo clippy --all-targets -- -D warnings` | PASS | Aucune alerte ni erreur. |
| `cargo test` | PASS | 1 test d’intégration réussi, 0 échec ; aucune autre suite ne contient de test. |
| `cargo build` | PASS | Binaire de développement produit. |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS | Commande CI exécutée avec le lockfile. |
| `cargo test --locked` | PASS | Même test de santé réussi. |
| `cargo build --locked` | PASS | Build avec le lockfile conservé. |

Le premier essai Cargo a échoué sur la résolution DNS de crates.io dans le
sandbox (code 101). Une relance avec accès réseau et au cache Cargo a permis
le téléchargement et la compilation. La suite finale avec `--locked` passe
dans le sandbox une fois le cache rempli. Aucun appel fournisseur vocal.

Le test `health_returns_ok_json` envoie une requête au routeur en mémoire et
vérifie HTTP 200, le type `application/json` et le JSON exact `{"status":"ok"}`.

## API démarrée et contrôles HTTP réels

Le binaire `apps/api/target/debug/labonneia-api` a été lancé sans `API_PORT`
pour vérifier les valeurs par défaut. Un script temporaire hors dépôt a envoyé
une requête HTTP réelle, inspecté l’écoute avec `ss -ltnp`, exercé les erreurs
et arrêté le processus. Ces opérations locales nécessitaient la sortie du sandbox.

| Contrôle | État | Résultat |
| --- | --- | --- |
| `GET http://127.0.0.1:3000/health` | PASS | HTTP 200, `Content-Type: application/json`, corps exact `{"status":"ok"}`. |
| Interface et port par défaut | PASS | Une seule socket d’écoute appartenant au processus : `127.0.0.1:3000` ; aucune écoute sur une interface publique. |
| Second lancement avec port occupé | PASS | Code de sortie 1 et message `Address already in use (os error 98)`. |
| Lancement avec `API_PORT=invalid` | PASS | Code de sortie 1 et message `invalid digit found in string`. |
| Arrêt par SIGINT / Ctrl+C | PASS | Arrêt gracieux, code de sortie 0. |

Le serveur API temporaire a été arrêté après vérification.

## Frontend

Commandes exécutées depuis `apps/web` :

| Commande / contrôle | État | Résultat |
| --- | --- | --- |
| `npm ci --cache /tmp/labonneia-npm-cache --offline` | PASS | Installation reproductible depuis le lockfile et le cache, 84 paquets ajoutés ; audit npm sans vulnérabilité signalée. |
| `npm run typecheck` | PASS | TypeScript sans erreur. |
| `npm run build` | PASS | 29 modules ; HTML 0,51 kB, CSS 8,51 kB, JS 224,72 kB. |
| `npm ls --depth=0` | PASS | Dépendances installées cohérentes. |
| `npm run dev -- --port <port-local-libre>` | PASS | HTML LaBonneIA Studio et module App transformé servis en HTTP 200. |
| `npm run preview -- --port <port-local-libre>` | PASS | HTML LaBonneIA Studio et ressources CSS / JS du build servis en HTTP 200. |

Le port 5173 était occupé par un serveur préexistant : le premier contrôle HTTP
frontend a reçu sa page au lieu de celle de LaBonneIA. Ce résultat a été rejeté,
et les contrôles ont été repris sur des ports libres en conservant `127.0.0.1`.
Le serveur préexistant n’a pas été modifié ni arrêté ; les serveurs LaBonneIA de
vérification ont été arrêtés. Pas de validation visuelle en navigateur.

npm 12 signale toujours le postinstall esbuild bloqué par sa politique locale
`allowScripts`. Aucun assouplissement : installation, build et serveurs passent
avec les binaires installés. L’audit indiqué porte uniquement sur le frontend.

## CI, Git et exclusions

| Contrôle | État | Résultat |
| --- | --- | --- |
| YAML CI et structure | PASS | Parsing et assertions : triggers `push` / `pull_request`, permission `contents: read`, répertoires de travail existants, Node 24, chemins des lockfiles et commandes attendues. |
| Commandes CI | PASS local | fmt, Clippy / tests / build avec `--locked`, installation frontend, typecheck et build exécutés localement. L’installation frontend utilise ici un cache hors ligne. |
| Workflow sur GitHub | NON EXÉCUTÉ | Aucun push ni PR ; aucune exécution distante affirmée. |
| `git diff --check` | PASS | Aucun problème dans le diff suivi. |
| Fichiers nouveaux et suivis | PASS | Contrôle des espaces finaux, sauts de ligne finaux et parsing JSON / TOML, lockfile Rust compris. |
| Exclusions Git | PASS | Secrets `.env`, fichiers audio et sorties de build exclus ; `.env.example`, `Cargo.lock` et `package-lock.json` admissibles. |
| Périmètre M0 | PASS | Aucune fonctionnalité M1, aucun appel ElevenLabs, commit ou push. |

La CI a été contrôlée par parsing et assertions ciblées, sans actionlint ni
exécution du runner GitHub. Ses étapes d’installation sur un runner neuf n’ont
pas été reproduites dans cet environnement WSL.

## Conclusion et suite

M0 est entièrement validé **localement** ; aucun échec fonctionnel ou blocage
local ne reste. Le résultat de la CI sur GitHub demeure à confirmer lors d’un
futur commit/push ou d’une PR explicitement autorisé. Les modifications, dont
les deux lockfiles, restent inspectables sans commit. M0.1 s’arrête ici ; M1
requiert une nouvelle demande.
