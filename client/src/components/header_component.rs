use seed::{prelude::*, *};

pub fn view<Msg: 'static>() -> Node<Msg> {
    let container_style = style! {
        St::Margin => "30px 0 40px 0";
    };

    header![&container_style, h1!["🐦 Patic"]]
}
