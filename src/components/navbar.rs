use yew::prelude::*;

// #[derive(Clone, PartialEq, Properties)]
// pub struct NavbarProps {}

// pub struct NavbarComponent {
//     props: NavbarProps,
// }

// impl Component for NavbarComponent {
//     type Message = ();
//     type Properties = NavbarProps;

//     fn create(ctx: &Context<Self>) -> Self {
//         NavbarComponent {
//             props: ctx.props().clone(),
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <nav class="flex shadow-xl">
//                 <span class="font-medium text-2xl pr-4">{"Item"}</span>
//                 <span class="font-medium text-2xl pr-4">{"Item2"}</span>
//             </nav>
//         }
//     }
// }

use yew_router::prelude::*; // Import yew_router

// Assuming Route is defined in main.rs and needs to be imported
use crate::Route; // Import Route from the root (main.rs)

#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {}

#[function_component(NavbarComponent)] // Convert to function component for simplicity
pub fn navbar_component(_props: &NavbarProps) -> Html {
    let navigator = use_navigator().unwrap(); // Get the navigator for routing

    // Callbacks for navigation
    let goto_item1 = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };
    let goto_item2 = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Dashboard))
    };

    html! {
        <nav class="flex shadow-xl">
            <span
                class="font-medium text-2xl pr-4 cursor-pointer"
                onclick={goto_item1}
            >
                {"Home"}
            </span>
            <span
                class="font-medium text-2xl pr-4 cursor-pointer"
                onclick={goto_item2}
            >
                {"Dashboard"}
            </span>
        </nav>
    }
}
