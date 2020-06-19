let request = (~method_: Fetch.requestMethod, ~url: string, ~body) => {
  let headers =
    Fetch.HeadersInit.make({
      "Content-Type": "application/json",
      "Access-Control-Allow-Credentials": "true",
    });

  let credentials = Fetch.Include;
  let options =
    switch (body) {
    | Some(body) =>
      let body =
        Fetch.BodyInit.make(body |> Js.Json.object_ |> Js.Json.stringify);
      Fetch.RequestInit.make(~method_, ~headers, ~body, ~credentials, ());
    | None => Fetch.RequestInit.make(~method_, ~headers, ~credentials, ())
    };

  Js.Promise.(
    Fetch.fetchWithInit(url, options)
    |> then_(Fetch.Response.json)
    |> then_(res => {
         switch (Js.Json.decodeObject(res)) {
         | Some(decodedRes) => resolve(Js.Dict.get(decodedRes, "data"))
         | None => resolve(None)
         }
       })
  );
};

let get = (~url: string) => {
  request(~method_=Get, ~url, ~body=None);
};

let post = (~url: string, ~body: Js.Dict.t(Js.Json.t)) => {
  request(~method_=Post, ~url, ~body=Some(body));
};

let patch = (~url: string, ~body: Js.Dict.t(Js.Json.t)) => {
  request(~method_=Patch, ~url, ~body=Some(body));
};
