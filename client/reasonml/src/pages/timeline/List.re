module Styles = {
  open Css;

  let container = style([marginBottom(`px(40))]);
};

[@react.component]
let make = () => {
  let (posts, setPosts) = React.useState(() => [||]);

  React.useEffect0(() => {
    let _ =
      Api.Post.fetchPosts()
      |> Js.Promise.then_(res => {
           setPosts(_ => res);
           Js.Promise.resolve(res);
         });
    None;
  });

  <Section style=Styles.container>
    {posts
     ->Belt.Array.map(post => <Item key={string_of_int(post.id)} post />)
     ->React.array}
  </Section>;
};
