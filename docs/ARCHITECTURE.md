# Architecture

## Socle actuel M0

Deux applications indépendantes dans un même dépôt :

- `apps/web` : React, TypeScript, Vite et Tailwind CSS via son plugin Vite.
  `src/App.tsx` contient une page statique, sans appel réseau ni action TTS.
  Vite écoute sur `127.0.0.1:5173` et la prévisualisation sur `127.0.0.1:4173`.
- `apps/api` : un seul crate Rust, utilisant Axum pour HTTP, Tokio pour le
  runtime asynchrone et Serde pour JSON. Reqwest est déclaré pour les futurs
  appels HTTP, avec TLS Rustls et sans requête exécutée dans M0.

L’API écoute exclusivement sur `127.0.0.1`, port 3000 par défaut. `API_PORT`
permet de changer le port via l’environnement du processus. Le programme ne
charge pas automatiquement `.env`. Une erreur de configuration, de bind ou de
serveur est écrite sur stderr et produit un code de sortie non nul. Ctrl+C
permet un arrêt gracieux. `/health` est une sonde de vie, pas une vérification
de disponibilité d’un fournisseur externe.

## Responsabilités des fichiers Rust

- `src/main.rs` : lecture du port, démarrage, arrêt et erreurs du processus.
- `src/lib.rs` : construction de l’application réutilisable en test.
- `src/routes.rs` : route et handler `/health`, réponse typée.
- `tests/health.rs` : requête au routeur en mémoire, assertions HTTP, type de
  contenu et contenu JSON exact, sans ouvrir de port ni appeler de fournisseur.

La configuration est assez petite pour rester dans `main.rs`. Aucun dossier
métier ou fournisseur vide n’est créé. Lorsqu’une synthèse sera implémentée,
extraire la configuration si nécessaire et ajouter un service métier ainsi
qu’un adaptateur fournisseur dans ce même crate. Les handlers valideront les
entrées et délégueront ; le service portera les règles indépendantes du
fournisseur ; l’adaptateur isolera son contrat HTTP et ses erreurs.

## Flux futur, non implémenté

```text
Frontend (texte, voix, réglages)
  → Backend (validation, règles métier)
  → Adaptateur fournisseur TTS (clé privée côté serveur)
  → Fournisseur TTS
  → Backend (résultat audio ou erreur)
  → Frontend (écoute et téléchargement MP3)
```

La clé `ELEVENLABS_API_KEY` restera côté backend, jamais dans `VITE_*`.
Le mode de transfert audio, les limites et la conservation seront définis lors
de l’intégration. Aucun stockage, proxy TTS, CORS ou authentification n’est
nécessaire à la page statique M0. Le mode de connexion web/API sera choisi au
premier besoin réel, avec préférence pour une origine commune ou un proxy de
développement local.

## Choix techniques et limites

React et TypeScript structurent l’interface avec des types vérifiés. Vite fournit
le développement et le build ; Tailwind évite une couche de composants dédiée
pour cette page minimale. Axum s’appuie sur Tokio et permet de tester le routeur
sans serveur. Reqwest préparera un accès HTTP sans exposer les secrets au client.
Deux applications et un seul crate suffisent : pas de microservices, Docker,
base de données ni moteur LLM en M0.

`apps/web/package-lock.json` fixe les dépendances frontend ; `npm ci` les
reproduit. `apps/api/Cargo.lock` est généré, conservé et validé avec Rust stable
1.99.1. La CI utilise `--locked` pour Clippy, les tests et le build afin de
respecter cette résolution. Le lockfile reste admissible dans Git et sera
versionné lors d’un commit explicitement autorisé.
La CI utilise Rust stable et Node.js 24. Elle est préparée mais son résultat
GitHub ne peut pas être affirmé avant exécution sur un push ou une PR autorisé.

Références : [Axum](https://docs.rs/axum/0.8/axum/),
[Vite](https://vite.dev/guide/),
[Tailwind pour Vite](https://tailwindcss.com/docs/installation/using-vite).
