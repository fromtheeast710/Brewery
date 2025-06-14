use dioxus::prelude::*;
use std::path::Path;

use crate::ui::icon::{Play, PlayFilled, Star};

pub struct Cover {

}

pub struct Game {
  title: String,
  playtime: f32,
  icon: Box<Path>,
  base: Box<Path>,
}

#[component]
pub fn Deck() -> Element {
  rsx! {
    div { id: "deck",
      for _ in 0..200 {
        Card { }
      }
    }
  }
}

#[component]
pub fn Card() -> Element {
  rsx! {
    div { id: "card",
      img { src: asset!("/assets/terraria.png"), width: 100 }

      Action {  }
    }
  }
}

#[component]
pub fn Action() -> Element {
  let mut is_starred = false;
  let mut is_playing = false;

  rsx! {
    div { id: "action",
      button {
        onclick: move |_| is_playing ^= true,
        if is_playing {
          PlayFilled {  }
        } else {
          Play {  }
        }
      }
      button {
        onclick: move |_| is_starred ^= true,
        Star {  }
      }
    }
  }
}
