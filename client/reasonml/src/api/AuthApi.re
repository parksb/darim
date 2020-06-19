let fetchSession = () => {
  let url = "http://127.0.0.1:8080/auth";
  Http.get(~url);
};

let login = (email: string, password: string) => {
  let url = "http://127.0.0.1:8080/auth/login";

  // TODO: Hash it.
  let hashedPassword = password;

  let body = Js.Dict.empty();
  Js.Dict.set(body, "email", Js.Json.string(email));
  Js.Dict.set(body, "password", Js.Json.string(hashedPassword));

  Http.post(~url, ~body);
};
