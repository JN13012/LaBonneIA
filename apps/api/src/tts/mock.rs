//! WAV PCM mono en mémoire : deux bips fixes, aucune parole, aucun accès réseau.
use super::ValidatedRequest;
use std::f64::consts::TAU;

pub(super) fn generate(_request: &ValidatedRequest) -> Vec<u8> {
    const SAMPLE_RATE: u32 = 16_000;
    const SAMPLES: u32 = SAMPLE_RATE * 6 / 5; // 1,2 seconde
    const DATA_SIZE: u32 = SAMPLES * 2;

    let mut wav = Vec::with_capacity(44 + DATA_SIZE as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + DATA_SIZE).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1_u16.to_le_bytes()); // mono
    wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    wav.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes()); // alignement
    wav.extend_from_slice(&16_u16.to_le_bytes()); // bits / échantillon
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&DATA_SIZE.to_le_bytes());

    for index in 0..SAMPLES {
        let time = f64::from(index) / f64::from(SAMPLE_RATE);
        let tone = if (0.1..0.45).contains(&time) {
            Some((660.0, 0.1, 0.45))
        } else if (0.65..1.0).contains(&time) {
            Some((880.0, 0.65, 1.0))
        } else {
            None
        };
        let sample = match tone {
            Some((frequency, start, end)) => {
                // Courtes rampes pour éviter les clics, volume de test modéré.
                let envelope = ((time - start) / 0.01).min((end - time) / 0.01).min(1.0);
                (4_000.0 * envelope * (TAU * frequency * (time - start)).sin()) as i16
            }
            None => 0,
        };
        wav.extend_from_slice(&sample.to_le_bytes());
    }
    wav
}
