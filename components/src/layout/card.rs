use leptos::*;

enum CardImageLocation {
    Upper,
    Lower,
}

/// Cards are used to group and display content in a way that is easily readable.
/// Cards are composed of a
#[component]
pub fn Card(
    children: Children,
    #[prop(optional)] image: Option<fn() -> View>,
    #[prop(default = CardImageLocation::Upper)] imageLocation: CardImageLocation,
    #[prop(optional)] actionButtons: Option<fn() -> View>,
) -> impl IntoView {
    view! {
        <div class="card shadow-xl">
            {match imageLocation {
                CardImageLocation::Upper => {
                    Some(
                        view! {
                            {match image {
                                Some(img) => img(),
                                None => view! { <div></div> }.into_view(),
                            }}
                        },
                    )
                }
                CardImageLocation::Lower => None,
            }}
            <div class="card-body">{children()}</div>
            {match imageLocation {
                CardImageLocation::Lower => {
                    Some(
                        view! {
                            {match image {
                                Some(img) => img(),
                                None => view! { <div></div> }.into_view(),
                            }}
                        },
                    )
                }
                CardImageLocation::Upper => None,
            }}
            {match actionButtons {
                Some(buttons) => view! { <div class="card-actions justify-end">{buttons()}</div> },
                None => view! { <div></div> },
            }}

        </div>
    }
}
