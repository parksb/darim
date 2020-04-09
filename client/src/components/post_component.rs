use seed::{*, prelude::*};
use chrono::NaiveDateTime;

pub fn view<Ms: 'static>(
    author: &str,
    content: &str,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
) -> Node<Ms> {
    let container_style = style!{
        St::Margin => "30px 0 30px 0";
        St::PaddingLeft => "10px";
        St::BorderLeft => "3px #ffce05 solid";
    };
    let info_style = style!{
        St::Color => "#c0c0c0";
    };
    let content_style = style!{
        St::Margin => "5px 0 0 0";
    };

    article![
        &container_style,
        strong![format!("{} ", author)],
        small![
            &info_style,
            time![format!("{}", created_at)],
            span![if let Some(_) = updated_at { " (edited)" } else { "" }],
        ],
        p![&content_style, format!("{}", content)],
    ]
}
