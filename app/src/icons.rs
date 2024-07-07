use leptos::*;

pub enum SVGIcons {
    Book(u32, bool),
    Click(u32, bool),
    Shapes(u32, bool),
}

impl SVGIcons {
    pub fn into_view(&self) -> impl IntoView {
        match *self {
            SVGIcons::Book(size, fill) => {
                view! {
                    <BookIcon size={size} fill={fill} />
                }
            }
            SVGIcons::Click(size, fill) => {
                view! {
                    <ClickIcon size={size} fill={fill} />
                }
            }
            SVGIcons::Shapes(size, fill) => {
                view! {
                    <ShapesIcon size={size} fill={fill} />
                }
            }
        }
    }
}

#[component]
pub fn ClickIcon(
    size: u32,
    fill: bool,
) -> impl IntoView {
    view! {
        <SVGIcon
            size={size}
            fill={fill}
            paths={vec![
                "M24 4V12".to_string(),
                "M22 22L42 26L36 30L42 36L36 42L30 36L26 42L22 22Z".to_string(),
                "M38.1421 9.85789L32.4853 15.5147".to_string(),
                "M9.85787 38.1421L15.5147 32.4853".to_string(),
                "M9.85795 9.85787L15.5148 15.5147".to_string(),
            ]}
        />
    }
}

#[component]
pub fn ShapesIcon(
    size: u32,
    fill: bool,
) -> impl IntoView {
    view! {
        <SVGIcon
            size={size}
            fill={fill}
            paths={vec![
                "M20 29H6V43H20V29Z".to_string(),
                "M24 4L34 21H14L24 4Z".to_string(),
                "M36 44C40.4183 44 44 40.4183 44 36C44 31.5817 40.4183 28 36 28C31.5817 28 28 31.5817 28 36C28 40.4183 31.5817 44 36 44Z".to_string(),
            ]}
        />
    }
}

#[component]
pub fn BookIcon(
    size: u32,
    fill: bool,
) -> impl IntoView {
    view! {
        <SVGIcon
            size={size}
            fill={fill}
            paths={vec![
                "M5 7H16C20.4183 7 24 10.5817 24 15V42C24 38.6863 21.3137 36 18 36H5V7Z".to_string(),
                "M43 7H32C27.5817 7 24 10.5817 24 15V42C24 38.6863 26.6863 36 30 36H43V7Z".to_string(),
            ]}
        />
    }
}

#[component]
fn SVGIcon(
    size: u32,
    fill: bool,
    paths: Vec<String>,
) -> impl IntoView {
    let fill_value: String = match fill {
        true => "black".to_string(),
        false => "none".to_string(),
    };

    view! {
        <svg 
            width={size}
            height={size} 
            viewBox="0 0 48 48" 
            fill={fill_value.clone()}>
            {paths.into_iter().map(|path| view! {
                <path 
                    d={path}
                    fill={fill_value.clone()}
                    stroke="currentColor"
                    stroke-width="4"
                    stroke-linejoin="bevel" />
            }).collect_view()}
        </svg>
    }
}

