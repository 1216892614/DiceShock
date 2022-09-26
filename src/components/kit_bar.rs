use yew::prelude::*;

#[function_component(KitBar)]
pub(crate) fn kit_bar() -> Html {
    html!(
        <>
            <div
                style="
                    height: 300px;
                    width: 40px;
                    min-width: 40px;
                    Box-shadow: 2px 2px 5px rgba(21, 22, 24, 0.6);
                    background-color: rgb(33, 37, 43);
                    margin: 10px;
                    border-radius: 5px;
                "
            ></div>
        </>
    )
}