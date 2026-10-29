import { useEffect, useRef, useState } from 'react'
import type { FormEvent } from 'react'
import { mockVoices, requestAudio } from './tts'

const initialText = `Bonjour, je suis Sentry, l'assistant IA de Jérémie Nagi.

Grâce au clonage vocal par intelligence artificielle, je peux vous parler avec sa propre voix.

Bienvenue dans son portfolio ! Je vais vous faire découvrir son parcours, ses compétences et ses projets.`
const maxCharacters = 2_000

type AudioResult = { url: string; size: number }
type RangeProps = {
  id: string
  label: string
  value: number
  min: number
  max: number
  onChange: (value: number) => void
  speed?: boolean
}

function RangeControl({ id, label, value, min, max, onChange, speed = false }: RangeProps) {
  const display = (number: number) => speed
    ? `${number.toLocaleString('fr-FR', { minimumFractionDigits: 2 })}×`
    : `${Math.round(number * 100)} %`
  return (
    <div className="range-control">
      <div className="range-label">
        <label htmlFor={id}>{label}</label>
        <output htmlFor={id}>{display(value)}</output>
      </div>
      <input id={id} type="range" min={min} max={max} step="0.01" value={value}
        aria-valuetext={display(value)} onChange={(event) => onChange(Number(event.target.value))} />
      <div className="range-bounds" aria-hidden="true"><span>{display(min)}</span><span>{display(max)}</span></div>
    </div>
  )
}

