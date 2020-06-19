type t = {
  id: int,
  title: string,
  content: string,
  date: string,
  createdAt: string,
  updatedAt: option(string),
};

module Decoder = {
  let t = json =>
    Json.Decode.{
      id: json |> field("id", int),
      title: json |> field("title", string),
      content: json |> field("content", string),
      date: json |> field("date", string),
      createdAt: json |> field("created_at", string),
      updatedAt: json |> optional(field("update_at", string)),
    };
};
