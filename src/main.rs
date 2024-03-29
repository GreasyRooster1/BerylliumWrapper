mod handle;
mod color;

use beryllium::events::Event;
use crate::color::{ColorBuilder, RGBAColor};
use crate::handle::GraphicsHandle;

fn main() {
    let mut graphics = &mut GraphicsHandle::init("Wrapper Graphics Test", 500, 500, [true,false,false]);
    start_loop(graphics,|g|{
        g.background(0.0,127.0,255.0);
        g.fill(127.0,255.0,127.0);
        g.rect(10.0,10.0,100.0,100.0);
    });

}

fn start_loop<F>(graphics: &mut GraphicsHandle, mut func:F)
    where F:FnMut(&mut GraphicsHandle) {
    graphics.background(127.0,127.0,127.0);
    'main_loop: loop {
        #[allow(clippy::never_loop)]
        #[allow(clippy::single_match)]
        while let Some((event, _timestamp)) = graphics.sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
        func(graphics);
        graphics.present();
    }
}
