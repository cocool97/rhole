use gloo::timers::callback::Interval;
use yew::html;
use yew::{Callback, Children, Component, Html, Properties};

pub enum UpdatedComponentMsg {
    Tick,
}

#[derive(Properties, PartialEq)]
pub struct UpdatedComponentProps {
    pub tick_interval_ms: u32,
    pub children: Children,
    pub update_callback: Callback<()>,
}

pub struct UpdatedComponent {
    _tick_interval: Interval,
}

impl Component for UpdatedComponent {
    type Message = UpdatedComponentMsg;

    type Properties = UpdatedComponentProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let tick_handle = {
            let link = ctx.link().clone();
            Interval::new(ctx.props().tick_interval_ms, move || {
                link.send_message(UpdatedComponentMsg::Tick);
            })
        };

        // Initial request
        ctx.props().update_callback.emit(());

        Self {
            _tick_interval: tick_handle,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            for ctx.props().children.iter()
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UpdatedComponentMsg::Tick => {
                ctx.props().update_callback.emit(());
                true
            }
        }
    }
}
