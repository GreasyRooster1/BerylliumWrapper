mod handle;
mod color;

use beryllium::events::Event;
use BerylliumWrapper::start_loop;
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
