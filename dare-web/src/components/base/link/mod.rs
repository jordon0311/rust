use yew::prelude::*;
use crate::components::{ HTMLTag, get_html_tag };

#[derive(Properties, PartialEq)]
pub struct CLinkProps {
    /**
     * Toggle the active state for the component.
     */
    #[prop_or(false)]
    pub active: bool,
    /**
     * A string of all className you want applied to the component.
     */
    #[prop_or_default]
    pub class_name: AttrValue,

    /**
     * Component used for the root node. Either a string to use a HTML element or a component.
     */
    #[prop_or(HTMLTag::A)]
    pub component: HTMLTag,
    /**
     * Toggle the disabled state for the component.
     */
    #[prop_or(false)]
    pub disabled: bool,
    /**
     * The href attribute specifies the URL of the page the link goes to.
     */
    pub href: Option<String>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component]
pub fn CLink(CLinkProps {
    active,
    class_name,
    component,
    disabled,
    href,
    onclick,
    children,
}: &CLinkProps) -> Html {
    let html_tag = get_html_tag(&component);
    let class = classes!(
        class_name.clone(),
        if *active {
            "active"
        } else {
            ""
        },
        if *disabled {
            "disabled"
        } else {
            ""
        }
    );
    let aria_current = if *active { Some("page") } else { None };
    let aria_disabled = *disabled && html_tag == "a";
    let tab_index = if aria_disabled { Some("-1") } else { None };
    html! {
        <@{html_tag} 
        {class} 
        href={href.clone()}
        aria-current={aria_current}
        aria-disabled={if aria_disabled { Some("true") } else { None }}
        tabindex={tab_index}
        onclick={onclick.clone()}
        disabled={true} 
        >
            {children.clone()}
        </@>
    }
}
