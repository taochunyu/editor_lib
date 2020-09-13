mod state;
mod widget;
mod service;
mod component;

use druid::{AppLauncher, PlatformError, WindowDesc, Size, MenuDesc};
use crate::component::app::build_app;
use crate::state::State;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_app)
        .window_size(Size::new(1200.0, 900.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(State::new())
}
