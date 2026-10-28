# Produit LaBonneIA

## Vision

LaBonneIA sera une plateforme SaaS permettant aux entreprises de créer des
assistants vocaux IA personnalisables. Maria sera l’agent par défaut. Les clients
pourront ensuite personnaliser leur nom, voix, personnalité et capacités.

Le premier usage visé est une présentation audio du portfolio de Jérémie,
prononcée par Sentry avec un clone autorisé de sa voix. Le portfolio reste un
projet distinct : M0 ne le modifie pas.

## Disponible dans le code M0

- Une page d’accueil statique LaBonneIA Studio en React.
- Une API Rust avec `GET /health` retournant `{"status":"ok"}`.
- Une configuration de développement et une CI de validation.

Il n’y a actuellement ni synthèse, ni clone vocal, ni export audio, ni dialogue,
ni compte utilisateur, ni stockage. L’état des vérifications locales est consigné
séparément dans [VERIFICATION.md](VERIFICATION.md).

## V1 — LaBonneIA Studio (futur)

Un atelier de génération audio, sans système conversationnel :

- Saisir un texte et choisir une voix disponible chez le fournisseur retenu.
- Utiliser un clone de voix uniquement avec autorisation explicite de son
  propriétaire et selon les conditions du fournisseur.
- Ajuster les réglages pris en charge par le fournisseur, avec bornes et
  validation côté serveur. Ne pas inventer de capacités non disponibles.
- Lancer une synthèse via le backend, afficher les états en cours, réussis ou en
  erreur, puis permettre l’écoute et le téléchargement du résultat MP3.
- Donner des erreurs compréhensibles pour clé absente, texte invalide, délai
  dépassé, quota ou refus fournisseur. Ne pas masquer un échec par un faux succès.
- Garder secrets, sources vocales et fichiers privés hors du dépôt et du frontend.

Le fournisseur pressenti est ElevenLabs ; son contrat et ses coûts seront
vérifiés au jalon d’intégration. Tout appel payant nécessite une autorisation.
Les limites de texte, les réglages exacts et le cycle de conservation des données
seront arrêtés avant cette intégration. V1 ne promet pas de dialogue temps réel,
de téléphonie, de mémoire conversationnelle ni de gestion multi-entreprises.

## V2 — Agents conversationnels (futur)

Ajouter STT, LLM, TTS, dialogue temps réel, interruptions et mémoire de session.
Les contraintes de latence et les règles de conservation seront traitées à ce
stade. La personnalisation de Maria deviendra une capacité d’agent, au-delà du
choix de voix du Studio.

## V3 — Agents métier (futur)

Ajouter téléphonie, agendas, réservations, intégrations et gestion de plusieurs
entreprises. Authentification, isolation des données, autorisations et stockage
nécessiteront alors une conception propre à ces usages.
