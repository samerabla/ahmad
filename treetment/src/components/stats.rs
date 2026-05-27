use leptos::*;

#[derive(Clone)]
struct Stat {
    value: u32,
    suffix: &'static str,
    label: &'static str,
    sublabel: &'static str,
}

#[component]
pub fn Stats() -> impl IntoView {
    let stats = vec![
        Stat { value: 500, suffix: "+", label: "Bäume gepflegt", sublabel: "in 15 Jahren" },
        Stat { value: 15, suffix: "", label: "Jahre Erfahrung", sublabel: "seit 2010" },
        Stat { value: 98, suffix: "%", label: "Kundenzufriedenheit", sublabel: "verifizierte Bewertungen" },
        Stat { value: 24, suffix: "h", label: "Notfallservice", sublabel: "immer erreichbar" },
    ];

    let (counters, set_counters) = create_signal(vec![0u32; 4]);
    let (started, set_started) = create_signal(false);

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        let stats_clone = stats.clone();

        create_effect(move |_| {
            let targets: Vec<u32> = stats_clone.iter().map(|s| s.value).collect();
            let set_counters = set_counters.clone();
            let set_started = set_started.clone();

            let document = web_sys::window().unwrap().document().unwrap();
            if let Some(section) = document.get_element_by_id("stats-section") {
                let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                    let entry = entries.get(0);
                    let entry: web_sys::IntersectionObserverEntry = entry.dyn_into().unwrap();
                    if entry.is_intersecting() && !started.get_untracked() {
                        set_started.set(true);
                        let targets = targets.clone();
                        let set_counters = set_counters.clone();
                        animate_counters(targets, set_counters);
                    }
                }) as Box<dyn Fn(js_sys::Array)>);

                let observer = web_sys::IntersectionObserver::new(
                    callback.as_ref().unchecked_ref()
                ).unwrap();
                observer.observe(&section);
                callback.forget();
            }
        });
    }

    view! {
        <section
            id="stats-section"
            class="section-padding"
            style="background: #1a3d2b;"
        >
            <div class="max-w-7xl mx-auto px-6">
                <div class="grid grid-cols-2 lg:grid-cols-4 gap-8 text-center">
                    {stats.iter().enumerate().map(|(i, stat)| {
                        let suffix = stat.suffix;
                        let label = stat.label;
                        let sublabel = stat.sublabel;
                        view! {
                            <div class="animate-on-scroll">
                                <div class="stat-number">
                                    {move || counters.get()[i]}
                                    {suffix}
                                </div>
                                <div class="text-white font-semibold mt-2 text-base">
                                    {label}
                                </div>
                                <div class="text-white/50 text-sm mt-1">
                                    {sublabel}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

#[cfg(feature = "hydrate")]
fn animate_counters(targets: Vec<u32>, set_counters: WriteSignal<Vec<u32>>) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    let total_frames = 60u32;
    let frame_duration = 33; // ~30fps over 2 seconds

    let frame = std::rc::Rc::new(std::cell::Cell::new(0u32));
    let frame_clone = frame.clone();
    let targets = std::rc::Rc::new(targets);
    let targets_clone = targets.clone();

    let closure: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn Fn()>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let closure_clone = closure.clone();

    *closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let f = frame_clone.get();
        if f <= total_frames {
            let progress = f as f64 / total_frames as f64;
            let eased = 1.0 - (1.0 - progress).powi(3);
            let new_values: Vec<u32> = targets_clone
                .iter()
                .map(|&t| (t as f64 * eased) as u32)
                .collect();
            set_counters.set(new_values);
            frame_clone.set(f + 1);

            let win = web_sys::window().unwrap();
            if let Some(c) = closure_clone.borrow().as_ref() {
                win.set_timeout_with_callback_and_timeout_and_arguments_0(
                    c.as_ref().unchecked_ref(),
                    frame_duration,
                ).unwrap();
            }
        } else {
            set_counters.set(targets_clone.as_ref().clone());
        }
    }) as Box<dyn Fn()>));

    let win = web_sys::window().unwrap();
    if let Some(c) = closure.borrow().as_ref() {
        win.set_timeout_with_callback_and_timeout_and_arguments_0(
            c.as_ref().unchecked_ref(),
            frame_duration,
        ).unwrap();
    }
    // Leak the closure to keep it alive
    std::mem::forget(closure);
}
