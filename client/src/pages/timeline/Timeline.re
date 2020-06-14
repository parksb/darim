module Styles = {
  open Css;

  let container = style([marginBottom(`px(30))]);
};

[@react.component]
let make = () => {
  <Section style=Styles.container> <TimelineHeader /> <List /> </Section>;
};
