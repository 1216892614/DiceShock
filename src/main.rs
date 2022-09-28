use gloo::console::log;
use yew::prelude::*;

/// parts of pages
mod components;
/// fmt/console/submit error message as a log,
mod error;
/// request wrap
mod request;
/// resources types and methods
mod resources;
/// runtime state
mod state;

#[function_component(App)]
fn app() -> Html {
    let using_tool = Callback::from(|u| {
        log!(format!("{:?}", u));
    });

    html!(
        <>
            <components::header::Header></ components::header::Header>
            <div
                style="
                    height: 100%;
                    width: 100%;
                    flex-direction: column;
                    flex-direction: row;
                    margin: 0;
                "
            >
                <components::sketchpad::Sketchpad />
                <div style="height: 100%; width: 100%; margin: 0 0 0 -100%;">
                    <components::kit_bar::KitBar {using_tool}/>
                    <div style="height: 100%; width: 100%"></div>
                    <components::side_menu::SideMenu />
                </div>
            </div>
        </>
    )
}

fn main() {
    yew::start_app::<App>();
}
