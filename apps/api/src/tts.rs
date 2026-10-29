//! Contrat métier M1 : validation puis génération exclusivement simulée.
use serde::Deserialize;

mod mock;

const MAX_TEXT_CHARS: usize = 2_000;
const MOCK_VOICES: [&str; 2] = ["mock-alba", "mock-nova"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GenerateRequest {
    text: String,
    voice_id: String,
    speed: f64,
    stability: f64,
    similarity: f64,
}

// Seule la validation peut construire cette entrée. Le Mock ignore volontairement
// son contenu : ni le texte ni les réglages ne changent le signal de test.
pub(crate) struct ValidatedRequest {
    _input: GenerateRequest,
}

pub(crate) struct ValidationError {
    pub code: &'static str,
    pub message: &'static str,
}

impl GenerateRequest {
    pub fn validate(self) -> Result<ValidatedRequest, ValidationError> {
        if self.text.trim().is_empty() {
            return Err(ValidationError {
                code: "empty_text",
                message: "Saisissez un texte non vide.",
            });
        }
        if self.text.chars().count() > MAX_TEXT_CHARS {
            return Err(ValidationError {
                code: "text_too_long",
                message: "Le texte est limité à 2 000 caractères Unicode.",
            });
        }
        if !MOCK_VOICES.contains(&self.voice_id.as_str()) {
            return Err(ValidationError {
                code: "unknown_voice",
                message: "Choisissez une voix de démonstration autorisée : mock-alba ou mock-nova.",
            });
        }
        for (value, min, max, code, message) in [
            (
                self.speed,
                0.7,
                1.2,
                "invalid_speed",
                "La vitesse doit être comprise entre 0,7 et 1,2.",
            ),
            (
                self.stability,
                0.0,
                1.0,
                "invalid_stability",
                "La stabilité doit être comprise entre 0 et 1.",
            ),
            (
                self.similarity,
                0.0,
                1.0,
                "invalid_similarity",
                "La similarité doit être comprise entre 0 et 1.",
            ),
        ] {
            if !value.is_finite() || !(min..=max).contains(&value) {
                return Err(ValidationError { code, message });
            }
        }
        Ok(ValidatedRequest { _input: self })
    }
}

pub(crate) fn generate(request: ValidatedRequest) -> Vec<u8> {
    mock::generate(&request)
}
