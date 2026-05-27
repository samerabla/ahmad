use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section
            id="hero"
            class="hero-bg relative flex flex-col items-center justify-center text-center px-6"
            style="min-height: 100vh;"
        >
            // Main Content
            <div class="relative z-10 max-w-4xl mx-auto pt-20">
                <p class="text-sage-light text-sm font-semibold tracking-[0.25em] uppercase mb-6 opacity-90"
                   style="color: #6ab885;">
                    "Rhein-Main-Gebiet"
                </p>

                <h1 class="font-display text-white font-bold leading-tight mb-6"
                    style="font-size: clamp(2.5rem, 6vw, 4.5rem);">
                    "Baumpflege mit "
                    <span style="color: #6ab885; font-style: italic;">"Präzision"</span>
                    " & Leidenschaft"
                </h1>

                <p class="text-white/80 text-lg md:text-xl max-w-2xl mx-auto mb-10 leading-relaxed">
                    "Professionelle Baumpflege im Rhein-Main-Gebiet – zertifiziert, zuverlässig, nachhaltig."
                </p>

                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-16">
                    <a href="#kontakt" class="btn-primary">
                        <svg width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07A19.5 19.5 0 013.95 12a19.79 19.79 0 01-3.07-8.67A2 2 0 012.86 1h3a2 2 0 012 1.72c.127.96.361 1.903.7 2.81a2 2 0 01-.45 2.11L7.09 8.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0122 16.92z"/>
                        </svg>
                        "Kostenlos anfragen"
                    </a>
                    <a href="#leistungen" class="btn-outline">
                        "Leistungen entdecken"
                        <svg width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path d="M7 13l3 3 7-7" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </a>
                </div>

                // Trust Badges
                <div class="flex flex-wrap justify-center gap-6 md:gap-10">
                    <div class="flex items-center gap-2 text-white/75 text-sm">
                        <span class="text-green-300 font-bold text-base">"✓"</span>
                        "15+ Jahre Erfahrung"
                    </div>
                    <div class="w-px bg-white/20 hidden md:block"/>
                    <div class="flex items-center gap-2 text-white/75 text-sm">
                        <span class="text-green-300 font-bold text-base">"✓"</span>
                        "ISA-zertifiziert"
                    </div>
                    <div class="w-px bg-white/20 hidden md:block"/>
                    <div class="flex items-center gap-2 text-white/75 text-sm">
                        <span class="text-green-300 font-bold text-base">"✓"</span>
                        "500+ zufriedene Kunden"
                    </div>
                </div>
            </div>

            // Scroll Indicator
            <div class="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 text-white/50">
                <span class="text-xs tracking-widest">"SCROLL"</span>
                <div class="w-px h-12 bg-white/30 relative overflow-hidden">
                    <div class="absolute top-0 left-0 right-0 h-4 bg-white/60"
                         style="animation: scrollDot 2s ease-in-out infinite;">
                    </div>
                </div>
            </div>
        </section>
    }
}
