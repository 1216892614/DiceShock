use yew::prelude::*;
use yew_canvas::*;
use web_sys::WebGl2RenderingContext;

/// The whole rander pipeline
mod rander;

#[function_component(Sketchpad)]
pub(crate) fn sketchpad() -> Html {
    let rander = rander::Rander{};

    html!(
        <div
            style="
                width: 100%;
                height: 100%;
                background-color: rgb(40, 44, 52);
                z-index:-1;
            "
        >
            <Canvas<WebGl2RenderingContext , rander::Rander>
                //Just use style, canvas can suit automatically.
                style="
                    width: 100%;
                    height: 100%;
                "
                rander={Box::new(rander)}
            />
        </div>
    )
}