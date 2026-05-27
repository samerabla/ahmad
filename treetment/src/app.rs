use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    about::About,
    contact::Contact,
    footer::Footer,
    hero::Hero,
    map::MapSection,
    nav::Nav,
    services::Services,
    stats::Stats,
    testimonials::Testimonials,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/treetment.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=Playfair+Display:ital,wght@0,400;0,600;0,700;1,400&display=swap"
            rel="stylesheet"
        />
        <Title text="treetment® – Ihre Bäume. Unsere Leidenschaft."/>
        <Meta name="description" content="Professionelle Baumpflege im Rhein-Main-Gebiet – zertifiziert, zuverlässig, nachhaltig."/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <>
            <Nav/>
            <main>
                <Hero/>
                <Services/>
                <About/>
                <Stats/>
                <Testimonials/>
                <MapSection/>
                <Contact/>
            </main>
            <Footer/>
        </>
    }
}
