module Styles = {
  open Css;

  let default =
    style([
      border(`px(1), `solid, `hex("000")),
      fontSize(`px(14)),
      backgroundColor(`hex("fff")),
      color(`hex("000")),
      padding2(~v=`px(5), ~h=`px(10)),
      cursor(`pointer),
      hover([backgroundColor(`hex("000")), color(`hex("fff"))]),
    ]);
};

[@react.component]
let make = (~style=?, ~onClick=?, ~children) => {
  let mergedStyles =
    switch (style) {
    | Some(style) => Css.merge([Styles.default, style])
    | None => Styles.default
    };

  switch (onClick) {
  | Some(onClick) =>
    <button onClick className=mergedStyles> children </button>
  | None => <button className=mergedStyles> children </button>
  };
};
