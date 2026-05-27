use leptos::*;

struct Testimonial {
    name: &'static str,
    location: &'static str,
    text: &'static str,
    initials: &'static str,
    color: &'static str,
}

fn get_testimonials() -> Vec<Testimonial> {
    vec![
        Testimonial {
            name: "Michael Braun",
            location: "Wiesbaden-Biebrich",
            text: "Absolut professionelle Arbeit! Die Kronenpflege unserer alten Eiche wurde mit großer Sorgfalt durchgeführt. Das Team war pünktlich, sauber und sehr kompetent. Klare Empfehlung!",
            initials: "MB",
            color: "#1a3d2b",
        },
        Testimonial {
            name: "Sabine Hoffmann",
            location: "Mainz-Gonsenheim",
            text: "Nach dem Sturm war treetment® schnell zur Stelle. Innerhalb von 2 Stunden war der gefährliche Ast sicher entfernt. Faire Preise, hervorragende Qualität. Nie wieder jemand anderen!",
            initials: "SH",
            color: "#4a9c6d",
        },
        Testimonial {
            name: "Dr. Thomas Richter",
            location: "Rüdesheim am Rhein",
            text: "Für unser Weingut betreut treetment® alle Obstbäume. Das Ergebnis ist beeindruckend – mehr Ertrag, gesündere Bäume. Das Gutachten war sehr professionell und hilfreich.",
            initials: "TR",
            color: "#2d6b47",
        },
    ]
}

#[component]
pub fn Testimonials() -> impl IntoView {
    let testimonials = get_testimonials();

    view! {
        <section id="referenzen" class="section-padding"
                 style="background: linear-gradient(135deg, #f9f6f0 0%, #eef5f1 100%);">
            <div class="max-w-7xl mx-auto px-6">

                // Header
                <div class="text-center mb-16 animate-on-scroll">
                    <p class="text-sage text-sm font-semibold tracking-[0.2em] uppercase mb-3"
                       style="color: #4a9c6d;">
                        "Kundenstimmen"
                    </p>
                    <h2 class="font-display font-bold text-charcoal"
                        style="font-size: clamp(1.875rem, 4vw, 3rem); color: #1c1c1c;">
                        "Das sagen unsere Kunden"
                    </h2>
                </div>

                // Cards Grid
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {testimonials.into_iter().enumerate().map(|(i, t)| {
                        let delay_class = match i {
                            0 => "animate-on-scroll delay-100",
                            1 => "animate-on-scroll delay-200",
                            _ => "animate-on-scroll delay-300",
                        };
                        let bg_color = t.color;
                        let initials = t.initials;
                        view! {
                            <div class=format!("testimonial-card {}", delay_class)>
                                // Stars
                                <div class="flex gap-1 mb-4">
                                    {(0..5).map(|_| view! {
                                        <svg width="18" height="18" viewBox="0 0 24 24" fill="#f59e0b">
                                            <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                                        </svg>
                                    }).collect_view()}
                                </div>

                                // Quote
                                <p class="text-gray-600 leading-relaxed text-sm mb-6 italic">
                                    "\""
                                    {t.text}
                                    "\""
                                </p>

                                // Author
                                <div class="flex items-center gap-3 pt-4 border-t border-gray-100">
                                    <div class="w-10 h-10 rounded-full flex items-center justify-center text-white text-sm font-bold flex-shrink-0"
                                         style=format!("background: {};", bg_color)>
                                        {initials}
                                    </div>
                                    <div>
                                        <div class="font-semibold text-charcoal text-sm"
                                             style="color: #1c1c1c;">
                                            {t.name}
                                        </div>
                                        <div class="text-gray-400 text-xs">{t.location}</div>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>

                // CTA
                <div class="text-center mt-12 animate-on-scroll">
                    <p class="text-gray-500 mb-4">
                        "Überzeugt? Werden Sie unser nächster zufriedener Kunde."
                    </p>
                    <a href="#kontakt" class="btn-primary inline-flex">
                        "Kostenloses Angebot anfragen"
                    </a>
                </div>

            </div>
        </section>
    }
}
