use dioxus::prelude::*;

use crate::ui::icon::PlayFilled;

#[component]
pub fn Nav() -> Element {
  rsx! {
    header { id: "nav",
      "Brewery"
      PlayFilled {  },
    }
  }
}
