use seed::{*, prelude::*};
use chrono::NaiveDateTime;

mod components {
    pub mod post_component;
}

use crate::components::post_component;

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

    div![
        &wrapper_container_style,
        section![h1!["Patic"]],
        section![
            model.posts.iter().map(|post| {
                post_component::view(&post.author, &post.content, post.created_at, post.updated_at)
            }),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
