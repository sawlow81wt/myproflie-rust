use crate::{Msg, Model, Urls};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "navbar",
            "shadow-lg",
            "bg-base-200",
        ],
        div![
            C![
                "px-2",
                "mx-2",
                "navbar-start"
            ],
            span![
                C![
                    "text-lg",
                    "font-bold",
                ],
                "SotaroProfile"
            ]
        ],
        div![
            C![
                "flex-none",
                "hidden",
                "px-2",
                "mx-2",
                "navbar-end",
                "md:flex",
            ],
            div![
                C![
                    "flex",
                    "items-stretch"
                ],
                a![
                    C![
                        "btn",
                        "btn-ghost",
                        "btn-sm",
                        "rounded-btn"
                    ],
                    "Home",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).home()
                    },
                ],
                a![
                    C![
                        "btn",
                        "btn-ghost",
                        "btn-sm",
                        "rounded-btn"
                    ],
                    "About",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).about()
                    },
                ],
                a![
                    C![
                        "btn",
                        "btn-ghost",
                        "btn-sm",
                        "rounded-btn"
                    ],
                    "MNIST",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).mnist()
                    },
                ],
                a![
                    C![
                        "btn",
                        "btn-ghost",
                        "btn-sm",
                        "rounded-btn"
                    ],
                    attrs! {
                        At::Href => "https://twitter.com/saw_poke",
                    },
                    svg![
                        C!["fill-current"],
                        attrs! {
                            At::Width => 24,
                            At::Height => 24,
                            At::ViewBox => "0 0 24 24",
                        },
                        path![
                            attrs! {
                                At::D => "M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"
                            }
                        ]
                    ]
                ],
            ]
        ],
        div![
            C![
                "flex",
                "px-2",
                "mx-2",
                "navbar-end",
                "md:flex-none",
                "md:hidden"
            ],
            button![
                C![
                    "btn",
                    "rounded-btn"
                    "btn-sm",
                    "btn-ghost",
                ],
                svg![
                    C![
                        "inline-block",
                        "stroke-current",
                    ],
                    attrs! {
                        At::Width => 24,
                        At::Height => 24,
                        At::ViewBox => "0 0 24 24",
                        At::Fill => "none",
                        At::StrokeWidth => 2,
                        At::StrokeLinecap => "round",
                        At::StrokeLineJoin => "round"
                    },
                    path![
                        attrs! {
                            At::D => "M4 6h16M4 12h16M4 18h16"
                        }
                    ],
                ],
                ev(Ev::Click, |_| Msg::TranslateSlideBar),
            ],
        ],
    ]
}
