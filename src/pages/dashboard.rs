use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::stock_card::StockCard;

// /// Stock item on Dashboard scanned stock list.
// #[derive(Clone, PartialEq, Properties)]
// pub struct Stock {
//     /// Stock symbol.
//     symbl: String,
//     ///// Scanners that like this stock.
//     //triggered_scanners: Vec<String>,
// }

/// Represents a News article
#[derive(Clone, PartialEq, Properties)]
pub struct Stock {
    title: String,
    description: String,
    link: String,
    ai_summary: String,
}

/// Represents a News article
#[derive(Clone, PartialEq, Properties)]
pub struct NewsArticle {
    title: String,
    description: String,
    link: String,
    ai_summary: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DashboardProps {
    pub(crate) news_articles: Vec<NewsArticle>,
    pub(crate) scanner_stocks: Vec<Stock>,
}
pub struct DashboardComponent {
    properties: DashboardProps,
}

impl Component for DashboardComponent {
    type Message = ();
    type Properties = DashboardProps;

    fn create(ctx: &Context<Self>) -> Self {
        DashboardComponent {
            properties: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}
