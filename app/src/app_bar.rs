use leptos::*;

#[component]
pub fn AppBar() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
            <div class="flex-none">
                <button class="bun btn-square btn-ghost">
                    <svg
                        xmlns="https://www.w3.org/2000/svg"
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
            </div>
        </div>
    }
}
