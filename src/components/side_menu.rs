use yew::prelude::*;
use super::img::*;

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
                class="hide_button"
            >
                <SVG
                    file_name={"DiceShock_angle.svg"}
                    style={format!("margin: 5px;{}", if *is_menu_hide {""} else {"transform:rotate(180deg);"})}
                />
            </div>
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
