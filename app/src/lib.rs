use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <div  data-theme={"dark"}>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>
                <main>
                    <Routes>
                        <Route path="" view=HomePage/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="container mx-auto ">
            <div class="flex flex-col items-center"> 
                <div>
                    <h1 class="font-bold text-2xl grow">"Welcome to Leptos!"</h1>
                </div>
                <div>
                    <button 
                        class="btn btn-primary"
                        on:click=on_click>
                        "Click Me: " {count}
                    </button>
                </div>
            </div> 
        </div>
    }
}
