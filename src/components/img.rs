use yew::prelude::*;

#[derive(Debug, Properties, Clone, PartialEq)]
pub(crate) struct Prop {
    pub(crate) file_name: &'static str,
    pub(crate) style: Option<String>,
}

#[function_component(SVG)]
pub(crate) fn svg(prop: &Prop) -> Html {
    let style = prop.style.clone().unwrap_or("".to_owned());
    html!(
        <img
                    {style}
                    src={format!("static/icons/{}", prop.file_name)}
                    alt={"img"}
        />
    )
}
