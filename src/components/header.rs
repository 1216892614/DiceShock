use yew::prelude::*;

#[function_component(Header)]
pub(crate) fn header() -> Html {
    html!(
        <>
            <div
                style="
                    height: 40px;
                    min-height: 40px;
                    width: 100%;
                    flex-direction: column;
                    Box-shadow: 0 2px 5px rgba(21, 22, 24, 0.3);
                    background-color: rgb(40, 44, 52);
                "
            ></div>
        </>
    )
}