# Vérification de clôture M1 — 29 octobre 2026

## Résultat local avant publication

**PASS : le Studio Mock est validé de bout en bout.** Les contrôles Rust et
React, les endpoints réels, le WAV et le parcours navigateur ont été rejoués
pour cette clôture. Aucune nouvelle anomalie fonctionnelle trouvée.

Le texte n’est pas prononcé : toutes les entrées valides produisent deux bips
fixes. Aucun appel ElevenLabs, clone, clé réelle, stockage serveur ou travail
M2. La publication est explicitement autorisée avec le message
`feat: implement mock voice studio`, suivie du contrôle des jobs distants.

Ce rapport consigne les résultats acquis avant le commit de publication.
Le SHA et les résultats GitHub Actions constatés après le push sont fournis
dans le compte rendu de clôture. Les exécutions restent consultables dans
[GitHub Actions — CI](https://github.com/JN13012/LaBonneIA/actions/workflows/ci.yml).

## Inspection et changements

- Branche `main`, remote `git@github.com:JN13012/LaBonneIA.git`.
- Départ de clôture : `82da166643462858c41c96b4a98603a6efbef3c4`.
  Ce commit frontend et son prédécesseur backend `86480b3` sont conservés,
  sans réécriture de l’historique.
- `git fetch origin main` puis `git rev-list --left-right --count
  HEAD...origin/main` : `0 0` avant publication, sans divergence.
- Modifications de clôture : commentaires de `.env.example`, README,
  architecture, produit, roadmap et ce rapport ; formatage de `routes.rs` ;
  ajout du test d’intégration `apps/api/tests/tts.rs` encore non suivi au départ.
- Aucun refactoring supplémentaire ou fonctionnalité hors M1. Un seul crate.
- `Cargo.lock`, `package-lock.json`, manifestes de dépendances et workflow CI
  inchangés pendant cette clôture. Aucun nouveau paquet ni outil système.

## Versions

Rust `1.99.1`, Cargo `1.99.1`, Node.js `24.18.0`, npm `12.2.0`.
Chromium existant `156.0.8078.4`, utilisé en headless via CDP.
Dépendances conservées : Axum 0.8.9, React / React DOM 19.3.1,
TypeScript 5.9.3, Vite 7.3.7 et Tailwind 4.3.3.

Le shell de validation utilise temporairement
`export PATH="/home/cyber/.cargo/bin:$PATH"`, sans modification système.

## Rust — contrôles rejoués

Depuis `apps/api` :

| Commande | Résultat |
| --- | --- |
| `cargo fmt --all` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS — aucun avertissement |
| `cargo test --locked` | PASS — 11 tests d’intégration, zéro échec |
| `cargo build --locked` | PASS |

Un test de santé et dix tests TTS couvrent : réponse WAV / en-têtes / tailles /
échantillons, déterminisme et invariance aux paramètres, texte vide et blanc
Unicode, longueur 2 000 / 2 001 caractères, espaces compris, voix inconnue,
paramètres hors limites et bornes inclusives, JSON incorrect, champs absents
ou inconnus, types et Content-Type incompatibles, requête trop volumineuse.
Pas d’autres tests unitaires ou doctests dans les suites.

Le chemin Mock est synchrone et pur : aucune utilisation de Reqwest, de fichier
ou de client fournisseur dans la génération. Le seul réseau de l’API est son
serveur HTTP local.

## React — contrôles rejoués

Depuis `apps/web` :

| Commande | Résultat |
| --- | --- |
| `npm ci --offline --cache /tmp/labonneia-npm-cache` | PASS — 84 paquets, audit npm sans vulnérabilité signalée |
| `npm run typecheck` | PASS |
| `npm run build` | PASS — 30 modules |

Build : HTML 0,51 kB, CSS 11,85 kB, JavaScript 231,54 kB.
npm signale le postinstall esbuild bloqué par la politique locale allowScripts.
Aucune politique modifiée ; installation, build et serveur Vite fonctionnent.

Pas d’infrastructure UI ajoutée au projet. Le navigateur déjà installé est
piloté par un script Node / CDP temporaire hors dépôt.

## Endpoints, WAV et navigateur réels

Commande : `node /tmp/labonneia-m1-e2e.cjs`.
Le port 3000 étant déjà occupé, le premier lancement a arrêté la vérification
avec `EADDRINUSE`, sans démarrer ou arrêter de serveur utilisateur.
Le script temporaire a été adapté pour utiliser `API_PORT` et une cible de
proxy Vite sur des ports libres. Le proxy est ajusté uniquement dans la
configuration de test en mémoire, sans changement des valeurs du dépôt.

Dernière exécution complète, code de sortie 0 : API
`http://127.0.0.1:43347`, frontend `http://127.0.0.1:37981`.
Les serveurs de test ont été arrêtés ; les serveurs préexistants sont conservés.
Le dépôt garde les valeurs par défaut API 3000 / Vite 5173 sur `127.0.0.1`.

| Contrôle | Résultat |
| --- | --- |
| `GET /health` | PASS — HTTP 200, `{"status":"ok"}` |
| `POST /api/tts/generate` via le proxy Vite | PASS — HTTP 200, `audio/wav`, `no-store`, 38 444 octets |
| Entrées invalides par HTTP réel | PASS — erreurs explicites 400 ; corps supérieur à 16 Kio : 413 |
| JSON mal formé | PASS — HTTP 400 et erreur JSON |
| Texte initial et contrôles UI | PASS — Sentry, compteur, deux voix fictives ; vide et 2 001 emoji refusés, 2 000 acceptés |
| Paramètres envoyés | PASS — texte, Nova et vitesse 1,1 présents dans la requête capturée |
| Chargement | PASS — indicateur visible, doubles envois bloqués, champs désactivés |
| Lecture HTML5 | PASS — durée 1,2 s, pas d’autoplay, démarrage et progression du temps puis pause |
| Décodage AudioContext | PASS — mono, 16 kHz, 19 200 échantillons |
| Téléchargement navigateur | PASS — fichier reçu, octets identiques à la réponse HTTP de référence |
| Cycle de vie des Object URLs | PASS — trois créées, trois révoquées ; invalidation, remplacement et démontage testés |
| Erreurs contrôlées | PASS — JSON 400, WAV corrompu, MIME inattendu : erreur lisible, aucun faux lecteur, reprise possible |
| Délai de 15 secondes | PASS — annulation réelle, message utilisateur, nouvel essai réussi |
| API réellement arrêtée | PASS — erreur de proxy compréhensible, ancien résultat supprimé |
| Console et réseau navigateur | PASS — aucune exception JavaScript ou requête HTTP externe |
| Bureau et mobile | PASS — captures 1 440, 390 et 320 px ; pas de débordement horizontal, captures bureau et mobile inspectées |
| Accessibilité ciblée | PASS — contrôles nommés et préférence de mouvement réduit |

Analyse indépendante du fichier téléchargé avec Python `wave`, puis corrélation
des échantillons : PCM signé 16 bits, mono, 16 000 Hz, 1,2 s, silences et bips
660 / 880 Hz confirmés. Aucun texte ou paramètre ne modifie ce signal.

Preuves temporaires : `/tmp/labonneia-m1-verification/results.json`, captures
PNG, journaux et WAV téléchargé dans ce même répertoire hors Git.
La validation headless couvre la lecture technique ; ce passage n’affirme pas
une écoute humaine ou un audit complet d’accessibilité / de compatibilité.
L’utilisateur confirme également génération, lecture et téléchargement dans
son navigateur.

## Git, secrets et CI

| Contrôle avant publication | Résultat |
| --- | --- |
| Périmètre suivi et non suivi | PASS — 27 fichiers admissibles inspectés |
| Secrets | PASS ciblé — aucun motif de clé connu ou clé privée ; template fournisseur vide ; aucun .env réel admissible |
| Fichiers privés / artefacts | PASS — aucun audio, binaire ou lien symbolique dans le périmètre admissible |
| Exclusions | PASS — target, node_modules et dist ignorés ; preuves et WAV uniquement sous /tmp |
| Lockfiles | PASS — présents, conservés, inchangés |
| CI statique | PASS — YAML, push / pull_request, contents:read, répertoires et commandes, Node 24 et chemins des lockfiles |
| Commandes CI locales | PASS — formatage, Clippy, tests et build Rust, installation / typecheck / build React |
| `git diff --check` | PASS — fichiers non suivis aussi contrôlés séparément |

La CI distante est vérifiée après publication sur le SHA poussé : les deux
jobs attendus sont **Rust** et **React**. Aucune réussite distante n’est
anticipée dans ce rapport prépublication. En cas d’échec, seules les anomalies
nécessaires à la clôture M1 seront corrigées et les contrôles concernés rejoués.
Le contrôle de secrets repose sur les noms, le contenu et des motifs connus ;
il ne constitue pas une garantie universelle de détection.

## Limites et arrêt du jalon

Le Mock ne prononce aucun texte. Voix et réglages sont démonstratifs, le signal
est toujours identique. Aucun fournisseur, clé réelle, clone, LLM, STT,
téléphonie, réservation, base de données ou stockage audio serveur.
Le téléchargement demandé crée un fichier sous le contrôle de l’utilisateur.
Le build statique nécessite un routage API adapté avant tout futur déploiement.

M1 s’arrête à cette clôture et sa publication autorisée. M2 n’est pas commencé.
Les commandes de démarrage et le contrat complet restent dans README.md et
ARCHITECTURE.md. Aucune intégration fournisseur ni essai payant automatique.
