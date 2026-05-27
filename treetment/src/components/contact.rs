use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub service: String,
    pub message: String,
}

#[server(SubmitContactForm, "/api")]
pub async fn submit_contact_form(
    name: String,
    phone: String,
    email: String,
    service: String,
    message: String,
) -> Result<String, ServerFnError> {
    use crate::db;

    if name.trim().is_empty() {
        return Err(ServerFnError::ServerError("Name ist erforderlich".to_string()));
    }
    if email.trim().is_empty() || !email.contains('@') {
        return Err(ServerFnError::ServerError("Gültige E-Mail-Adresse erforderlich".to_string()));
    }
    if message.trim().is_empty() {
        return Err(ServerFnError::ServerError("Nachricht ist erforderlich".to_string()));
    }

    use leptos_axum::extract;
    use axum::Extension;

    let Extension(pool): Extension<sqlx::PgPool> = extract().await
        .map_err(|e| ServerFnError::ServerError(format!("DB-Fehler: {e}")))?;

    db::insert_contact_request(&pool, name, phone, email, service, message)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Speicherfehler: {e}")))?;

    Ok("Vielen Dank! Wir melden uns innerhalb von 24 Stunden.".to_string())
}

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (phone, set_phone) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (service, set_service) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (privacy, set_privacy) = create_signal(false);
    let (submitting, set_submitting) = create_signal(false);
    let (success_msg, set_success_msg) = create_signal::<Option<String>>(None);
    let (error_msg, set_error_msg) = create_signal::<Option<String>>(None);

    let submit = create_action(move |_: &()| {
        let n = name.get();
        let p = phone.get();
        let e = email.get();
        let s = service.get();
        let m = message.get();
        set_submitting.set(true);
        set_success_msg.set(None);
        set_error_msg.set(None);

        async move {
            let result = submit_contact_form(n, p, e, s, m).await;
            set_submitting.set(false);
            match result {
                Ok(msg) => {
                    set_success_msg.set(Some(msg));
                    set_name.set(String::new());
                    set_phone.set(String::new());
                    set_email.set(String::new());
                    set_service.set(String::new());
                    set_message.set(String::new());
                    set_privacy.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if !privacy.get() {
            set_error_msg.set(Some("Bitte stimmen Sie der Datenschutzerklärung zu.".to_string()));
            return;
        }
        submit.dispatch(());
    };

    view! {
        <section id="kontakt" class="section-padding" style="background: #f9f6f0;">
            <div class="max-w-7xl mx-auto px-6">

                // Header
                <div class="text-center mb-16 animate-on-scroll">
                    <p class="text-sage text-sm font-semibold tracking-[0.2em] uppercase mb-3"
                       style="color: #4a9c6d;">
                        "Kontakt"
                    </p>
                    <h2 class="font-display font-bold text-charcoal mb-4"
                        style="font-size: clamp(1.875rem, 4vw, 3rem); color: #1c1c1c;">
                        "Jetzt Ihr kostenloses Angebot anfragen"
                    </h2>
                    <p class="text-gray-500 text-lg max-w-xl mx-auto">
                        "Wir antworten innerhalb von 24 Stunden – garantiert."
                    </p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">

                    // Form
                    <div class="bg-white rounded-2xl p-8 shadow-sm animate-on-scroll">
                        <form on:submit=on_submit class="space-y-5">

                            // Name + Phone Row
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-charcoal mb-1.5"
                                           style="color: #1c1c1c;">
                                        "Ihr Name *"
                                    </label>
                                    <input
                                        type="text"
                                        class="form-input"
                                        placeholder="Max Mustermann"
                                        required
                                        prop:value=name
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-charcoal mb-1.5"
                                           style="color: #1c1c1c;">
                                        "Telefon"
                                    </label>
                                    <input
                                        type="tel"
                                        class="form-input"
                                        placeholder="+49 611 ..."
                                        prop:value=phone
                                        on:input=move |ev| set_phone.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>

                            // Email
                            <div>
                                <label class="block text-sm font-medium text-charcoal mb-1.5"
                                       style="color: #1c1c1c;">
                                    "E-Mail-Adresse *"
                                </label>
                                <input
                                    type="email"
                                    class="form-input"
                                    placeholder="max@example.de"
                                    required
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                />
                            </div>

                            // Service Dropdown
                            <div>
                                <label class="block text-sm font-medium text-charcoal mb-1.5"
                                       style="color: #1c1c1c;">
                                    "Gewünschte Leistung"
                                </label>
                                <select
                                    class="form-input"
                                    prop:value=service
                                    on:change=move |ev| set_service.set(event_target_value(&ev))
                                >
                                    <option value="">"Leistung wählen..."</option>
                                    <option value="baumpflege">"Baumpflege & Kronenpflege"</option>
                                    <option value="baumfaellung">"Baumfällung & Entsorgung"</option>
                                    <option value="kontrolle">"Baumkontrolle & Gutachten"</option>
                                    <option value="sturm">"Sturmschadenbeseitigung"</option>
                                    <option value="obstbaumschnitt">"Obstbaumschnitt"</option>
                                    <option value="neupflanzung">"Neupflanzung & Beratung"</option>
                                    <option value="sonstiges">"Sonstiges"</option>
                                </select>
                            </div>

                            // Message
                            <div>
                                <label class="block text-sm font-medium text-charcoal mb-1.5"
                                       style="color: #1c1c1c;">
                                    "Ihre Nachricht *"
                                </label>
                                <textarea
                                    class="form-input"
                                    rows="4"
                                    placeholder="Beschreiben Sie kurz Ihr Anliegen..."
                                    required
                                    prop:value=message
                                    on:input=move |ev| set_message.set(event_target_value(&ev))
                                />
                            </div>

                            // Privacy Checkbox
                            <div class="flex items-start gap-3">
                                <input
                                    type="checkbox"
                                    id="privacy"
                                    class="mt-0.5 w-4 h-4 accent-sage cursor-pointer"
                                    style="accent-color: #4a9c6d;"
                                    prop:checked=privacy
                                    on:change=move |ev| set_privacy.set(event_target_checked(&ev))
                                />
                                <label for="privacy" class="text-sm text-gray-500 cursor-pointer">
                                    "Ich stimme der "
                                    <a href="#" class="underline text-sage" style="color: #4a9c6d;">
                                        "Datenschutzerklärung"
                                    </a>
                                    " zu und bin damit einverstanden, dass meine Daten zur Bearbeitung meiner Anfrage verwendet werden. *"
                                </label>
                            </div>

                            // Messages
                            {move || success_msg.get().map(|msg| view! {
                                <div class="p-4 rounded-xl text-sm font-medium"
                                     style="background: rgba(74,156,109,0.1); color: #1a3d2b;">
                                    <div class="flex items-center gap-2">
                                        <svg width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                        {msg}
                                    </div>
                                </div>
                            })}

                            {move || error_msg.get().map(|msg| view! {
                                <div class="p-4 rounded-xl text-sm font-medium"
                                     style="background: rgba(220,38,38,0.08); color: #dc2626;">
                                    <div class="flex items-center gap-2">
                                        <svg width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                            <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
                                        </svg>
                                        {msg}
                                    </div>
                                </div>
                            })}

                            // Submit Button
                            <button
                                type="submit"
                                class="btn-primary w-full justify-center"
                                disabled=move || submitting.get()
                            >
                                {move || if submitting.get() {
                                    view! {
                                        <>
                                            <div class="spinner"/>
                                            "Wird gesendet..."
                                        </>
                                    }
                                } else {
                                    view! {
                                        <>
                                            <svg width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                                <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z" stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                            "Anfrage senden"
                                        </>
                                    }
                                }}
                            </button>

                        </form>
                    </div>

                    // Contact Info Card
                    <div class="space-y-6 animate-on-scroll delay-200">

                        // Contact Details
                        <div class="bg-white rounded-2xl p-8 shadow-sm">
                            <h3 class="font-display font-bold text-xl mb-6"
                                style="color: #1c1c1c;">
                                "Direkt Kontakt aufnehmen"
                            </h3>

                            <div class="space-y-5">
                                <div class="flex items-start gap-4">
                                    <div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                                         style="background: rgba(26,61,43,0.08);">
                                        <svg width="20" height="20" fill="none" stroke="#1a3d2b" stroke-width="2" viewBox="0 0 24 24">
                                            <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07A19.5 19.5 0 013.95 12a19.79 19.79 0 01-3.07-8.67A2 2 0 012.86 1h3a2 2 0 012 1.72c.127.96.361 1.903.7 2.81a2 2 0 01-.45 2.11L7.09 8.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0122 16.92z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <div class="text-xs text-gray-400 font-medium uppercase tracking-wider mb-0.5">
                                            "Telefon"
                                        </div>
                                        <a href="tel:+4961112345678"
                                           class="font-semibold text-charcoal hover:text-sage transition-colors"
                                           style="color: #1c1c1c;">
                                            "+49 611 12345678"
                                        </a>
                                        <div class="text-xs text-sage mt-0.5" style="color: #4a9c6d;">
                                            "24h Notfallservice verfügbar"
                                        </div>
                                    </div>
                                </div>

                                <div class="flex items-start gap-4">
                                    <div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                                         style="background: rgba(26,61,43,0.08);">
                                        <svg width="20" height="20" fill="none" stroke="#1a3d2b" stroke-width="2" viewBox="0 0 24 24">
                                            <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <div class="text-xs text-gray-400 font-medium uppercase tracking-wider mb-0.5">
                                            "E-Mail"
                                        </div>
                                        <a href="mailto:info@treetment.de"
                                           class="font-semibold text-charcoal hover:text-sage transition-colors"
                                           style="color: #1c1c1c;">
                                            "info@treetment.de"
                                        </a>
                                    </div>
                                </div>

                                <div class="flex items-start gap-4">
                                    <div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                                         style="background: rgba(26,61,43,0.08);">
                                        <svg width="20" height="20" fill="none" stroke="#1a3d2b" stroke-width="2" viewBox="0 0 24 24">
                                            <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0118 0z"/><circle cx="12" cy="10" r="3"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <div class="text-xs text-gray-400 font-medium uppercase tracking-wider mb-0.5">
                                            "Adresse"
                                        </div>
                                        <div class="font-semibold text-charcoal" style="color: #1c1c1c;">
                                            "Musterstraße 42"
                                        </div>
                                        <div class="text-gray-400 text-sm">
                                            "65195 Wiesbaden"
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Opening Hours
                        <div class="bg-white rounded-2xl p-8 shadow-sm">
                            <h3 class="font-display font-bold text-xl mb-5"
                                style="color: #1c1c1c;">
                                "Öffnungszeiten"
                            </h3>
                            <div class="space-y-3">
                                {[
                                    ("Mo – Fr", "07:00 – 18:00 Uhr", true),
                                    ("Samstag", "08:00 – 14:00 Uhr", true),
                                    ("Sonntag", "Geschlossen", false),
                                ].iter().map(|(day, hours, open)| view! {
                                    <div class="flex justify-between items-center py-2 border-b border-gray-50 last:border-0">
                                        <span class="text-gray-500 text-sm">{*day}</span>
                                        <span class=if *open {
                                            "text-sm font-semibold"
                                        } else {
                                            "text-sm text-gray-400"
                                        }
                                        style=if *open { "color: #1c1c1c;" } else { "" }>
                                            {*hours}
                                        </span>
                                    </div>
                                }).collect_view()}
                            </div>
                            <div class="mt-4 p-3 rounded-lg text-xs font-medium"
                                 style="background: rgba(26,61,43,0.06); color: #1a3d2b;">
                                "Notfälle: 24h erreichbar unter +49 611 12345678"
                            </div>
                        </div>

                    </div>
                </div>
            </div>
        </section>
    }
}
