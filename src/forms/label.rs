use yew::prelude::*;

use crate::forms::FormField;

pub struct Label<T>
where
    T: FormField,
{
    props: Props<T>,
}

#[derive(Clone, Properties)]
pub struct Props<T>
where
    T: FormField,
{
    pub text: String,
    pub field: T,
}

impl<T> Component for Label<T>
where
    T: FormField,
{
    type Message = ();
    type Properties = Props<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <label class="label" for=self.props.field.form_id()>{ &self.props.text }</label>
        }
    }
}
