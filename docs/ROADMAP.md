# Feuille de route

Les jalons sont séquentiels. M0 est publié ; M1 apporte le parcours simulé.
Aucun passage automatique à M2. Les résultats réellement exécutés figurent
dans [VERIFICATION.md](VERIFICATION.md).

| Jalon | État et objectif | Livrable / validation |
| --- | --- | --- |
| M0 — Fondation | Publié à `3366112699970637bc92dbf151439655ddd2bc0d` | React / TypeScript / Tailwind, API Rust `/health`, lockfiles, documentation et CI Rust / React. |
| M1 — Studio simulé | Implémenté et validé localement ; clôture et publication autorisées, contrôles dans VERIFICATION.md | Texte, compteur, voix fictives, réglages démonstratifs, validation HTTP typée, WAV fixe, écoute et téléchargement, erreurs, proxy local. Tests hors fournisseur et parcours HTTP réel ; aucune parole ni clé API. |
| M2 — Première synthèse réelle | Futur, sur demande | Vérifier le contrat officiel ElevenLabs, les limites et coûts ; adaptateur backend avec clé privée, délais, quotas et erreurs. Tests HTTP simulés ; essai réel uniquement après autorisation explicite et coût borné. Réévaluer format audio et limites provisoires M1. |
| M3 — Voix personnalisée | Futur, sur demande | Voix utilisée avec autorisation de son propriétaire, accès limité, conservation et suppression définies ; aucun fichier privé dans Git. Valider l’usage Sentry sans modifier le portfolio implicitement. |
| M4 — Studio V1 validé | Futur | Stabiliser génération réelle, écoute, export, erreurs, accessibilité et protection des données ; tests et CI validés. |

M1 ne prévoit ni fournisseur réel, clone, LLM, STT, téléphonie, réservation,
base de données, stockage audio permanent ou déploiement public.

V2 : agents conversationnels (STT / LLM / TTS, interruptions, mémoire).
V3 : agents métier (téléphonie, agendas, réservations, intégrations et gestion
multi-entreprises). Ces capacités feront l’objet de périmètres séparés.
