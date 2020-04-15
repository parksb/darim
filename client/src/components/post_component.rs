use crate::{EditedPost, Msg, Post};
use seed::{prelude::*, *};

pub fn view(id: u64, post: Post, edited_post: EditedPost) -> Node<Msg> {
    let container_style = style! {
        St::Margin => "30px 0 30px 0";
        St::PaddingLeft => "10px";
        St::BorderLeft => "3px #ffce05 solid";
    };
    let info_style = style! {
        St::Color => "#c0c0c0";
    };
    let content_style = style! {
        St::Margin => "5px 0 0 0";
    };
    let text_field_style = style! {
        St::FontSize => "16px";
        St::Border => 0;
        St::Padding => 0;
        St::BorderBottom => "1px #000 solid";
    };
    let author_text_field_style = style! {
        St::FontWeight => "bold";
        St::Margin => "0 0 5px 0";
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
                &author_text_field_style,
                &text_field_style,
                attrs! {At::Value => edited_post.author.unwrap_or(post.author)},
                input_ev(Ev::Input, Msg::EditedPostAuthor),
            ]
        } else {
            strong![post.author]
        },
        small![
            &info_style,
            time![format!(" {}", post.created_at)],
            span![if post.updated_at.is_some() {
                " (edited)"
            } else {
                ""
            }],
        ],
        if is_post_to_be_updated {
            vec![
                textarea![
                    &text_field_style,
                    attrs! {
                        At::Value => edited_post.content.unwrap_or(post.content),
                        At::Rows => 3,
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
