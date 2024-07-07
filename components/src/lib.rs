use leptos::*;

pub mod layout;

#[component]
pub fn SVGButton() -> impl IntoView {
    view! {
        <button class="btn btn-square btn-ghost">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                class="inline-block w-5 h-5 stroke-current"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 6h16M4 12h16M4 18h16"
                ></path>
            </svg>
        </button>
    }
}

#[component]
pub fn Button(text: &'static str) -> impl IntoView {
    view! { <a class="btn btn-ghost text-xl">{text}</a> }
}

/// TODO: Should add props for has_new_notification and a list of notifications
#[component]
pub fn NotificationBell() -> impl IntoView {
    let has_unread = true;
    view! {
        <DropdownBase button=|| {
            view! {
                <button class="btn btn-ghost btn-circle">
                    <div class:indicator=move || has_unread>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
                            ></path>
                        </svg>
                        <span class="badge badge-xs badge-primary indicator-item"></span>
                    </div>
                </button>
            }
        }>

            <div>Test</div>
        </DropdownBase>
    }
}

/// Base dropdown takes any component as button class
#[component]
fn DropdownBase<F, IV>(button: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let wrapped_children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();
    view! {
        <div class="dropdown dropdown-end">
            {button()}
            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
            >
                {wrapped_children}
            </ul>
        </div>
    }
}

#[component]
pub fn DropdownWithImage(
    alt: &'static str,
    children: Children,
    src: &'static str,
) -> impl IntoView {
    view! {
        <DropdownBase button=|| {
            view! {
                <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
                    <div class="w-10 rounded-full">
                        <img alt=alt src=src/>
                    </div>
                </div>
            }
        }>

            {children()}
        </DropdownBase>
    }
}

fn PBProfileSettings() -> impl IntoView {
    view! {
        <DropdownWithImage
            alt="Navbar Example Profile"
            src="https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg"
        >
            <a class="justify-between">Profile <span class="badge">New</span></a>
            <a>Settings</a>
            <a>Logout</a>
        </DropdownWithImage>
    }
}

#[component]
pub fn PBMenuBar() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
            <div class="flex-none">
                <SVGButton/>
            </div>
            <div class="flex-1">
                <Button text="Project Boneyard Component Library"/>
            </div>
            <div class="flex-none">
                <NotificationBell/>
                <PBProfileSettings/>
            </div>
        </div>
    }
}
