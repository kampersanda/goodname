pub mod app;
pub mod cand_view;
pub mod range_input;
pub mod text_input;

use app::App;

fn main() {
    yew::start_app::<App>();
}
