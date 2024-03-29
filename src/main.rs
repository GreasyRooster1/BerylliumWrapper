mod handle;
mod color;

use beryllium::events::Event;
use crate::color::{ColorBuilder, RGBAColor};
use crate::handle::GraphicsHandle;
use crate::handle::RectRendering;
use crate::handle::Coloring;

fn main() {
    let mut graphics = &mut GraphicsHandle::init("Wrapper Graphics Test", 500, 500, [true,false,false]);

    'main_loop: loop {
        #[allow(clippy::never_loop)]
        #[allow(clippy::single_match)]
        while let Some((event, _timestamp)) = graphics.sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
        graphics.fill(255.0,255.0,255.0);
        graphics.rect(10.0,10.0,100.0,100.0);
    }
}
