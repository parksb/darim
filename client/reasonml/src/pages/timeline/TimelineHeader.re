module Styles = {
  open Css;

  let container = style([marginBottom(`px(40))]);
};

[@react.component]
let make = () => {
  <Section row=true style=Styles.container>
    <a href="/post"> <Button> {React.string({js|New +|js})} </Button> </a>
  </Section>;
};
