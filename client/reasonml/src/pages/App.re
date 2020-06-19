module Styles = {
  open Css;

  let container =
    style([
      maxWidth(`px(800)),
      padding4(~top=`px(30), ~right=`zero, ~bottom=`px(30), ~left=`zero),
      margin(`auto),
      wordBreak(`keepAll),
      fontFamily(`sansSerif),
    ]);
};

[@react.component]
let make = () => {
  let (session, setSession) = React.useState(_ => None);

  React.useEffect0(() => {
    let _ =
      Api.Auth.fetchSession()
      |> Js.Promise.then_(res => {
           setSession(_ => Some(res));
           Js.Promise.resolve(res);
         });
    None;
  });

  let url = ReasonReactRouter.useUrl();
  let page =
    switch (url.path, session) {
    | ([], None) => <Login sessionState=(session, setSession) />
    | ([], Some(_)) => <Timeline />
    | (["join"], None) => <Join />
    | (["post", id], Some(_)) => <Post id />
    | (["post"], Some(_)) => <Post />
    | _ => <h1> {React.string("404 Not Found")} </h1>
    };

  <div className=Styles.container> <Header /> page </div>;
};
