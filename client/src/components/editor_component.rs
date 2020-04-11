use seed::{*, prelude::*};
use crate::{Msg, NewPost};

pub fn view(new_post: NewPost) -> Node<Msg> {
    let container_style = style!{
        St::Margin => "30px 0 30px 0";
    };
    let text_field_style = style!{
        St::FontSize => "16px";
        St::MaxWidth => "100%";
        St::MinWidth => "100%";
        St::Border => "1px #000 solid";
        St::Padding => 0;
    };
    let button_style = style!{
        St::FontSize => "14px";
        St::Width => "100%";
        St::Height => "25px";
    };

    section![
        &container_style,
        input![&text_field_style, attrs!{At::Value => new_post.author.unwrap_or("".to_string()), At::Placeholder => "name..."}, input_ev(Ev::Input, Msg::NewPostAuthor)],
        textarea![&text_field_style, attrs!{At::Value => new_post.content.unwrap_or("".to_string()), At::Rows => 3, At::Placeholder => "content..."}, input_ev(Ev::Input, Msg::NewPostContent)],
        button![&button_style, "submit🚀", ev(Ev::Click, |_| Msg::Create)],
    ]
}
