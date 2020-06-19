module Styles = {
  open Css;

  let default = row =>
    style([display(`flex), flexDirection(row ? `row : `column)]);
};

[@react.component]
let make = (~row=false, ~style=?, ~children) => {
  let mergedStyles =
    switch (style) {
    | Some(style) => Css.merge([Styles.default(row), style])
    | None => Styles.default(row)
    };

  <section className=mergedStyles> children </section>;
};
