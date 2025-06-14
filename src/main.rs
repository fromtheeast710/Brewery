use dioxus::prelude::*;
use ui::{deck::Deck, nav::Nav};

mod ui {
  pub mod deck;
  pub mod nav;
  pub mod icon;
}

const CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
  rsx! {
    document::Link { rel: "stylesheet", href: CSS }

    Nav { }
    Deck { }
  }
}

fn main() {
  dioxus::launch(App);
}
