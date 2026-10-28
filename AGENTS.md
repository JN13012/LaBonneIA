# Règles de travail — LaBonneIA

- Lire le README et les documents concernés avant toute modification.
- Modifier uniquement le périmètre demandé. Conserver le travail existant et inspecter
  l’état Git avant d’éditer. Ne pas créer de commit ni pousser sans autorisation.
- Séparer les capacités réellement disponibles des fonctionnalités futures.
- Garder un seul crate Rust et une architecture simple. Créer un module seulement
  lorsqu’il contient une responsabilité réelle ; éviter les abstractions prématurées.
- Valider systématiquement après modification : dans `apps/api`, exécuter
  `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test`, `cargo build`. Dans `apps/web`, exécuter `npm ci`,
  `npm run typecheck`, `npm run build`. Vérifier `GET /health` si l’API est concernée,
  puis `git diff --check`. Expliquer précisément tout contrôle bloqué ou non exécuté.
- Ne jamais versionner de secrets, fichiers `.env`, sources de clones vocaux,
  enregistrements privés ou exports audio. Vérifier aussi les fichiers non suivis.
- Garder les clés fournisseurs exclusivement côté backend ; aucune clé dans une
  variable `VITE_*`, le navigateur, un log ou une réponse HTTP.
- Aucun appel payant sans autorisation explicite. Ne pas appeler ElevenLabs en M0.
- Toute future utilisation d’une voix doit être autorisée par son propriétaire ;
  limiter sa collecte, son accès, sa conservation et prévoir sa suppression.
- Écouter sur la boucle locale. Ne pas exposer les serveurs publiquement sans demande.
- Ne pas installer ni modifier les outils système automatiquement. Si un outil
  manque, expliquer son installation et signaler les validations bloquées.
- Fournir un rapport de vérification : changements, versions, commandes, résultats
  PASS / FAIL / BLOQUÉ, limites et prochaine étape. Ne jamais annoncer comme réussi
  un contrôle non exécuté. S’arrêter au jalon demandé.
