use seed::{*, prelude::*};
use chrono::NaiveDateTime;
use serde::Deserialize;

mod components {
    pub mod post_component;
    pub mod editor_component;
}

use crate::components::*;

#[derive(Clone, Deserialize)]
pub struct Post {
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

#[derive(Clone, Deserialize)]
pub struct PostsResponse {
    pub data: Vec<Post>,
}

struct Model {
    pub posts: Vec<Post>,
    pub new_post: NewPost,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            posts: vec![],
            new_post: NewPost { author: None, content: None },
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Create,
    NewPostAuthor(String),
    NewPostContent(String),
    PostsFetched(fetch::ResponseDataResult<PostsResponse>),
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(
        fetch::Request::new("http://localhost:8080/posts")
            .fetch_json_data(Msg::PostsFetched)
    );
    AfterMount::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
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

        Msg::PostsFetched(Ok(posts)) => model.posts = posts.data,
        Msg::PostsFetched(Err(_)) => {
            orders.skip();
        },
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
        .after_mount(after_mount)
        .build_and_start();
}
