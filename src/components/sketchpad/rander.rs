#[derive(Clone, PartialEq)]
pub(super) struct Rander;

impl yew_canvas::WithRander for Rander {
    fn rand(self, canvas: &web_sys::HtmlCanvasElement) {
        
    }
}