use crate::state::using_tool::UsingTool;

use super::img::*;
use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub(crate) using_tool: Callback<UsingTool>,
}

#[function_component(KitBar)]
pub(crate) fn kit_bar(props: &Props) -> Html {
    let using_tool_state = use_state(|| UsingTool::Select);
    let bar_position_state = use_state(|| 10);
    let is_drug_state = use_state(|| false);

    let drug_onmousedown = {
        let is_drug_state = is_drug_state.clone();
        Callback::from(move |_| is_drug_state.set(true))
    };

    let drug_onmouseup = {
        let is_drug_state = is_drug_state.clone();
        Callback::from(move |_| is_drug_state.set(false))
    };

    let drug_onmousemove = {
        let is_drug_state = is_drug_state.clone();
        let bar_position_state = bar_position_state.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_drug_state {
                e.prevent_default();
                let new_margin = e.page_y() - 55;
                bar_position_state.set(if new_margin >= 10 && new_margin <= 400 {
                    new_margin
                } else {
                    is_drug_state.set(false);
                    *bar_position_state
                });
            }
        })
    };

    let onclick_roll = {
        let using_tool_state = using_tool_state.clone();
        let using_tool = props.using_tool.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            using_tool_state.set(UsingTool::Roll);
            using_tool.emit(UsingTool::Roll);
        })
    };

    let onclick_stack = {
        let using_tool_state = using_tool_state.clone();
        let using_tool = props.using_tool.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            using_tool_state.set(UsingTool::Stack);
            using_tool.emit(UsingTool::Stack);
        })
    };

    let onclick_select = {
        let using_tool_state = using_tool_state.clone();
        let using_tool = props.using_tool.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            using_tool_state.set(UsingTool::Select);
            using_tool.emit(UsingTool::Select);
        })
    };

    let onclick_format_brush = {
        let using_tool_state = using_tool_state.clone();
        let using_tool = props.using_tool.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            using_tool_state.set(UsingTool::FormatBrush);
            using_tool.emit(UsingTool::FormatBrush);
        })
    };

    let onclick_eyedropper = {
        let using_tool_state = using_tool_state.clone();
        let using_tool = props.using_tool.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            using_tool_state.set(UsingTool::Eyedropper);
            using_tool.emit(UsingTool::Eyedropper);
        })
    };

    let onclick_delete = {
        let using_tool_state = using_tool_state.clone();
        let using_tool = props.using_tool.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            using_tool_state.set(UsingTool::Delete);
            using_tool.emit(UsingTool::Delete);
        })
    };

    html!(
        <>
            <div
                onmouseup={drug_onmouseup}
                onmousemove={drug_onmousemove}
                style={format!("
                    height: 280px;
                    width: 40px;
                    display: flex;
                    min-width: 40px;
                    Box-shadow: 2px 2px 5px rgba(21, 22, 24, 0.6);
                    background-color: rgb(33, 37, 43);
                    margin: 10px;
                    margin-top: {}px;
                    flex-direction: column;
                    border-radius: 5px;
                ", *bar_position_state)}
            >
                <div
                    onmousedown={drug_onmousedown}
                    class="tools_on_bar drug"
                >
                    <SVG file_name={"DiceShock_drug.svg"} style="margin-top: 10px; margin-bottom: -7px;"/>
                </div>

                <div
                    title="Roll"
                    onclick={onclick_roll}
                    class="act_back tools_on_bar"
                    style={
                        if let UsingTool::Roll = *using_tool_state {
                            "background-color: rgb(63, 68, 83);"
                        } else {""}
                    }
                >
                        <SVG file_name={"DiceShock_roll.svg"}/>
                </div>

                <div
                    title="Stack"
                    onclick={onclick_stack}
                    class="act_back tools_on_bar"
                    style={
                        if let UsingTool::Stack = *using_tool_state {
                            "background-color: rgb(63, 68, 83);"
                        } else {""}
                    }
                >
                        <SVG file_name={"DiceShock_stack.svg"}/>
                </div>

                <div
                    title="Select"
                    onclick={onclick_select}
                    class="act_back tools_on_bar"
                    style={
                        if let UsingTool::Select = *using_tool_state {
                            "background-color: rgb(63, 68, 83);"
                        } else {""}
                    }
                >
                        <SVG file_name={"DiceShock_select.svg"}/>
                </div>

                <div
                    title="FormatBrush"
                    onclick={onclick_format_brush}
                    class="act_back tools_on_bar"
                    style={
                        if let UsingTool::FormatBrush = *using_tool_state {
                            "background-color: rgb(63, 68, 83);"
                        } else {""}
                    }
                >
                        <SVG file_name={"DiceShock_format_brush.svg"}/>
                </div>

                <div
                    title="Eyedropper"
                    onclick={onclick_eyedropper}
                    class="act_back tools_on_bar"
                    style={
                        if let UsingTool::Eyedropper = *using_tool_state {
                            "background-color: rgb(63, 68, 83);"
                        } else {""}
                    }
                >
                        <SVG file_name={"DiceShock_eyedropper.svg"}/>
                </div>

                <div
                    title="Delete"
                    onclick={onclick_delete}
                    class="act_back tools_on_bar"
                    style={
                        if let UsingTool::Delete = *using_tool_state {
                            "background-color: rgb(63, 68, 83);"
                        } else {""}
                    }
                >
                        <SVG file_name={"DiceShock_delete.svg"}/>
                </div>

            </div>
        </>
    )
}
