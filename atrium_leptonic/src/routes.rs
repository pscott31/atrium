use leptos_icons::BsIcon;
use leptos_router::*;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum AppRoutes {
    Welcome,
    Doc,
    NotFound,
}

impl AppRoutes {
    pub fn route(self) -> &'static str {
        match self {
            AppRoutes::Welcome => "",
            AppRoutes::Doc => "doc",
            AppRoutes::NotFound => "*", // Leptos requires this to be be named "*"!
        }
    }

    pub fn icon(self) -> Option<BsIcon> {
        match self {
            AppRoutes::Welcome => Some(BsIcon::BsBook),
            AppRoutes::Doc => Some(BsIcon::BsSearch),
            AppRoutes::NotFound => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            AppRoutes::Welcome => "Welcome",
            AppRoutes::Doc => "Documentation",
            AppRoutes::NotFound => "",
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl Display for AppRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.route()) }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for AppRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}", self.route()))
    }
}

