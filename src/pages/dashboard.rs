use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum StockRoute {
    #[at("/stock/:id")]
    Overview { id: String },
    #[at("/stock/:id/details")]
    Details { id: String },
}

pub fn stock_switch(routes: StockRoute) -> Html {
    match routes {
        StockRoute::Overview { id } => html! { <div>{ format!("Overview for {}", id) }</div> },
        StockRoute::Details { id } => html! { <div>{ format!("Details for {}", id) }</div> },
    }
}
