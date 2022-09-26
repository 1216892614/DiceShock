use yew::prelude::*;

#[function_component(SideMenu)]
pub(crate) fn side_menu() -> Html {
    let is_menu_hide = use_state(|| false);

    let onclick = {
        let is_menu_hide = is_menu_hide.clone();
        Callback::from(move |_| {
            is_menu_hide.set(!*is_menu_hide);
        })
    };

    html!(
        <>
            <div
                {onclick}
                style="
                    height: 40px;
                    width: 40px;
                    min-height: 40px;
                    min-width: 40px;
                    margin: 10px;
                    background-color: rgb(33, 37, 43);
                    border-radius: 5px;
                    Box-shadow: -2px 2px 5px rgba(21, 22, 24, 0.6);
                "
            ></div>
            <div
                style={format!("
                    height: 100%;
                    width: 300px;
                    min-width: 300px;
                    display: {};
                    background-color: rgb(33, 37, 43);
                ", if *is_menu_hide {"none"} else {"flex"})}
            ></div>
        </>
    )
}
