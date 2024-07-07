use leptos::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::navigation_panel::*;
use crate::icons::*;
use crate::pb_app::*;

pub mod error_template;
pub mod navigation_panel;
pub mod app_bar;
pub mod icons;
pub mod pb_app;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <PBApp theme=Theme::Dark>
            <SideNavigation slot>
                <CollapsableNavigationSection slot 
                    icon=Some(SVGIcons::Book(18, false))
                    title="Docs".to_owned()>
                    <NavigationItem slot 
                        label="Use".to_owned() 
                        path=Some("/use".to_owned()) />
                    <NavigationItem slot 
                        label="Themes".to_owned()
                        path=Some("/themes".to_owned()) />
                </CollapsableNavigationSection>
                <CollapsableNavigationSection slot 
                    icon=Some(SVGIcons::Shapes(18, false))
                    title="Components".to_owned()>
                    <NavigationSection slot 
                        icon=Some(SVGIcons::Click(14, false))
                        title="Actions".to_owned()>
                        <NavigationItem slot label="Button".to_owned()/>
                        <NavigationItem slot label="Dropdown".to_owned()/>
                        <NavigationItem slot label="Modal".to_owned()/>
                        <NavigationItem slot label="Swap".to_owned()/>
                        <NavigationItem slot label="Theme Controller".to_owned()/>
                    </NavigationSection>
                    <NavigationSection slot title="Data Display".to_owned()>
                        <NavigationItem slot label="Accordian".to_owned()/>
                        <NavigationItem slot label="Avatar".to_owned()/>
                        <NavigationItem slot label="Badge".to_owned()/>
                        <NavigationItem slot label="Card".to_owned()/>
                        <NavigationItem slot label="Table".to_owned()/>
                        <NavigationItem slot label="Text".to_owned()/>
                    </NavigationSection>
                    <NavigationSection slot title="Navigation".to_owned()>
                        <NavigationItem slot label="Breadcrums".to_owned()/>
                        <NavigationItem slot label="Bottom navigation".to_owned()/>
                        <NavigationItem slot label="Link".to_owned()/>
                        <NavigationItem slot label="Menu".to_owned()/>
                        <NavigationItem slot label="Navbar".to_owned()/>
                        <NavigationItem slot label="Pagination".to_owned()/>
                        <NavigationItem slot label="Steps".to_owned()/>
                        <NavigationItem slot label="Tab".to_owned()/>
                    </NavigationSection>
                    <NavigationSection slot title="Feedback".to_owned()>
                        <NavigationItem slot label="Alert".to_owned()/>
                        <NavigationItem slot label="Loading".to_owned()/>
                        <NavigationItem slot label="Progress".to_owned()/>
                        <NavigationItem slot label="Radial progress".to_owned()/>
                        <NavigationItem slot label="Skeleton".to_owned()/>
                        <NavigationItem slot label="Toast".to_owned()/>
                        <NavigationItem slot label="Tooltip".to_owned()/>
                    </NavigationSection>
                    <NavigationSection slot title="Data input".to_owned()>
                        <NavigationItem slot label="Checkbox".to_owned()/>
                        <NavigationItem slot label="File Input".to_owned()/>
                        <NavigationItem slot label="Radio".to_owned()/>
                        <NavigationItem slot label="Range".to_owned()/>
                        <NavigationItem slot label="Rating".to_owned()/>
                        <NavigationItem slot label="Select".to_owned()/>
                        <NavigationItem slot label="Text input".to_owned()/>
                        <NavigationItem slot label="Textarea".to_owned()/>
                        <NavigationItem slot label="Toggle".to_owned()/>
                    </NavigationSection>
                </CollapsableNavigationSection>
                <CollapsableNavigationSection slot title="Layouts".to_owned()>
                    <NavigationItem slot label="Divider".to_owned()/>
                    <NavigationItem slot label="Drawer".to_owned()/>
                    <NavigationItem slot label="Footer".to_owned()/>
                    <NavigationItem slot label="Hero".to_owned()/>
                    <NavigationItem slot label="Inidcator".to_owned()/>
                    <NavigationItem slot label="Join".to_owned()/>
                    <NavigationItem slot label="Mask".to_owned()/>
                    <NavigationItem slot label="Stack".to_owned()/>
                </CollapsableNavigationSection>
                <CollapsableNavigationSection slot title="Pages".to_owned()>
                    <NavigationItem slot label="Nav Bar".to_owned()/>
                    <NavigationItem slot label="Nivigation Panel".to_owned()/>
                </CollapsableNavigationSection>
            </SideNavigation>

            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>
                <main>
                    <Routes>
                        <Route path="" view=HomePage />
                        <Route path="/use" view=UsePage />
                        <Route path="/themes" view=ThemePage />
                    </Routes>
                </main>
            </Router>
        </PBApp>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4">
            <article class="prose">
                <h1>Large Leptos component library meant to rapidly prototype applications.</h1>
            </article>
        </div>
    }
}

#[component]
fn UsePage() -> impl IntoView {
    view! {
        Use Page
    }
}

#[component]
fn ThemePage() -> impl IntoView {
    view! {
        Theme Page
    }
}
