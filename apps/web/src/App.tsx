export default function App() {
  return (
    <div className="min-h-screen bg-stone-50 text-stone-900">
      <header className="mx-auto flex max-w-5xl items-center justify-between gap-4 px-6 py-7">
        <a className="font-semibold tracking-tight focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-emerald-700" href="#accueil">
          LaBonneIA<span className="text-emerald-700">.</span>
        </a>
        <span className="rounded-full border border-stone-200 px-3 py-1 text-xs text-stone-600">M0 · Fondation</span>
      </header>
      <main id="accueil" className="mx-auto max-w-5xl px-6 py-16 sm:py-24">
        <p className="mb-5 text-sm font-medium uppercase tracking-widest text-emerald-700">LaBonneIA Studio</p>
        <h1 className="max-w-3xl text-5xl font-semibold leading-tight tracking-tight sm:text-7xl">Une voix à votre image.</h1>
        <p className="mt-7 max-w-xl text-lg leading-relaxed text-stone-600">
          Le futur atelier pour créer vos contenus vocaux et donner une voix à vos assistants IA.
        </p>
        <section aria-labelledby="status-title" className="mt-14 max-w-2xl rounded-2xl border border-stone-200 bg-white p-7 sm:p-9">
          <p className="mb-3 text-xs font-semibold uppercase tracking-widest text-emerald-700">Le projet commence ici</p>
          <h2 id="status-title" className="text-xl font-semibold">Les fondations sont en place.</h2>
          <p className="mt-3 leading-relaxed text-stone-600">
            Cette première version pose le socle technique du Studio. La synthèse vocale,
            le clonage de voix et l’export audio arriveront lors des prochains jalons.
          </p>
          <div className="mt-7 border-t border-stone-100 pt-6">
            <h3 className="font-medium">À venir : Maria</h3>
            <p className="mt-2 text-sm leading-relaxed text-stone-600">
              Votre agent par défaut, puis des assistants dont vous pourrez personnaliser
              le nom, la voix, la personnalité et les capacités.
            </p>
          </div>
        </section>
      </main>
      <footer className="mx-auto max-w-5xl px-6 pb-8 text-xs text-stone-500">LaBonneIA · Studio en développement</footer>
    </div>
  )
}
