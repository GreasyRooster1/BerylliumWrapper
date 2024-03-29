mod handle;
mod color;

use crate::color::{ColorBuilder, RGBAColor};
use crate::handle::GraphicsHandle;
use crate::handle::RectRendering;
use crate::handle::Coloring;

fn main() {
    let mut graphics = &mut GraphicsHandle::init("Wrapper Graphics Test", 500, 500, [true,false,false]);
    graphics.start_loop(||{
        graphics.fill(255.0,255.0,255.0);
        graphics.rect(10.0,10.0,100.0,100.0);
    });

}
