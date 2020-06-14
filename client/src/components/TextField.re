module Styles = {
  open Css;

  let default =
    style([
      padding(`px(5)),
      border(`zero, `solid, `hex("000")),
      borderTopWidth(`px(1)),
      borderBottomWidth(`px(1)),
      fontSize(`px(14)),
      backgroundColor(`hex("fcfcfc")),
    ]);
};

[@react.component]
let make =
  React.memo((~type_, ~placeholder, ~value, ~onBlur=?, ~onChange=?, ~style=?) => {
    let mergedStyles =
      switch (style) {
      | Some(style) => Css.merge([Styles.default, style])
      | None => Styles.default
      };

    switch (onChange) {
    | Some(onChange) =>
      switch (onBlur) {
      | Some(onBlur) =>
        <input
          type_
          placeholder
          value
          onChange
          onBlur
          className=mergedStyles
        />
      | None =>
        <input type_ placeholder value onChange className=mergedStyles />
      }
    | None => <input type_ placeholder value className=mergedStyles />
    };
  });
