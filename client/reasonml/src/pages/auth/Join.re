module Styles = {
  open Css;

  let container = style([marginBottom(`px(30))]);

  let fullWidthTextField = style([flex(`num(1.))]);

  let nonBorderFuWidthTextField =
    style([flex(`num(1.)), borderTopWidth(`zero)]);

  let nonBorderButton = style([borderTopWidth(`zero)]);
};

[@react.component]
let make = () => {
  let (email, setEmail) = React.useState(() => "");
  let (password, setPassword) = React.useState(() => "");
  let (name, setName) = React.useState(() => "");
  let (avatarUrl, setAvatarUrl) = React.useState(() => "");

  <Section style=Styles.container>
    <Section row=true>
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
      <TextField
        type_="name"
        placeholder="Name"
        value=name
        onChange={event => {
          let value = ReactEvent.Form.target(event)##value;
          setName(_ => value);
        }}
        style=Styles.fullWidthTextField
      />
    </Section>
    <Section row=true>
      <TextField
        type_="url"
        placeholder="Avatar URL"
        value=avatarUrl
        onChange={event => {
          let value = ReactEvent.Form.target(event)##value;
          setAvatarUrl(_ => value);
        }}
        style=Styles.nonBorderFuWidthTextField
      />
      <a href="/join">
        <Button style=Styles.nonBorderButton>
          {React.string({js|Create account|js})}
        </Button>
      </a>
    </Section>
  </Section>;
};
