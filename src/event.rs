use beryllium::events::Event;
use crate::handle::GraphicsHandle;

pub fn start_loop<F>(graphics: &mut GraphicsHandle, mut func:F)
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
