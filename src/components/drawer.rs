use yew::prelude::*;

pub mod content;
pub use content::Content;
pub mod header;
pub use header::Header;

pub struct Drawer {
    // inner: Option<MDCDrawer>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Drawer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <aside class="mdc-drawer" id=&self.props.id>
                { self.props.children.render() }
            </aside>
        }
    }
}
