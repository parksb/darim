module Styles = {
  open Css;

  let container = style([marginBottom(`px(40))]);

  let link = style([textDecoration(`none), color(rgb(0, 0, 0))]);

  let title = style([display(`inline)]);
};

[@react.component]
let make =
  React.memo(() => {
    <header className=Styles.container>
      <a href="/" className=Styles.link>
        <h1 className=Styles.title> {React.string({js|🏕 Darim|js})} </h1>
      </a>
    </header>
  });
