use leptos::*;

struct Service {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

fn get_services() -> Vec<Service> {
    vec![
        Service {
            icon: r#"<svg width="40" height="40" fill="none" stroke="#4a9c6d" stroke-width="1.5" viewBox="0 0 24 24"><path d="M12 22V12M12 12C12 12 7 9 7 5a5 5 0 0110 0c0 4-5 7-5 7z" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 12c0 0 5-3 5-8" stroke-linecap="round" opacity="0.5"/><path d="M12 12c0 0-5-3-5-8" stroke-linecap="round" opacity="0.5"/></svg>"#,
            title: "Baumpflege & Kronenpflege",
            description: "Professioneller Baumschnitt und Kronenpflege für gesundes Wachstum. Wir erhalten die natürliche Form Ihrer Bäume.",
        },
        Service {
            icon: r#"<svg width="40" height="40" fill="none" stroke="#4a9c6d" stroke-width="1.5" viewBox="0 0 24 24"><path d="M6 22h12M12 2v20M4 8l8-6 8 6" stroke-linecap="round" stroke-linejoin="round"/><path d="M9 22V12h6v10" stroke-linecap="round"/></svg>"#,
            title: "Baumfällung & Entsorgung",
            description: "Sichere Fällung auch in schwierigen Lagen. Vollständige Entsorgung des Schnittguts inklusive.",
        },
        Service {
            icon: r#"<svg width="40" height="40" fill="none" stroke="#4a9c6d" stroke-width="1.5" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><path d="M12 8v4l3 3" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 2v2M22 12h-2M12 22v-2M2 12h2" stroke-linecap="round" opacity="0.5"/></svg>"#,
            title: "Baumkontrolle & Gutachten",
            description: "Fachmännische Baumkontrolle nach FLL-Richtlinien. Schriftliche Gutachten für Versicherung und Behörden.",
        },
        Service {
            icon: r#"<svg width="40" height="40" fill="none" stroke="#4a9c6d" stroke-width="1.5" viewBox="0 0 24 24"><path d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A8 8 0 0117.657 18.657z" stroke-linecap="round" stroke-linejoin="round"/><path d="M9.879 16.121A3 3 0 1012 10.5" opacity="0.5" stroke-linecap="round"/></svg>"#,
            title: "Sturmschadenbeseitigung",
            description: "24h Notfallservice nach Sturm. Schnelle und sichere Beseitigung von Sturmschäden an Bäumen.",
        },
        Service {
            icon: r#"<svg width="40" height="40" fill="none" stroke="#4a9c6d" stroke-width="1.5" viewBox="0 0 24 24"><circle cx="12" cy="5" r="3"/><path d="M12 8v14M6 22h12" stroke-linecap="round" stroke-linejoin="round"/><path d="M8 12c-2 1-3 3-3 5M16 12c2 1 3 3 3 5" stroke-linecap="round" opacity="0.6"/></svg>"#,
            title: "Obstbaumschnitt",
            description: "Verjüngungsschnitt und Erziehungsschnitt für Obstbäume. Förderung von Ertrag und Vitalität.",
        },
        Service {
            icon: r#"<svg width="40" height="40" fill="none" stroke="#4a9c6d" stroke-width="1.5" viewBox="0 0 24 24"><path d="M12 22V12M8 18s2-2 4-2 4 2 4 2" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 12c0-4-4-7-4-7s-1 4 1 6c2 2 3 1 3 1z" fill="rgba(74,156,109,0.15)" stroke-linecap="round"/><path d="M12 12c0-4 4-7 4-7s1 4-1 6c-2 2-3 1-3 1z" fill="rgba(74,156,109,0.15)" stroke-linecap="round"/></svg>"#,
            title: "Neupflanzung & Beratung",
            description: "Kompetente Beratung bei der Auswahl und fachgerechte Pflanzung von Bäumen und Sträuchern.",
        },
    ]
}

#[component]
pub fn Services() -> impl IntoView {
    let services = get_services();

    view! {
        <section id="leistungen" class="section-padding bg-cream">
            <div class="max-w-7xl mx-auto px-6">
                // Header
                <div class="text-center mb-16 animate-on-scroll">
                    <p class="text-sage text-sm font-semibold tracking-[0.2em] uppercase mb-3"
                       style="color: #4a9c6d;">
                        "Unsere Leistungen"
                    </p>
                    <h2 class="font-display text-charcoal font-bold mb-4"
                        style="font-size: clamp(1.875rem, 4vw, 3rem); color: #1c1c1c;">
                        "Was wir für Sie tun"
                    </h2>
                    <p class="text-gray-500 text-lg max-w-xl mx-auto">
                        "Von der Routinepflege bis zum Notfalleinsatz – wir sind Ihr zuverlässiger Partner rund um den Baum."
                    </p>
                </div>

                // Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {services.into_iter().enumerate().map(|(i, service)| {
                        let delay_class = match i {
                            0 => "animate-on-scroll delay-100",
                            1 => "animate-on-scroll delay-200",
                            2 => "animate-on-scroll delay-300",
                            3 => "animate-on-scroll delay-400",
                            4 => "animate-on-scroll delay-500",
                            _ => "animate-on-scroll delay-500",
                        };
                        view! {
                            <div class=format!("service-card bg-white rounded-2xl p-8 {}", delay_class)>
                                <div class="mb-5" inner_html=service.icon/>
                                <h3 class="font-display font-semibold text-xl text-charcoal mb-3"
                                    style="color: #1c1c1c;">
                                    {service.title}
                                </h3>
                                <p class="text-gray-500 leading-relaxed text-sm">
                                    {service.description}
                                </p>
                                <div class="mt-5 pt-5 border-t border-gray-100">
                                    <a href="#kontakt"
                                       class="text-sage text-sm font-semibold hover:text-forest transition-colors duration-200 flex items-center gap-1"
                                       style="color: #4a9c6d;">
                                        "Mehr erfahren "
                                        <span>"→"</span>
                                    </a>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
