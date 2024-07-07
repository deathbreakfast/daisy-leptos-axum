use leptos::*;

/// Provides a layout to construct a web page in a single view.
///
/// - Menu Bar - Bar at the top of the page
/// - App Drawer - Collapsable bar on the left side of the page
/// - Details Drawer - Expandable bar on the right side of the page
/// - Banner - Closeable notice on the top of the page
/// - Header - Header above the page body
/// - Body - Main application content in the center of the page
/// - Left Panel - Optional panel to the left of the body
/// - Right panel - Optional panel to the right of the body
/// - Footer - Section below the body
#[component]
pub fn AppPage<C, IV>(
    #[prop(optional)] menu_bar: Option<C>,
    #[prop(optional)] app_drawer: Option<Box<dyn FnOnce() -> Fragment>>,
    #[prop(optional)] details_drawer: Option<Box<dyn Fn() -> dyn IntoView>>,
    #[prop(optional)] banner: Option<Box<dyn Fn() -> dyn IntoView>>,
    #[prop(optional)] header: Option<Box<dyn Fn() -> dyn IntoView>>,
    body: C,
    #[prop(optional)] left_panel: Option<Box<dyn Fn() -> dyn IntoView>>,
    #[prop(optional)] right_panel: Option<Box<dyn Fn() -> dyn IntoView>>,
    #[prop(optional)] footer: Option<Box<dyn Fn() -> dyn IntoView>>,
) -> impl IntoView
where
    C: Fn() -> IV,
    IV: IntoView,
{
    view! { {body()} }
}
