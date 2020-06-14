module Styles = {
  open Css;

  let container = style([alignItems(`center), marginBottom(`px(40))]);

  let time = style([fontSize(`px(12)), alignSelf(`center)]);

  let horizontalLine =
    style([
      width(`px(30)),
      height(`px(1)),
      borderWidth(`zero),
      margin2(~v=`px(0), ~h=`px(20)),
      alignSelf(`center),
      backgroundColor(`hex("000")),
    ]);

  let title = style([margin(`zero)]);

  let link =
    style([
      display(`flex),
      textDecoration(`none),
      color(`hex("000")),
      alignItems(`center),
    ]);
};

[@react.component]
let make = (~post: PostModel.t) => {
  let displayedDate =
    MomentRe.moment(post.date) |> MomentRe.Moment.format("YYYY / MM / DD");

  <Section row=true style=Styles.container>
    <a href={"/post/" ++ string_of_int(post.id)} className=Styles.link>
      <time dateTime={post.date}> {React.string(displayedDate)} </time>
      <hr className=Styles.horizontalLine />
      <h3 className=Styles.title> {React.string(post.title)} </h3>
    </a>
  </Section>;
};
