# Produit LaBonneIA

## Vision

LaBonneIA vise une plateforme d’assistants vocaux IA personnalisables. Maria
sera l’agent par défaut ; nom, voix, personnalité et capacités pourront être
personnalisés. Le premier produit est un atelier de création audio, sans
système conversationnel : **LaBonneIA Studio**.

Le premier usage réel envisagé reste la présentation du portfolio de Jérémie
par Sentry, avec une voix utilisée sur autorisation de son propriétaire. Le
portfolio est un projet distinct et n’est pas modifié par M1.

## Disponible — M1, exclusivement simulé

- Texte multiligne prérempli avec la présentation Sentry demandée, modifiable,
  avec compteur et limite provisoire de 2 000 caractères Unicode.
- Deux choix de voix fictives : Alba (`mock-alba`) et Nova (`mock-nova`).
- Vitesse de 0,7 à 1,2 ; stabilité et similarité de 0 à 1. Ces paramètres de
  démonstration sont envoyés et validés par le backend.
- Bouton « Tester la génération », chargement, protection contre les doubles
  envois, erreurs lisibles et délai client de 15 secondes.
- Lecture HTML5 volontaire et téléchargement WAV après succès.
- Invalidation du résultat dès qu’une entrée change et libération des URLs
  temporaires au remplacement ou au démontage du composant.
- API locale Rust : santé et génération Mock, sans secret ni accès fournisseur.

**Le Mock ne prononce pas le texte.** Il renvoie toujours le même signal de
1,2 seconde : deux bips à 660 puis 880 Hz séparés par du silence. Ni le texte,
ni le choix de voix, ni les réglages ne modifient sa tonalité ou sa durée.
Le texte initial mentionne le clonage pour le futur usage Sentry ; il ne
constitue pas une capacité disponible dans cette démonstration.

Le serveur ne stocke ni texte ni audio et ne journalise pas le contenu soumis.
Le navigateur garde le résultat en mémoire jusqu’à son invalidation ou la
fermeture de la page. Un téléchargement demandé par l’utilisateur produit un
fichier local qui reste sous son contrôle. Aucun audio privé ne doit entrer
dans Git. Les résultats exécutés sont dans [VERIFICATION.md](VERIFICATION.md).

## V1 — Suite future

M2 devra intégrer un fournisseur TTS réel après vérification de son contrat et
autorisation. Le fournisseur pressenti est ElevenLabs. Il faudra traiter les
clés exclusivement côté backend, les délais, les quotas, les erreurs, le format
audio et les limites propres au fournisseur, avec des tests HTTP simulés.
Aucun appel payant n’est autorisé implicitement par M1.

Une voix personnalisée exigera le consentement de son propriétaire, un accès
limité et des règles de collecte, de conservation et de suppression. Aucun
clone réel n’est actuellement disponible. Le téléchargement MP3 et la parole
réelle restent futurs ; le seul export M1 est le WAV de test.

## V2 et V3 — Futur

V2 : conversation avec STT, LLM, TTS, interruptions et mémoire de session.
V3 : téléphonie, agendas, réservations, intégrations et gestion de plusieurs
entreprises, avec authentification et isolation des données adaptées.

Ni ces capacités, ni une base de données, ni une gestion de comptes ne font
partie de M1. Aucun passage automatique à M2.
