use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("app.css");

use crate::components::atoms::main_title::MainTitle;

#[derive(Serialize, Deserialize)]
struct Obj {
    a: String,
    b: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let vals: Obj = Obj {
        a: "awe".to_owned(),
        b: "123".to_owned(),
    };
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    log!(serde_json::to_string_pretty(&vals).unwrap());
    let messages: Vec<&str> = vec!["some message", "another", "and one more"];

    html! {
        <html class={stylesheet}>
            <MainTitle title="A title!" />
            if messages.len() > 0 {
                <ul>
                {list_to_html(messages)}
                </ul>
            } else {
                <p class={css!("color: red; font-size: 75px;")}>{"No messages"}</p>
                // under the hood this makes a new style in the head
                // as does all other css declarations from stylist
            }
        </html>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect() // No semi means implicit return
}
