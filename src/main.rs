mod components;
mod pages;
use crate::components::stock_card::StockCard;
use crate::pages::dashboard::{stock_switch, StockRoute};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/stock/:id")]
    Stock { id: String },
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/dashboard")]
    Dashboard,
}

// Home page
#[function_component(Home)]
fn home() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to the Home Page" }</h1>
            <p><Link<Route> to={Route::About}>{ "Go to About" }</Link<Route>></p>
            <p><Link<Route> to={Route::Stock { id: "AAPL".to_string() }}>{ "View AAPL Stock" }</Link<Route>></p>
        </div>
    }
}

// Stock page (with dynamic parameter)
#[derive(Properties, PartialEq, Clone)]
struct StockProps {
    id: String,
}

#[function_component(Stock)]
fn stock(props: &StockProps) -> Html {
    html! {
        <div>
            <h1>{ format!("Stock: {}", props.id) }</h1>
            <p><Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>></p>
        </div>
    }
}

// About page
#[function_component(About)]
fn about() -> Html {
    html! {
        <div>
            <h1>{ "About Us" }</h1>
            <p><Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>></p>
        </div>
    }
}

// 404 page
#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <div>
            <h1>{ "404 - Page Not Found" }</h1>
            <p><Link<Route> to={Route::Home}>{ "Go Home" }</Link<Route>></p>
        </div>
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {}
}

// Main app component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

// Switch function to map routes to components
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },

        // Example of breaking out a route to a page
        Route::Stock { id } => html! { <Switch <StockRoute> render={stock_switch}/> },

        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Dashboard => html! { <Dashboard /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// struct App;

// impl Component for App {
//     type Message = ();
//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {
//         App
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         true
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <h1>{ "Stock Analysis Tool" }</h1>
//                 <p>{ "Track your favorite stocks in real-time!" }</p>
//                 <div>
//                     <h1>{ "Stock Analysis Tool" }</h1>
//                     <StockCard stock_name="AAPL" stock_price=150.23 change_percent=1.23 />
//                 </div>
//             </div>
//         }
//     }
// }
// fn main() {
//     yew::Renderer::<App>::new().render();
// }
