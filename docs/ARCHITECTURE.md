# Architecture — M1

## Applications et flux

Un frontend React / TypeScript / Tailwind et un seul crate Rust / Axum :

```text
Formulaire React
  → POST /api/tts/generate (JSON, même origine navigateur)
  → Proxy Vite de développement → 127.0.0.1:3000
  → HTTP : extraction typée et limite de corps
  → Validation métier → entrée validée → service TTS → Mock pur
  → Réponse binaire WAV en mémoire
  → Blob / Object URL → lecteur HTML5 et téléchargement
```

Le Mock ne contacte aucun fournisseur et ne nécessite ni clé ni Internet une
fois les dépendances installées. Reqwest reste déclaré pour un futur adaptateur,
mais n’est pas utilisé. Aucun trait fournisseur générique, module vide,
microservice, stockage, base de données, LLM ou STT n’est ajouté.

## Responsabilités

- `apps/api/src/main.rs` : configuration `API_PORT` (3000 par défaut), bind
  IPv4 `127.0.0.1`, erreurs de démarrage et arrêt gracieux Ctrl+C.
- `src/lib.rs` : application réutilisée par les tests.
- `src/routes.rs` : endpoints, extraction JSON, limite de 16 Kio, conversion
  des erreurs en JSON et réponse WAV. Aucune logique de signal dans HTTP.
- `src/tts.rs` : DTO Serde, validation et type `ValidatedRequest` constructible
  uniquement après validation ; le service délègue au Mock.
- `src/tts/mock.rs` : WAV déterministe produit en mémoire depuis une requête
  validée. Le contenu de cette requête est volontairement ignoré par le Mock.
- `tests/health.rs`, `tests/tts.rs` : contrats HTTP au routeur, sans socket,
  fichier ni requête fournisseur.
- `apps/web/src/tts.ts` : contrat client, requête, erreurs serveur et contrôle
  du type MIME et des signatures RIFF/WAVE avant création d’une URL.
- `src/App.tsx` : formulaire, validation de confort, état de requête, lecteur,
  téléchargement et gestion des URLs temporaires.
- `src/index.css` : interface responsive, focus visibles et mouvement réduit.
- `vite.config.ts` : ports stricts sur loopback ; proxy `/api` vers
  `http://127.0.0.1:3000`. Aucune configuration CORS permissive.

## Contrat HTTP

### Santé

`GET /health` → HTTP 200, `Content-Type: application/json`, `{"status":"ok"}`.
Sonde de vie uniquement, sans contrôle d’un fournisseur externe.

### Génération simulée

`POST /api/tts/generate`, `Content-Type: application/json` :

```json
{
  "text": "Bonjour, ceci est un test.",
  "voice_id": "mock-alba",
  "speed": 1.0,
  "stability": 0.5,
  "similarity": 0.75
}
```

Tous les champs sont obligatoires ; les champs supplémentaires sont rejetés.

| Champ | Type | Validation M1 |
| --- | --- | --- |
| `text` | string | Non vide après trim ; longueur du texte brut ≤ 2 000 valeurs scalaires Unicode, espaces compris. Ni octets ni unités UTF-16 ni graphèmes. |
| `voice_id` | string | Exactement `mock-alba` ou `mock-nova`. Identités fictives. |
| `speed` | number fini | 0,7 ≤ valeur ≤ 1,2. |
| `stability` | number fini | 0 ≤ valeur ≤ 1. |
| `similarity` | number fini | 0 ≤ valeur ≤ 1. |

La taille totale du corps encodé est limitée à **16 384 octets** avant
production audio, indépendamment du nombre de caractères. Un JSON très
échappé peut atteindre cette limite avec un texte plus court. Les bornes sont
inclusives et provisoires, à revoir lors de l’intégration du vrai fournisseur.

Succès : HTTP 200, corps binaire WAV et en-têtes :

```text
Content-Type: audio/wav
Content-Disposition: attachment; filename="labonneia-test.wav"
Cache-Control: no-store
```

WAV RIFF / PCM signé 16 bits little-endian, mono, 16 000 Hz, 1,2 seconde,
38 444 octets. Bips 660 Hz puis 880 Hz, rampes de 10 ms et silences. Le même
signal est renvoyé pour toutes les requêtes valides : aucune parole, aucune
simulation d’un clone et aucune modification selon les réglages.

Erreur : `Content-Type: application/json`, structure commune :

```json
{
  "error": {
    "code": "empty_text",
    "message": "Saisissez un texte non vide."
  }
}
```

| HTTP | Code | Cause |
| --- | --- | --- |
| 400 | `empty_text` | Texte vide après trim. |
| 400 | `text_too_long` | Plus de 2 000 caractères. |
| 400 | `unknown_voice` | Identifiant de voix non autorisé. |
| 400 | `invalid_speed` | Vitesse hors limites. |
| 400 | `invalid_stability` | Stabilité hors limites. |
| 400 | `invalid_similarity` | Similarité hors limites. |
| 400 | `invalid_json` | JSON mal formé, champ absent / inconnu, mauvais type ou Content-Type incompatible. |
| 413 | `request_too_large` | Corps supérieur à 16 Kio. |

Les messages ne reproduisent ni le texte soumis ni une valeur secrète.
Les routes inexistantes et les méthodes non prises en charge utilisent les
réponses Axum 404 / 405 habituelles, hors de ce contrat d’erreur de génération.

## Cycle de vie audio et requêtes

L’interface empêche les doubles envois et désactive les paramètres pendant la
requête. Un `AbortController` limite l’attente client à 15 secondes ; le
démontage annule aussi la requête. L’audio n’est créé qu’après succès HTTP et
contrôle du fichier. Une modification d’entrée efface le résultat afin de ne
pas associer un ancien fichier aux nouveaux paramètres. Les URLs sont
révoquées à l’invalidation, au remplacement et au démontage ; aucun autoplay.

Les octets WAV sont assemblés dans un `Vec<u8>` puis envoyés, sans écriture sur
disque ni cache serveur. Le frontend conserve un Blob temporaire. Le bouton
de téléchargement enregistre le fichier uniquement sur demande utilisateur.
Pas de persistance des textes ni de logs de leur contenu.

## Développement, CI et suite

Node 24 et Rust stable ; `package-lock.json` et `Cargo.lock` conservés. La CI
existante contrôle fmt, Clippy, tests et build Rust avec `--locked`, puis
`npm ci`, TypeScript et build frontend. Pas de dépendance ni stack de tests UI
supplémentaire pour M1. Voir [VERIFICATION.md](VERIFICATION.md) pour les
contrôles réels et les limites de validation.

Le build statique ne suffit pas à déployer l’API. Le routage d’une origine
commune et l’hébergement restent à définir hors M1. Le port backend peut être
changé avec `API_PORT`, sans chargement automatique de `.env` ; adapter alors
le proxy Vite. Tous les serveurs restent sur la boucle locale.

M2 pourra remplacer la délégation au Mock par un adaptateur fournisseur dans
ce crate, après vérification du contrat, des coûts et des autorisations. Le
format audio et les limites seront alors réévalués. Toute clé fournisseur
restera côté backend, jamais dans `VITE_*`, les logs ou les réponses HTTP.
