use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);
    let (scrolled, set_scrolled) = create_signal(false);

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        create_effect(move |_| {
            let window = web_sys::window().unwrap();
            let closure = Closure::wrap(Box::new(move || {
                let win = web_sys::window().unwrap();
                let scroll_y = win.scroll_y().unwrap_or(0.0);
                set_scrolled.set(scroll_y > 60.0);
            }) as Box<dyn Fn()>);

            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        });
    }

    let nav_class = move || {
        if scrolled.get() {
            "fixed top-0 left-0 right-0 z-50 nav-white transition-all duration-300"
        } else {
            "fixed top-0 left-0 right-0 z-50 nav-transparent transition-all duration-300"
        }
    };

    let link_class = move || {
        if scrolled.get() {
            "text-charcoal hover:text-sage transition-colors duration-200 font-medium text-sm"
        } else {
            "text-white hover:text-green-200 transition-colors duration-200 font-medium text-sm"
        }
    };

    let logo_class = move || {
        if scrolled.get() {
            "font-display text-2xl font-bold text-forest"
        } else {
            "font-display text-2xl font-bold text-white"
        }
    };

    view! {
        <nav id="main-nav" class=nav_class>
            <div class="max-w-7xl mx-auto px-6 h-20 flex items-center justify-between">
                // Logo
                <a href="#" class=logo_class>
                    "treetment®"
                </a>

                // Desktop Menu
                <div class="hidden md:flex items-center gap-8">
                    <a href="#leistungen" class=link_class>"Leistungen"</a>
                    <a href="#ueber-uns" class=link_class>"Über uns"</a>
                    <a href="#referenzen" class=link_class>"Referenzen"</a>
                    <a href="#kontakt" class=link_class>"Kontakt"</a>
                    <a href="#kontakt" class="btn-primary text-sm py-2.5 px-5">
                        "Angebot anfragen"
                    </a>
                </div>

                // Mobile Hamburger
                <button
                    class="md:hidden flex flex-col gap-1.5 p-2"
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    aria-label="Menü öffnen"
                >
                    <span class=move || {
                        if menu_open.get() {
                            "hamburger-line translate-y-[7px] rotate-45"
                        } else {
                            "hamburger-line"
                        }
                    }
                    style=move || if scrolled.get() { "" } else { "background: white;" }
                    />
                    <span class=move || {
                        if menu_open.get() {
                            "hamburger-line opacity-0"
                        } else {
                            "hamburger-line"
                        }
                    }
                    style=move || if scrolled.get() { "" } else { "background: white;" }
                    />
                    <span class=move || {
                        if menu_open.get() {
                            "hamburger-line -translate-y-[7px] -rotate-45"
                        } else {
                            "hamburger-line"
                        }
                    }
                    style=move || if scrolled.get() { "" } else { "background: white;" }
                    />
                </button>
            </div>

            // Mobile Menu Drawer
            <div class=move || {
                if menu_open.get() {
                    "md:hidden bg-white border-t border-gray-100 overflow-hidden transition-all duration-300"
                } else {
                    "md:hidden bg-white border-t border-gray-100 overflow-hidden max-h-0 transition-all duration-300"
                }
            }>
                <div class="px-6 py-4 flex flex-col gap-4">
                    <a
                        href="#leistungen"
                        class="text-charcoal font-medium py-2 border-b border-gray-100"
                        on:click=move |_| set_menu_open.set(false)
                    >
                        "Leistungen"
                    </a>
                    <a
                        href="#ueber-uns"
                        class="text-charcoal font-medium py-2 border-b border-gray-100"
                        on:click=move |_| set_menu_open.set(false)
                    >
                        "Über uns"
                    </a>
                    <a
                        href="#referenzen"
                        class="text-charcoal font-medium py-2 border-b border-gray-100"
                        on:click=move |_| set_menu_open.set(false)
                    >
                        "Referenzen"
                    </a>
                    <a
                        href="#kontakt"
                        class="text-charcoal font-medium py-2 border-b border-gray-100"
                        on:click=move |_| set_menu_open.set(false)
                    >
                        "Kontakt"
                    </a>
                    <a
                        href="#kontakt"
                        class="btn-primary text-center mt-2"
                        on:click=move |_| set_menu_open.set(false)
                    >
                        "Angebot anfragen"
                    </a>
                </div>
            </div>
        </nav>
    }
}
