let fetchPost = id => {
  let url = "http://127.0.0.1:8080/posts/" ++ id;

  Http.get(~url)
  |> Js.Promise.then_(res => {
       switch (res) {
       | Some(res) =>
         Js.Promise.resolve(
           Some(
             Js.Json.stringify(res)
             |> Json.parseOrRaise
             |> PostModel.Decoder.t,
           ),
         )
       | None => Js.Promise.resolve(None)
       }
     });
};

let fetchPosts = () => {
  let url = "http://127.0.0.1:8080/posts";

  Http.get(~url)
  |> Js.Promise.then_(res => {
       switch (res) {
       | Some(res) =>
         let decodedList =
           switch (Js.Json.decodeArray(res)) {
           | Some(decoedArray) =>
             decoedArray->Belt.Array.keepMap(item =>
               Some(
                 Js.Json.stringify(item)
                 |> Json.parseOrRaise
                 |> PostModel.Decoder.t,
               )
             )
           | None => [||]
           };
         Js.Promise.resolve(decodedList);
       | None => Js.Promise.resolve([||])
       }
     });
};

let createPost = (title, date, content) => {
  let url = "http://127.0.0.1:8080/posts";

  let body = Js.Dict.empty();
  Js.Dict.set(body, "title", Js.Json.string(title));
  Js.Dict.set(body, "date", Js.Json.string(date));
  Js.Dict.set(body, "content", Js.Json.string(content));

  Http.post(~url, ~body)
  |> Js.Promise.then_(res => {
       switch (res) {
       | Some(res) =>
         Js.Promise.resolve(
           Some(
             Js.Json.stringify(res) |> Json.parseOrRaise |> Json.Decode.int,
           ),
         )
       | None => Js.Promise.resolve(None)
       }
     });
};

let updatePost = (id, title, date, content) =>
  if (title == None && date == None && content == None) {
    Js.Promise.resolve(false);
  } else {
    let url = "http://127.0.0.1:8080/posts/" ++ id;

    let body = Js.Dict.empty();
    switch (title) {
    | Some(title) => Js.Dict.set(body, "title", Js.Json.string(title))
    | None => ()
    };
    switch (date) {
    | Some(date) => Js.Dict.set(body, "date", Js.Json.string(date))
    | None => ()
    };
    switch (content) {
    | Some(content) => Js.Dict.set(body, "content", Js.Json.string(content))
    | None => ()
    };

    Http.patch(~url, ~body)
    |> Js.Promise.then_(res => {
         switch (res) {
         | Some(res) =>
           Js.Promise.resolve(
             Js.Json.stringify(res) |> Json.parseOrRaise |> Json.Decode.bool,
           )
         | None => Js.Promise.resolve(false)
         }
       });
  };
