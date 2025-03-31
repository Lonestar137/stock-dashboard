use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub stock_name: String,
    pub stock_price: f64,
    pub change_percent: f64,
}

pub struct StockCard {
    props: Props, // Store the props instead of ctx
}

impl Component for StockCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        StockCard {
            props: ctx.props().clone(), // Clone the props to store them
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let change_color = if self.props.change_percent > 0.0 {
            "green"
        } else {
            "red"
        };

        html! {
            <div >
                <div>
                    <div class="bg-white-100 shadow size-24">
                        <div>
                            <h2>{ &self.props.stock_name }</h2>
                            <p>{ format!("${:.2}", self.props.stock_price) }</p>
                            <p style={format!("color: {}", change_color)}>
                                { format!("{:.2}%", self.props.change_percent) }
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
