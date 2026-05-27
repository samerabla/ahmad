use leptos::*;

#[component]
pub fn MapSection() -> impl IntoView {
    view! {
        <section id="einzugsgebiet" class="section-padding bg-white">
            <div class="max-w-7xl mx-auto px-6">

                <div class="text-center mb-12 animate-on-scroll">
                    <p class="text-sage text-sm font-semibold tracking-[0.2em] uppercase mb-3"
                       style="color: #4a9c6d;">
                        "Unser Einzugsgebiet"
                    </p>
                    <h2 class="font-display font-bold text-charcoal"
                        style="font-size: clamp(1.875rem, 4vw, 3rem); color: #1c1c1c;">
                        "Wir kommen zu Ihnen"
                    </h2>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">

                    // Map
                    <div class="map-container animate-on-scroll" style="aspect-ratio: 4/3;">
                        <iframe
                            src="https://www.openstreetmap.org/export/embed.html?bbox=7.8%2C49.85%2C8.6%2C50.15&layer=mapnik&marker=49.997068%2C8.2414101"
                            style="width:100%;height:100%;border:0;"
                            loading="lazy"
                            title="Einzugsgebiet treetment® – Wiesbaden/Rhein-Main"
                        />
                    </div>

                    // Info
                    <div class="animate-on-scroll delay-200">
                        <h3 class="font-display font-bold text-xl text-charcoal mb-6"
                            style="color: #1c1c1c;">
                            "Unser Einzugsgebiet"
                        </h3>

                        <div class="space-y-4 mb-8">
                            {[
                                ("Wiesbaden", "Stadtgebiet & alle Stadtteile"),
                                ("Mainz", "Mainz & Mainz-Bingen"),
                                ("Rheingau", "Rüdesheim, Geisenheim, Eltville"),
                                ("Rhein-Main", "Frankfurt, Darmstadt, Rüsselsheim"),
                                ("Taunus", "Bad Schwalbach, Idstein, Limburg"),
                            ].iter().map(|(city, region)| view! {
                                <div class="flex items-start gap-3 p-4 rounded-xl bg-cream"
                                     style="background: #f9f6f0;">
                                    <div class="w-2 h-2 rounded-full mt-2 flex-shrink-0"
                                         style="background: #4a9c6d;">
                                    </div>
                                    <div>
                                        <span class="font-semibold text-charcoal" style="color: #1c1c1c;">
                                            {*city}
                                        </span>
                                        <span class="text-gray-400 ml-2 text-sm">{*region}</span>
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>

                        <div class="p-5 rounded-xl border-l-4"
                             style="background: rgba(26,61,43,0.05); border-color: #1a3d2b;">
                            <p class="text-sm text-charcoal font-medium mb-1" style="color: #1c1c1c;">
                                "Außerhalb des Einzugsgebiets?"
                            </p>
                            <p class="text-sm text-gray-500">
                                "Sprechen Sie uns an – bei größeren Projekten kommen wir auch weiter."
                            </p>
                        </div>
                    </div>

                </div>
            </div>
        </section>
    }
}
