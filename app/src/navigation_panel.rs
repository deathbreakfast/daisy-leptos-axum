use leptos::*;
use leptos_meta::*;
use crate::icons::*;

/// TODO: 
///     * Add optional Left Image
#[slot]
pub struct CollapsableNavigationSection {
    title: String,
    #[prop(default=vec![])]
    navigation_section: Vec<NavigationSection>,
    #[prop(default=vec![])]
    navigation_item: Vec<NavigationItem>,
    #[prop(default=None)]
    icon: Option<SVGIcons>,
}

#[slot]
pub struct NavigationSection {
    #[prop(default=None)]
    icon: Option<SVGIcons>,
    navigation_item: Vec<NavigationItem>,
    title: String,
}

#[slot]
pub struct NavigationItem {
    label: String,
    #[prop(default=None)]
    path: Option<String>,
}

/// TODO: 
///     * Make the navigation panel scroll
///     * Options to show / hide
#[component]
pub fn NavigationPanel(
    #[prop(default=vec![])] collapsable_navigation_section: Vec<CollapsableNavigationSection>,
    #[prop(default=vec![])] navigation_section: Vec<NavigationSection>,
    #[prop(default=vec![])] navigation_item: Vec<NavigationItem>,
) -> impl IntoView {
    let nav_items = navigation_item
        .into_iter()
        .map(|item| view! {
            <NavLi label=item.label path=item.path />
        })
        .collect_view();
    let navigation_sections = navigation_section.into_iter()
        .map(|section| view! {
            <NavSection 
                icon=section.icon
                section_title=section.title 
                nav_items=section.navigation_item 
            />
        })
        .collect_view();
    let collapsable_nav_sections = collapsable_navigation_section.into_iter()
        .map(|section| view! { 
            <CollapseableNavSection 
                nav_items=section.navigation_item
                navigation_section=section.navigation_section
                section_title=section.title
                icon=section.icon
            /> 
        })
        .collect_view();

    view! {
        <div class="drawer-side">
            <label for="my-drawer-3" aria-label="close sidebar" class="drawer-overlay"></label>
            <ul class="menu p-4 w-80 min-h-full bg-base-200">
                {collapsable_nav_sections}
                {navigation_sections}
                {nav_items}
            </ul>
        </div>
    }
}

#[component]
fn NavSection(
    #[prop(default=vec![])]
    nav_items: Vec<NavigationItem>,
    section_title: String,
    #[prop(default=None)]
    icon: Option<SVGIcons>,
) -> impl IntoView {
    let icon = match icon {
        None => None,
        Some(value) => Some(view! {
            <div style="padding-right:4px">
                {value.into_view()}
            </div>
        }),
    };
    view! {
        <div style="display:flex; align-items:center;">
            {icon}
            <NavLi label=section_title />
        </div>
        <li>
            <ul>
                {nav_items
                    .into_iter()
                    .map(|nav| {
                        view! {
                            <NavLi label=nav.label path=nav.path />
                        }
                    })
                    .collect_view()}
            </ul>
        </li>
    }
}

#[component]
fn CollapseableNavSection(
    #[prop(default=vec![])]
    nav_items: Vec<NavigationItem>,
    #[prop(default=vec![])]
    navigation_section: Vec<NavigationSection>,
    section_title: String,
    #[prop(default=None)]
    icon: Option<SVGIcons>,
) -> impl IntoView {
    let navigation_items = nav_items
        .into_iter()
        .map(|item| view! {
            <NavLi label=item.label path=item.path />
        })
        .collect_view();
    let nav_sections = navigation_section.into_iter()
        .map(|section| view! {
            <NavSection 
                icon=section.icon
                section_title=section.title nav_items=section.navigation_item />
        })
        .collect_view();
    let icon = match icon {
        None => None,
        Some(value) => Some(view! {
            <div style="padding-right:4px">
                {value.into_view()}
            </div>
        }),
    };
    view! {
        <div tabindex="0" class="collapse collapse-arrow">
            <input type="checkbox" class="peer"/>
            <div class="collapse-title text-xl font-medium">
                <div style="display:flex; align-items:center;">
                    {icon}
                    {section_title}
                </div>
            </div>
            <div class="collapse-content">
                <li>
                    <ul>
                        {navigation_items}
                        {nav_sections}
                    </ul>
                </li>
            </div>
        </div>
    }
}

#[component]
fn NavLi(
    label: String,
    #[prop(default=None)]
    path: Option<String>,
) -> impl IntoView {
    view! {
        <li>
            <a href=path>{label}</a>
        </li>
    }
}