export default function App() {
  const [text, setText] = useState(initialText)
  const [voiceId, setVoiceId] = useState<string>(mockVoices[0].id)
  const [speed, setSpeed] = useState(1)
  const [stability, setStability] = useState(0.5)
  const [similarity, setSimilarity] = useState(0.75)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [audio, setAudio] = useState<AudioResult | null>(null)
  const activeRequest = useRef<AbortController | null>(null)
  const characterCount = Array.from(text).length
  const textError = !text.trim() ? 'Saisissez un texte pour lancer le test.'
    : characterCount > maxCharacters ? 'Le texte est limité à 2 000 caractères.' : null

  useEffect(() => () => { if (audio) URL.revokeObjectURL(audio.url) }, [audio])
  useEffect(() => () => {
    activeRequest.current?.abort()
    activeRequest.current = null
  }, [])

  function invalidateResult() {
    setAudio(null)
    setError(null)
  }

  async function generate(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (activeRequest.current) return
    invalidateResult()
    if (textError) { setError(textError); return }
    const controller = new AbortController()
    activeRequest.current = controller
    const timeout = window.setTimeout(() => controller.abort(), 15_000)
    setLoading(true)
    try {
      const blob = await requestAudio({ text, voice_id: voiceId, speed, stability, similarity }, controller.signal)
      if (activeRequest.current === controller) {
        setAudio({ url: URL.createObjectURL(blob), size: blob.size })
      }
    } catch (cause) {
      if (activeRequest.current === controller) {
        setError(controller.signal.aborted
          ? 'Le serveur met trop de temps à répondre. Vérifiez l’API puis réessayez.'
          : cause instanceof TypeError
            ? 'Connexion impossible. Vérifiez que le frontend et l’API Rust sont démarrés.'
            : cause instanceof Error ? cause.message : 'Le test a échoué. Réessayez.')
      }
    } finally {
      window.clearTimeout(timeout)
      if (activeRequest.current === controller) {
        activeRequest.current = null
        setLoading(false)
      }
    }
  }

  return (
    <div className="app-shell">
      <header className="site-header">
        <a className="brand" href="#studio" aria-label="LaBonneIA, accueil du Studio">
          <span className="brand-mark" aria-hidden="true"><i /><i /><i /><i /><i /></span>
          LaBonneIA<span className="brand-dot">.</span>
        </a>
        <span className="header-label">STUDIO VOCAL <span>/ M1</span></span>
      </header>

      <main id="studio">
        <div className="intro">
          <div>
            <p className="eyebrow">Votre espace de création</p>
            <h1>Donnez forme à votre voix<span>.</span></h1>
            <p className="intro-description">Écrivez, explorez les réglages et testez le parcours audio.</p>
          </div>
          <span className="simulation-badge"><span aria-hidden="true" /> Mode simulation</span>
        </div>

        <aside className="simulation-note" aria-label="Limites de la simulation">
          <span className="note-icon" aria-hidden="true">i</span>
          <p><strong>Un studio de démonstration, en toute transparence.</strong> Le test produit deux bips fixes,
            sans prononcer votre texte. Les voix sont fictives ; aucun clone vocal ni fournisseur externe n’est utilisé.</p>
        </aside>

        <form onSubmit={generate} className="studio-grid" aria-busy={loading}>
          <section className="panel editor-panel" aria-labelledby="editor-title">
            <div className="panel-heading">
              <div><span className="step-number">01</span><h2 id="editor-title">Votre texte</h2></div>
              <span className="subtle-label">Le point de départ</span>
            </div>
            <label className="sr-only" htmlFor="script">Texte à tester</label>
            <textarea id="script" value={text} disabled={loading}
              aria-describedby="text-hint character-count" aria-invalid={Boolean(textError)}
              spellCheck lang="fr" onChange={(event) => { setText(event.target.value); invalidateResult() }} />
            <div className="editor-footer">
              <span id="text-hint">{textError ?? 'Aucune parole générée en simulation'}</span>
              <span id="character-count" className={characterCount > maxCharacters ? 'count over-limit' : 'count'}>
                {characterCount.toLocaleString('fr-FR')} / 2 000 <span className="sr-only">caractères</span>
              </span>
            </div>
            <div className="generate-area">
              <p>Un signal de test WAV, prêt à écouter.<br /><span>Sans clé API et sans génération vocale réelle.</span></p>
              <button className="primary-button" type="submit" disabled={loading || Boolean(textError)}>
                {loading ? <span className="spinner" aria-hidden="true" /> : <span aria-hidden="true">▷</span>}
                {loading ? 'Test en cours…' : 'Tester la génération'}
              </button>
            </div>
          </section>

          <section className="panel settings-panel" aria-labelledby="settings-title">
            <div className="panel-heading"><div><span className="step-number">02</span><h2 id="settings-title">Voix & réglages</h2></div></div>
            <fieldset disabled={loading}>
              <legend className="sr-only">Paramètres de démonstration</legend>
              <label className="field-label" htmlFor="voice">Voix simulée</label>
              <select id="voice" value={voiceId} onChange={(event) => { setVoiceId(event.target.value); invalidateResult() }}>
                {mockVoices.map((voice) => <option key={voice.id} value={voice.id}>{voice.name} — {voice.detail}</option>)}
              </select>
              <p className="field-help">Deux identités fictives, le même signal de test.</p>
              <div className="settings-divider" />
              <RangeControl id="speed" label="Vitesse" value={speed} min={0.7} max={1.2} speed
                onChange={(value) => { setSpeed(value); invalidateResult() }} />
              <RangeControl id="stability" label="Stabilité" value={stability} min={0} max={1}
                onChange={(value) => { setStability(value); invalidateResult() }} />
              <RangeControl id="similarity" label="Similarité" value={similarity} min={0} max={1}
                onChange={(value) => { setSimilarity(value); invalidateResult() }} />
              <p className="settings-note">Paramètres de démonstration : ils sont envoyés et validés par l’API,
                mais ne modifient ni la tonalité, ni la durée du signal.</p>
            </fieldset>
          </section>

          <section className={`panel result-panel${audio ? ' has-audio' : ''}`} aria-labelledby="result-title">
            <div className="panel-heading">
              <div><span className="step-number">03</span><h2 id="result-title">Votre test audio</h2></div>
              <span className="subtle-label">WAV · mono · 16 kHz</span>
            </div>
            <div role="status" aria-live="polite" className="result-status">
              {loading ? 'Création du signal de test…' : audio ? 'Signal de test prêt. Deux bips, aucune parole.'
                : 'Votre signal apparaîtra ici après le test.'}
            </div>
            {error && <p className="error-message" role="alert">{error}</p>}
            {audio ? (
              <div className="audio-result">
                <audio key={audio.url} controls preload="metadata" src={audio.url} aria-label="Signal de test : deux bips sans parole" />
                <a className="download-button" href={audio.url} download="labonneia-test.wav">↓ Télécharger le WAV</a>
                <p className="audio-meta">1,2 s · {(audio.size / 1024).toLocaleString('fr-FR', { maximumFractionDigits: 1 })} Kio · conservé uniquement en mémoire jusqu’au prochain changement ou à la fermeture de la page.</p>
              </div>
            ) : (
              <div className="empty-wave" aria-hidden="true">{Array.from({ length: 45 }, (_, index) =>
                <span key={index} style={{ height: `${8 + ((index * 17 + 5) % 29)}px` }} />)}
              </div>
            )}
          </section>
        </form>
      </main>
      <footer className="site-footer"><span>LaBonneIA Studio</span><span>M1 · Simulation locale · Aucun audio stocké sur le serveur</span></footer>
    </div>
  )
}
