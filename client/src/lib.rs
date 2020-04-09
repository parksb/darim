use seed::{*, prelude::*};
use chrono::NaiveDateTime;

struct Post {
    pub id: u64,
    pub author: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

struct Model {
    pub posts: Vec<Post>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            posts: vec![
                Post { id: 0, author: "park".to_string(), content: "Vestibulum orci massa, iaculis cursus mollis malesuada".to_string(), created_at: NaiveDateTime::from_timestamp(1_586_429_335, 0), updated_at: None },
                Post { id: 1, author: "lee".to_string(), content: "consectetur adipiscing elit".to_string(), created_at: NaiveDateTime::from_timestamp(1_586_429_335, 0), updated_at: None },
                Post { id: 2, author: "kim".to_string(), content: "Lorem ipsum dolor sit amet".to_string(), created_at: NaiveDateTime::from_timestamp(1_586_429_335, 0), updated_at: Some(NaiveDateTime::from_timestamp(1_586_429_335, 0)) },
            ]
        }
    }
}

#[derive(Clone)]
enum Msg { }

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) { }

fn view(model: &Model) -> impl View<Msg> {
    let wrapper_container_style = style!{
        St::MaxWidth => "500px";
        St::Margin => "auto";
    };
    let post_container_style = style!{
        St::Margin => "30px 0 30px 0";
        St::PaddingLeft => "10px";
        St::BorderLeft => "3px #ffce05 solid";
    };
    let post_info_style = style!{
        St::Color => "#c0c0c0";
    };
    let post_content_style = style!{
        St::Margin => "5px 0 0 0";
    };

    div![
        &wrapper_container_style,
        section![
            h1!["Patic"],
        ],
        section![
            model.posts.iter().map(|post| {
                article![
                    &post_container_style,
                    strong![format!("{} ", post.author)],
                    small![
                        &post_info_style,
                        time![format!("{}", post.created_at)],
                        span![if let Some(_) = post.updated_at { " (edited)" } else { "" }],
                    ],
                    p![&post_content_style, format!("{}", post.content)],
                ]
            }),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
