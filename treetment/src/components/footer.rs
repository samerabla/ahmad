use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer style="background: #1a3d2b;">
            <div class="max-w-7xl mx-auto px-6 py-16">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-10 mb-12">

                    // Brand Column
                    <div class="md:col-span-1">
                        <div class="font-display text-2xl font-bold text-white mb-3">
                            "treetment®"
                        </div>
                        <p class="text-white/60 text-sm leading-relaxed mb-5">
                            "Ihre Bäume. Unsere Leidenschaft."
                        </p>
                        <p class="text-white/50 text-xs leading-relaxed">
                            "Professionelle Baumpflege im Rhein-Main-Gebiet seit 2010."
                        </p>
                        // Social Links
                        <div class="flex gap-3 mt-5">
                            <a href="#"
                               class="w-9 h-9 rounded-lg flex items-center justify-center transition-colors"
                               style="background: rgba(255,255,255,0.08);"
                               aria-label="Facebook">
                                <svg width="16" height="16" fill="white" viewBox="0 0 24 24">
                                    <path d="M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z"/>
                                </svg>
                            </a>
                            <a href="#"
                               class="w-9 h-9 rounded-lg flex items-center justify-center transition-colors"
                               style="background: rgba(255,255,255,0.08);"
                               aria-label="Instagram">
                                <svg width="16" height="16" fill="none" stroke="white" stroke-width="1.5" viewBox="0 0 24 24">
                                    <rect x="2" y="2" width="20" height="20" rx="5" ry="5"/><path d="M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37z"/><line x1="17.5" y1="6.5" x2="17.51" y2="6.5"/>
                                </svg>
                            </a>
                        </div>
                    </div>

                    // Navigation Links
                    <div>
                        <h4 class="text-white font-semibold mb-5 text-sm tracking-wider uppercase">
                            "Navigation"
                        </h4>
                        <ul class="space-y-3">
                            {["Leistungen", "Über uns", "Referenzen", "Kontakt"].iter().zip(
                                ["#leistungen", "#ueber-uns", "#referenzen", "#kontakt"]
                            ).map(|(label, href)| view! {
                                <li>
                                    <a href=href
                                       class="text-white/60 text-sm hover:text-white transition-colors duration-200">
                                        {*label}
                                    </a>
                                </li>
                            }).collect_view()}
                        </ul>
                    </div>

                    // Services Links
                    <div>
                        <h4 class="text-white font-semibold mb-5 text-sm tracking-wider uppercase">
                            "Leistungen"
                        </h4>
                        <ul class="space-y-3">
                            {[
                                "Baumpflege & Kronenpflege",
                                "Baumfällung",
                                "Baumkontrolle",
                                "Sturmschadenbeseitigung",
                                "Obstbaumschnitt",
                                "Neupflanzung",
                            ].iter().map(|label| view! {
                                <li>
                                    <a href="#leistungen"
                                       class="text-white/60 text-sm hover:text-white transition-colors duration-200">
                                        {*label}
                                    </a>
                                </li>
                            }).collect_view()}
                        </ul>
                    </div>

                    // Contact Info
                    <div>
                        <h4 class="text-white font-semibold mb-5 text-sm tracking-wider uppercase">
                            "Kontakt"
                        </h4>
                        <div class="space-y-3">
                            <div class="flex items-start gap-2">
                                <svg width="14" height="14" fill="none" stroke="rgba(255,255,255,0.5)" stroke-width="2" viewBox="0 0 24 24" class="mt-0.5 flex-shrink-0">
                                    <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0118 0z"/><circle cx="12" cy="10" r="3"/>
                                </svg>
                                <span class="text-white/60 text-sm">
                                    "Musterstraße 42"
                                    <br/>
                                    "65195 Wiesbaden"
                                </span>
                            </div>
                            <div class="flex items-center gap-2">
                                <svg width="14" height="14" fill="none" stroke="rgba(255,255,255,0.5)" stroke-width="2" viewBox="0 0 24 24" class="flex-shrink-0">
                                    <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07A19.5 19.5 0 013.95 12a19.79 19.79 0 01-3.07-8.67A2 2 0 012.86 1h3a2 2 0 012 1.72c.127.96.361 1.903.7 2.81a2 2 0 01-.45 2.11L7.09 8.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0122 16.92z"/>
                                </svg>
                                <a href="tel:+4961112345678" class="text-white/60 text-sm hover:text-white transition-colors">
                                    "+49 611 12345678"
                                </a>
                            </div>
                            <div class="flex items-center gap-2">
                                <svg width="14" height="14" fill="none" stroke="rgba(255,255,255,0.5)" stroke-width="2" viewBox="0 0 24 24" class="flex-shrink-0">
                                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/>
                                </svg>
                                <a href="mailto:info@treetment.de" class="text-white/60 text-sm hover:text-white transition-colors">
                                    "info@treetment.de"
                                </a>
                            </div>
                        </div>
                    </div>

                </div>

                // Bottom Bar
                <div class="pt-8 border-t flex flex-col md:flex-row justify-between items-center gap-4"
                     style="border-color: rgba(255,255,255,0.1);">
                    <p class="text-white/40 text-sm">
                        "© 2025 treetment® – Alle Rechte vorbehalten"
                    </p>
                    <div class="flex gap-6">
                        <a href="#" class="text-white/40 text-sm hover:text-white/70 transition-colors">
                            "Datenschutz"
                        </a>
                        <a href="#" class="text-white/40 text-sm hover:text-white/70 transition-colors">
                            "Impressum"
                        </a>
                        <a href="#" class="text-white/40 text-sm hover:text-white/70 transition-colors">
                            "AGB"
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
