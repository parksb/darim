use seed::{*, prelude::*};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

mod api;
mod components {
    pub mod post_component;
    pub mod footer_component;
    pub mod header_component;
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

#[derive(Clone, Serialize)]
pub struct NewPost {
    pub author: Option<String>,
    pub content: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct EditedPost {
    pub id: Option<u64>,
    pub author: Option<String>,
    pub content: Option<String>,
}

struct Model {
    pub posts: Vec<Post>,
    pub new_post: NewPost,
    pub edited_post: EditedPost,
}

#[derive(Clone)]
pub enum Msg {
    Create,
    Update,
    UpdateCanceled,
    Delete(u64),

    NewPostAuthor(String),
    NewPostContent(String),
    EditedPostAuthor(String),
    EditedPostContent(String),
    SetEditedPost(u64),

    PostsFetched(fetch::ResponseDataResult<api::Response<Vec<Post>>>),
    PostDeleted(fetch::ResponseDataResult<api::Response<bool>>),
    PostCreated(fetch::ResponseDataResult<api::Response<bool>>),
    PostUpdated(fetch::ResponseDataResult<api::Response<bool>>),
}

impl Default for Model {
    fn default() -> Self {
        Self {
            posts: vec![],
            new_post: NewPost { author: None, content: None },
            edited_post: EditedPost { id: None, author: None, content: None },
        }
    }
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(api::get_list());
    AfterMount::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Create => { orders.perform_cmd(api::create(model.new_post.clone())); },
        Msg::Update => { orders.perform_cmd(api::update(model.edited_post.clone())); },
        Msg::UpdateCanceled => { model.edited_post = EditedPost { id: None, author: None, content: None }; },
        Msg::Delete(id) => { orders.perform_cmd(api::delete(id)); },

        Msg::NewPostAuthor(author) => model.new_post.author = Some(author),
        Msg::NewPostContent(content) => model.new_post.content = Some(content),

        Msg::EditedPostAuthor(author) => model.edited_post.author = Some(author),
        Msg::EditedPostContent(content) => model.edited_post.content = Some(content),
        Msg::SetEditedPost(id) => {
            let target_post = model.posts.iter().find(|post| post.id == id);
            if let Some(target) = target_post {
                model.edited_post = EditedPost {
                    id: Some(id),
                    author: Some(target.author.clone()),
                    content: Some(target.content.clone())
                };
            }
        },

        Msg::PostsFetched(Ok(posts)) => model.posts = posts.data,
        Msg::PostsFetched(Err(_)) => { orders.skip(); },

        Msg::PostDeleted(Ok(_)) => { orders.perform_cmd(api::get_list()); }
        Msg::PostDeleted(Err(_)) => { orders.skip(); }

        Msg::PostCreated(Ok(_)) => {
            orders.perform_cmd(api::get_list());
            model.new_post = NewPost { author: None, content: None };
        }
        Msg::PostCreated(Err(_)) => { orders.skip(); }

        Msg::PostUpdated(Ok(_)) => {
            orders.perform_cmd(api::get_list());
            model.edited_post = EditedPost { id: None, author: None, content: None };
        }
        Msg::PostUpdated(Err(_)) => { orders.skip(); }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let wrapper_container_style = style!{
        St::MaxWidth => "500px";
        St::Margin => "auto";
    };

    div![
        &wrapper_container_style,
        header_component::view(),
        editor_component::view(model.new_post.clone()),
        section![
            model.posts.iter().map(|post| {
                post_component::view(
                    post.id,
                    post.clone(),
                    model.edited_post.clone(),
                )
            }),
        ],
        footer_component::view(),
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
