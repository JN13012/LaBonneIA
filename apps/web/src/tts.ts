// Contrat M1 : les deux voix sont fictives ; le signal reste identique.
export const mockVoices = [
  { id: 'mock-alba', name: 'Alba', detail: 'Voix fictive · démo A' },
  { id: 'mock-nova', name: 'Nova', detail: 'Voix fictive · démo B' },
] as const

export type GenerateRequest = {
  text: string
  voice_id: string
  speed: number
  stability: number
  similarity: number
}

export async function requestAudio(input: GenerateRequest, signal: AbortSignal): Promise<Blob> {
  const response = await fetch('/api/tts/generate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(input),
    signal,
  })
  if (!response.ok) {
    let message = response.status >= 500
      ? 'L’API est indisponible. Vérifiez que le serveur Rust est démarré sur le port 3000.'
      : `La requête a été refusée (HTTP ${response.status}). Vérifiez les paramètres.`
    try {
      const payload: unknown = await response.json()
      if (typeof payload === 'object' && payload !== null && 'error' in payload) {
        const error = payload.error
        if (typeof error === 'object' && error !== null && 'message' in error && typeof error.message === 'string') {
          message = error.message
        }
      }
    } catch { /* Une erreur de proxy peut ne pas contenir de JSON. */ }
    throw new Error(message)
  }
  if (response.headers.get('content-type')?.split(';')[0].trim() !== 'audio/wav') {
    throw new Error('Le serveur a renvoyé un format inattendu. Un fichier WAV est attendu.')
  }
  const audio = await response.blob()
  const header = new Uint8Array(await audio.slice(0, 12).arrayBuffer())
  const ascii = (bytes: Uint8Array) => String.fromCharCode(...bytes)
  if (audio.size < 44 || ascii(header.slice(0, 4)) !== 'RIFF' || ascii(header.slice(8, 12)) !== 'WAVE') {
    throw new Error('Le fichier audio reçu est invalide. Relancez le test.')
  }
  return audio
}
