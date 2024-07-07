use leptos::*;
use leptos_meta::*;

use crate::navigation_panel::*;

pub enum Theme {
    Dark,
}

impl Theme {
    pub fn get_data_theme(&self) -> String {
        match self {
            Theme::Dark => "dark".to_owned(), 
        }
    }
}

#[slot]
pub struct SideNavigation {
    #[prop(default=vec![])] 
    collapsable_navigation_section: Vec<CollapsableNavigationSection>,
    #[prop(default=vec![])] 
    navigation_section: Vec<NavigationSection>,
    #[prop(default=vec![])] 
    navigation_item: Vec<NavigationItem>,
}

/// TODO: 
///     * Move Icon into icons file
///     * Cleanup app bar into it's own file
///     * Navigation panel should have sticky option
#[component]
pub fn PBApp(
    children: Children,
    side_navigation: SideNavigation,
    theme: Theme,
) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let nav_section = NavigationPanel(
        NavigationPanelProps {
            collapsable_navigation_section: side_navigation.collapsable_navigation_section,
            navigation_section: side_navigation.navigation_section,
            navigation_item: side_navigation.navigation_item,
        }
    );

    view! {
        <Stylesheet id="leptos" href="/pkg/pb-component-library.css"/>

        // sets the document title
        <Title text="Project Boneyard Components"/>

        // content for this welcome page
        <div data-theme=theme.get_data_theme()>

            <div class="drawer">
                <input id="my-drawer-3" type="checkbox" class="drawer-toggle"/>
                <div class="drawer-content flex flex-col">
                    <div class="w-full navbar bg-base-300">
                        <div class="flex-none">
                            <label
                                for="my-drawer-3"
                                aria-label="open sidebar"
                                class="btn btn-square btn-ghost"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="inline-block w-6 h-6 stroke-current"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M4 6h16M4 12h16M4 18h16"
                                    ></path>
                                </svg>
                            </label>
                        </div>
                        <div class="flex-1 px-2 mx-2">PB Component Library</div>
                    </div>
                    {children()}
                </div>
                {nav_section}
            </div>
        </div>
    }
}
