use yew::prelude::*;

pub struct Modal {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub show: bool,
    pub children: Children,
    pub on_close: Callback<()>,
}

impl Component for Modal {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Modal { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.show {
            html! {
                <div class="modal">
                    <div class="modal-content">
                        { for self.props.children.iter() }
                    </div>
                    <button onclick=self.props.on_close.clone()>{ "Close" }</button>
                </div>
            }
        } else {
            html! {}
        }
    }
}
