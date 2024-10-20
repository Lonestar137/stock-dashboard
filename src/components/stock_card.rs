use yew::prelude::*;

pub struct StockCard {
    ctx: &Context<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub stock_name: String,
    pub stock_price: f64,
    pub change_percent: f64,
}

impl Component for StockCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        StockCard { ctx }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change_color = if self.ctx.change_percent > 0.0 {
            "green"
        } else {
            "red"
        };
        html! {
            <div>
                <h2>{ &self.ctx.stock_name }</h2>
                <p>{ format!("${:.2}", self.ctx.stock_price) }</p>
                <p style={format!("color: {}", change_color)}>
                    { format!("{:.2}%", self.ctx.change_percent) }
                </p>
            </div>
        }
    }
}
