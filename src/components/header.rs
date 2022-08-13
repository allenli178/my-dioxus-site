use crate::hooks::mode::{is_dark, mode};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{fa_brands_icons, fa_solid_icons},
    Icon,
};

pub fn Header(cx: Scope) -> Element {
    log::info!("dark mode: {:?}", is_dark(&cx));
    let mode_icon = if is_dark(&cx) {
        fa_solid_icons::FaSun
    } else {
        fa_solid_icons::FaMoon
    };
    cx.render(rsx! {
        header{
            class:"bg-white dark:bg-gray-600",
            div{
                class:"max-w-screen-xl px-4 mx-auto sm:px-6 lg:px-8",
                div{
                    class:"flex items-center justify-between h-16",
                    div{
                        class:"flex-1 md:flex md:items-center md:gap-12",
                        Link{
                            class:"block text-teal-600",
                            to: "/",
                            img{
                                alt:"home-icon",
                                src:"public/assets/dioxus.png",
                            }
                        }
                    }
                    div{
                        class:"md:flex md:items-center md:gap-12",
                        nav{
                            class:"hidden md:block",
                            h2{ class:"sr-only",id:"header-navigation","Header navigation"}

                            ul{
                                class:"flex items-center gap-6 text-sm",
                                
                                li{
                                    Link{
                                        class:"text-black dark:text-white transition hover:text-gray-500/75",
                                        to:"/",
                                        "Home"
                                    }
                                    
                                }
                                li{
                                    Link{
                                        class:"text-black dark:text-white transition hover:text-gray-500/75",
                                        to:"/about",
                                        "About"
                                    },
                                }
                            }
                        }

                        div{
                            class:"flex items-center gap-4",
                            div{
                                class:"sm:gap-4 sm:flex",
                                a {
                                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                    href: "javascript:;",
                                    onclick: move |_| {
                                        let is_dark = mode_icon == fa_solid_icons::FaMoon;
                                        mode(&cx, is_dark);
                                        cx.needs_update();
                                    },
                                    Icon {
                                        size: 26,
                                        icon: mode_icon
                                    }
                                }
                                a {
                                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                    href: "https://github.com/mrxiaozhuox/dioxus-starter",
                                    Icon {
                                        size: 26,
                                        icon: fa_brands_icons::FaGithub
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        div{
            class:"bg-white border"
        }
    })
}
