use seed::{prelude::*, *};

pub fn view<Msg: 'static>() -> Node<Msg> {
    let container_style = style! {
        St::Margin => "50px 0 30px 0";
    };
    let text_style = style! {
        St::Color => "#cfcfcf";
    };
    let link_style = style! {
        St::Color => "#bddbf8";
    };

    footer![
        &container_style,
        small![
            &text_style,
            "This project is licensed under the MIT License - see the ",
            a![
                &link_style,
                "LICENSE.md",
                attrs! {At::Href => "https://github.com/ParkSB/patic"},
            ],
            " file for details",
        ],
    ]
}
