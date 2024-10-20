mod components;
use crate::components::stock_card::StockCard;

use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Stock Analysis Tool" }</h1>
                <p>{ "Track your favorite stocks in real-time!" }</p>
                <div>
                    <h1>{ "Stock Analysis Tool" }</h1>
                    <StockCard stock_name="AAPL" stock_price=150.23 change_percent=1.23 />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
