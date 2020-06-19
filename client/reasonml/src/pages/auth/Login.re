module Styles = {
  open Css;

  let container = style([marginBottom(`px(30))]);

  let signUpButton = style([borderLeftWidth(`zero)]);

  let fullWidthTextField = style([flex(`num(1.))]);
};

[@react.component]
let make = (~sessionState) => {
  let (email, setEmail) = React.useState(() => "");
  let (password, setPassword) = React.useState(() => "");
  let (_, setSession) = sessionState;

  let login = _ => {
    let _ =
      Api.Auth.login(email, password)
      |> Js.Promise.then_(res => {
           setSession(_ => Some(res));
           Js.Promise.resolve(Some(res));
         });
    ();
  };

  <Section row=true style=Styles.container>
    <TextField
      type_="email"
      placeholder="Email"
      value=email
      onChange={event => {
        let value = ReactEvent.Form.target(event)##value;
        setEmail(_ => value);
      }}
      style=Styles.fullWidthTextField
    />
    <TextField
      type_="password"
      placeholder="Password"
      value=password
      onChange={event => {
        let value = ReactEvent.Form.target(event)##value;
        setPassword(_ => value);
      }}
      style=Styles.fullWidthTextField
    />
    <Button onClick=login> {React.string("Sign in")} </Button>
    <a href="/join">
      <Button style=Styles.signUpButton>
        {React.string({js|Sign up ↗|js})}
      </Button>
    </a>
  </Section>;
};
