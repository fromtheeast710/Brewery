use dioxus::prelude::*;

#[component]
pub fn AZUp() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-sort-ascending-letters",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M15 10v-5c0 -1.38 .62 -2 2 -2s2 .62 2 2v5m0 -3h-4" }
        path { d: "M19 21h-4l4 -7h-4" }
        path { d: "M4 15l3 3l3 -3" }
        path { d: "M7 6v12" }
    }
  }
}

#[component]
pub fn AZDown() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-sort-descending-letters",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M15 21v-5c0 -1.38 .62 -2 2 -2s2 .62 2 2v5m0 -3h-4" }
        path { d: "M19 10h-4l4 -7h-4" }
        path { d: "M4 15l3 3l3 -3" }
        path { d: "M7 6v12" }
    }
  }
}

#[component]
pub fn NumUp() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-sort-ascending-numbers",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M4 15l3 3l3 -3" }
        path { d: "M7 6v12" }
        path { d: "M17 3a2 2 0 0 1 2 2v3a2 2 0 1 1 -4 0v-3a2 2 0 0 1 2 -2z" }
        path { d: "M17 16m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" }
        path { d: "M19 16v3a2 2 0 0 1 -2 2h-1.5" }
    }
  }
}

#[component]
pub fn NumDown() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-sort-descending-numbers",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M4 15l3 3l3 -3" }
        path { d: "M7 6v12" }
        path { d: "M17 14a2 2 0 0 1 2 2v3a2 2 0 1 1 -4 0v-3a2 2 0 0 1 2 -2z" }
        path { d: "M17 5m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" }
        path { d: "M19 5v3a2 2 0 0 1 -2 2h-1.5" }
    }
  }
}

#[component]
pub fn Star() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-star",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z" }
    }
  }
}

#[component]
pub fn StarFilled() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-filled icon-tabler-star",
        fill: "currentColor",
        height: "24",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M8.243 7.34l-6.38 .925l-.113 .023a1 1 0 0 0 -.44 1.684l4.622 4.499l-1.09 6.355l-.013 .11a1 1 0 0 0 1.464 .944l5.706 -3l5.693 3l.1 .046a1 1 0 0 0 1.352 -1.1l-1.091 -6.355l4.624 -4.5l.078 -.085a1 1 0 0 0 -.633 -1.62l-6.38 -.926l-2.852 -5.78a1 1 0 0 0 -1.794 0l-2.853 5.78z" }
    }
  }
}

#[component]
pub fn Settings() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-settings",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065z" }
        path { d: "M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" }
    }
  }
}

#[component]
pub fn Search() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-search",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" }
        path { d: "M21 21l-6 -6" }
    }
  }
}

#[component]
pub fn Play() -> Element {
  rsx! {
    svg {
        class: "icon icon-tabler icons-tabler-outline icon-tabler-player-play",
        fill: "none",
        height: "24",
        stroke: "currentColor",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        stroke_width: "2",
        view_box: "0 0 24 24",
        width: "24",
        xmlns: "http://www.w3.org/2000/svg",
        path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
        path { d: "M7 4v16l13 -8z" }
    }
  }
}

#[component]
pub fn PlayFilled() -> Element {
  rsx! {
    svg {
      class: "icon icon-tabler icons-tabler-filled icon-tabler-player-play",
      fill: "currentColor",
      height: "24",
      view_box: "0 0 24 24",
      width: "24",
      xmlns: "http://www.w3.org/2000/svg",
      path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
      path { d: "M6 4v16a1 1 0 0 0 1.524 .852l13 -8a1 1 0 0 0 0 -1.704l-13 -8a1 1 0 0 0 -1.524 .852z" }
    }
  }
}

#[component]
pub fn ChartPie() -> Element {
  rsx! {
      svg {
          class: "icon icon-tabler icons-tabler-outline icon-tabler-chart-pie",
          fill: "none",
          height: "24",
          stroke: "currentColor",
          stroke_linecap: "round",
          stroke_linejoin: "round",
          stroke_width: "2",
          view_box: "0 0 24 24",
          width: "24",
          xmlns: "http://www.w3.org/2000/svg",
          path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
          path { d: "M10 3.2a9 9 0 1 0 10.8 10.8a1 1 0 0 0 -1 -1h-6.8a2 2 0 0 1 -2 -2v-7a.9 .9 0 0 0 -1 -.8" }
          path { d: "M15 3.5a9 9 0 0 1 5.5 5.5h-4.5a1 1 0 0 1 -1 -1v-4.5" }
      }
  }
}
