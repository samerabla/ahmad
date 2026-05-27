use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="ueber-uns" class="section-padding bg-white">
            <div class="max-w-7xl mx-auto px-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">

                    // Left: Image
                    <div class="animate-on-scroll relative">
                        <div class="relative rounded-2xl overflow-hidden"
                             style="aspect-ratio: 4/5; max-height: 600px;">
                            <img
                                src="https://images.unsplash.com/photo-1593079831268-3381b0db4a77?w=800&q=80"
                                alt="Zertifizierter Baumpfleger beim Klettern"
                                class="w-full h-full object-cover"
                                loading="lazy"
                            />
                            <div class="absolute inset-0"
                                 style="background: linear-gradient(to top, rgba(26,61,43,0.4) 0%, transparent 60%);">
                            </div>
                        </div>
                        // Floating Badge
                        <div class="absolute -bottom-6 -right-6 bg-white rounded-2xl shadow-xl p-5 border border-gray-100">
                            <div class="flex items-center gap-3">
                                <div class="w-12 h-12 rounded-xl flex items-center justify-center"
                                     style="background: #1a3d2b;">
                                    <svg width="24" height="24" fill="none" stroke="white" stroke-width="2" viewBox="0 0 24 24">
                                        <path d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" stroke-linecap="round" stroke-linejoin="round"/>
                                    </svg>
                                </div>
                                <div>
                                    <div class="font-bold text-charcoal text-sm" style="color: #1c1c1c;">
                                        "Familienbetrieb"
                                    </div>
                                    <div class="text-gray-400 text-xs">"seit 2010"</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Right: Content
                    <div class="animate-on-scroll delay-200">
                        <p class="text-sage text-sm font-semibold tracking-[0.2em] uppercase mb-3"
                           style="color: #4a9c6d;">
                            "Über uns"
                        </p>

                        <h2 class="font-display font-bold text-charcoal mb-6"
                            style="font-size: clamp(1.875rem, 3.5vw, 2.75rem); color: #1c1c1c; line-height: 1.2;">
                            "Erfahrung, die man sieht –"
                            <br/>
                            <span style="color: #1a3d2b;">"Qualität, die bleibt"</span>
                        </h2>

                        <p class="text-gray-500 leading-relaxed mb-8 text-lg">
                            "Als Familienbetrieb aus der Region kennen wir die Bäume im Rhein-Main-Gebiet wie kaum jemand sonst. Seit 2010 verbinden wir traditionelles Handwerk mit modernster Technik."
                        </p>

                        // USPs
                        <div class="space-y-4 mb-8">
                            {[
                                ("Zertifizierte Fachkräfte (SKT, ISA)", "Unsere Kletterer und Baumkontrolleure sind nach internationalen Standards ausgebildet."),
                                ("Modernste Ausrüstung", "Professionelles Gerät für sichere und schonende Arbeit in jeder Höhe."),
                                ("Versicherter Fachbetrieb", "Vollständig haftpflichtversichert – für Ihre Sicherheit und unsere Professionalität."),
                                ("Kostenlose Erstberatung", "Wir begutachten Ihre Bäume kostenlos und unverbindlich vor Ort."),
                            ].iter().map(|(title, desc)| view! {
                                <div class="flex gap-4">
                                    <div class="flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center mt-0.5"
                                         style="background: rgba(74,156,109,0.15);">
                                        <svg width="14" height="14" fill="none" stroke="#4a9c6d" stroke-width="2.5" viewBox="0 0 24 24">
                                            <path d="M5 13l4 4L19 7" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <div class="font-semibold text-charcoal text-sm mb-0.5"
                                             style="color: #1c1c1c;">
                                            {*title}
                                        </div>
                                        <div class="text-gray-400 text-sm leading-relaxed">
                                            {*desc}
                                        </div>
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>

                        <a href="#kontakt" class="btn-primary inline-flex">
                            "Jetzt Termin vereinbaren"
                        </a>
                    </div>

                </div>
            </div>
        </section>
    }
}
