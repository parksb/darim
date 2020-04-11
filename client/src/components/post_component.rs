use seed::{*, prelude::*};
use crate::{Msg, Post, EditedPost};

pub fn view(
    id: u64,
    post: Post,
    edited_post: EditedPost,
) -> Node<Msg> {
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

    let is_post_to_be_updated = if let Some(id) = edited_post.id {
        id == post.id
    } else {
        false
    };

    article![
        &container_style,
        if is_post_to_be_updated {
            input![
                attrs!{At::Value => edited_post.author.unwrap_or(post.author.to_string())},
                input_ev(Ev::Input, Msg::EditedPostAuthor),
            ]
        } else {
            strong![post.author]
        },
        small![
            &info_style,
            time![format!("{}", post.created_at)],
            span![if let Some(_) = post.updated_at { " (edited)" } else { "" }],
        ],
        if is_post_to_be_updated {
            vec![
                textarea![
                    attrs!{
                        At::Value => edited_post.content.unwrap_or(post.content.to_string()),
                        At::Rows => 3
                    },
                    input_ev(Ev::Input, Msg::EditedPostContent),
                ],
                button!["✅", ev(Ev::Click, move |_| Msg::Update)],
                button!["❌", ev(Ev::Click, move |_| Msg::UpdateCanceled)],
            ]
        } else {
            vec![
                p![&content_style, post.content],
                button!["🗑", ev(Ev::Click, move |_| Msg::Delete(id))],
                button!["✏️", ev(Ev::Click, move |_| Msg::SetEditedPost(id))],
            ]
        },
    ]
}
