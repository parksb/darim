[@bs.val] external alert: string => unit = "alert";

module Styles = {
  open Css;

  let container = style([marginBottom(`px(30))]);

  let titleTextFielld = style([fontSize(`px(24)), fontWeight(`bold)]);

  let dateField =
    style([
      padding4(
        ~top=`px(15),
        ~right=`px(5),
        ~bottom=`px(10),
        ~left=`px(5),
      ),
      fontFamily(`sansSerif),
      fontSize(`px(16)),
      borderWidth(`zero),
      borderBottom(`px(1), `solid, `hex("000")),
    ]);

  let textArea =
    style([
      maxWidth(`percent(100.)),
      marginTop(`px(30)),
      padding(`px(5)),
      fontFamily(`sansSerif),
      fontSize(`px(16)),
      borderWidth(`zero),
      borderBottom(`px(1), `solid, `hex("000")),
      resize(`none),
      lineHeight(`percent(150.)),
    ]);
};

[@react.component]
let make = (~id=?) => {
  let getFormattedDate = (~date, ~withTime) => {
    let format = withTime ? "YYYY-MM-DDT00:00:00" : "YYYY-MM-DD";
    switch (date) {
    | Some(date) => MomentRe.moment(date) |> MomentRe.Moment.format(format)
    | None => MomentRe.momentNow() |> MomentRe.Moment.format(format)
    };
  };

  let (postId, setPostId) = React.useState(() => None);
  let (title, setTitle) = React.useState(() => "");
  let (date, setDate) =
    React.useState(() => getFormattedDate(~date=None, ~withTime=false));
  let (content, setContent) = React.useState(() => "");
  let (originalPost, setOriginalPost) = React.useState(() => None);

  let updatePost = (id, newTitle, newDate, newContent) => {
    let isValid =
      switch (originalPost) {
      | Some((originalPost: PostModel.t)) =>
        newTitle != None
        && newTitle != Some(originalPost.title)
        || newDate != None
        && newDate != Some(originalPost.date)
        || newContent != None
        && newContent != Some(originalPost.content)
      | None => !(newTitle == None && newDate == None && newContent == None)
      };
    if (isValid) {
      let dateWithTime = getFormattedDate(~date=newDate, ~withTime=true);
      let _ =
        Api.Post.updatePost(id, newTitle, Some(dateWithTime), newContent)
        |> Js.Promise.then_(res => {
             if (!res) {
               alert("Failed to save post");
             };
             Js.Promise.resolve(res);
           });
      ();
    };
  };

  let createPost = () =>
    if (title != "" && date != "" && content != "") {
      let dateWithTime = getFormattedDate(~date=Some(date), ~withTime=true);
      let _ =
        Api.Post.createPost(title, dateWithTime, content)
        |> Js.Promise.then_(res => {
             switch (res) {
             | Some(res) => setPostId(_ => Some(string_of_int(res)))
             | None => alert("Failed to save post")
             };
             Js.Promise.resolve(res);
           });
      ();
    };

  let upsertPost = (newTitle, newDate, newContent) => {
    switch (postId) {
    | Some(postId) =>
      switch (originalPost) {
      | Some(_) => updatePost(postId, newTitle, newDate, newContent)
      | None => updatePost(postId, newTitle, newDate, newContent)
      }
    | None => createPost()
    };
  };

  React.useEffect0(() => {
    switch (id) {
    | Some(id) =>
      setPostId(_ => Some(id));
      let _ =
        Api.Post.fetchPost(id)
        |> Js.Promise.then_(res => {
             switch (res) {
             | Some((post: PostModel.t)) =>
               setOriginalPost(_ => Some(post));
               setTitle(_ => post.title);
               setContent(_ => post.content);
               setDate(_ => post.date);
               Js.Promise.resolve(res);
             | None => Js.Promise.resolve(None)
             }
           });
      ();
    | None => ()
    };

    None;
  });

  <Section style=Styles.container>
    <TextField
      type_="text"
      placeholder="Title"
      value=title
      style=Styles.titleTextFielld
      onBlur={event => {
        let value = ReactEvent.Focus.target(event)##value;
        upsertPost(Some(value), None, None);
      }}
      onChange={event => {
        let value = ReactEvent.Form.target(event)##value;
        setTitle(_ => value);
      }}
    />
    <input
      type_="date"
      value={getFormattedDate(~date=Some(date), ~withTime=false)}
      className=Styles.dateField
      onBlur={event => {
        let value = ReactEvent.Focus.target(event)##value;
        upsertPost(None, Some(value), None);
      }}
      onChange={event => {
        let value = ReactEvent.Form.target(event)##value;
        setDate(_ => value);
      }}
    />
    <textarea
      rows=30
      placeholder="Content"
      value=content
      className=Styles.textArea
      onBlur={event => {
        let value = ReactEvent.Focus.target(event)##value;
        upsertPost(None, None, Some(value));
      }}
      onChange={event => {
        let value = ReactEvent.Form.target(event)##value;
        setContent(_ => value);
      }}
    />
  </Section>;
};
