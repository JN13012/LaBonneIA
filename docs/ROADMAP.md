# Feuille de route

Les jalons sont séquentiels. M0 seul est demandé et implémenté ici. Les suivants
sont des propositions à confirmer ; aucun passage automatique à M1.

| Jalon | Objectif | Livrable | Critères de validation |
| --- | --- | --- | --- |
| M0 — Fondation | Poser Rust + React | Page Studio, API `/health`, documentation, CI | Installation web, TypeScript et build réussis ; fmt, Clippy, tests et build Rust réussis ; `/health` HTTP 200 et JSON valide ; écoute locale ; diff propre. Validation locale réussie ; voir VERIFICATION.md pour les résultats et la limite de CI distante. |
| M1 — Contrat de synthèse | Définir la première tranche V1 sans coût externe | Formulaire texte/voix/réglages, validation backend et contrat fournisseur avec doubles de test | Entrées invalides rejetées ; erreurs et états UI testés hors ligne ; aucun appel payant ; limites et contrat officiel documentés. |
| M2 — Première synthèse | Relier un fournisseur TTS après autorisation explicite | Adaptateur ElevenLabs et restitution MP3 | Tests HTTP simulés ; clé côté backend ; délais, quotas et erreurs traités ; essai réel uniquement autorisé et coût borné ; aucun audio privé dans Git. |
| M3 — Voix personnalisée | Permettre un clone autorisé pour le premier usage Sentry | Sélection d’une voix autorisée, réglages et écoute | Consentement et accès établis ; réglages validés ; conservation et suppression définies ; sources privées exclues ; essai autorisé de présentation du portfolio. |
| M4 — Studio V1 validé | Stabiliser le parcours de génération et d’export | Studio utilisable de bout en bout et documentation | Génération, écoute et téléchargement validés ; erreurs, accessibilité et données privées vérifiées ; tests et CI verts ; limites V1 documentées. |

V2 (conversation, STT/LLM, interruptions, mémoire) et V3 (agents métier,
téléphonie, agendas, réservations, multi-entreprises) feront l’objet de leurs
propres jalons. Elles ne sont pas comprises dans M0–M4.
