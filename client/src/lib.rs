use seed::{*, prelude::*};
use chrono::NaiveDateTime;

mod components {
    pub mod post_component;
    pub mod editor_component;
}

use crate::components::*;

struct Post {
    pub id: u64,
    pub author: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

struct NewPost {
    pub author: Option<String>,
    pub content: Option<String>,
}

struct Model {
    pub posts: Vec<Post>,
    pub new_post: NewPost,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            posts: vec![
                Post { id: 0, author: "park".to_string(), content: "Vestibulum orci massa, iaculis cursus mollis malesuada".to_string(), created_at: NaiveDateTime::from_timestamp(1_586_429_335, 0), updated_at: None },
                Post { id: 1, author: "lee".to_string(), content: "consectetur adipiscing elit".to_string(), created_at: NaiveDateTime::from_timestamp(1_586_429_335, 0), updated_at: None },
                Post { id: 2, author: "kim".to_string(), content: "Lorem ipsum dolor sit amet".to_string(), created_at: NaiveDateTime::from_timestamp(1_586_429_335, 0), updated_at: Some(NaiveDateTime::from_timestamp(1_586_429_335, 0)) },
            ],
            new_post: NewPost { author: None, content: None },
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Create,
    NewPostAuthor(String),
    NewPostContent(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Create => {
            if let Some(author) = model.new_post.author.clone() {
                if let Some(content) = model.new_post.content.clone() {
                    if !author.is_empty() && !content.is_empty() {
                        // create a new post
                    }
                }
            }
        },
        Msg::NewPostAuthor(author) => model.new_post.author = Some(author),
        Msg::NewPostContent(content) => model.new_post.content = Some(content),
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let wrapper_container_style = style!{
        St::MaxWidth => "500px";
        St::Margin => "auto";
    };

    div![
        &wrapper_container_style,
        section![h1!["Patic"]],
        section![editor_component::view()],
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
