mod handle;
mod color;
mod event;

use beryllium::events::Event;
use crate::color::{ColorBuilder, RGBAColor};
use crate::handle::GraphicsHandle;
use crate::event::*;

fn main() {
    let mut graphics = &mut GraphicsHandle::init("Wrapper Graphics Test", 500, 500, [true,false,false]);
    start_loop(graphics,draw);
}

fn draw(g:&mut GraphicsHandle){
    g.background(0.0,127.0,255.0);
    g.fill(127.0,255.0,127.0);
    g.rect(10.0,10.0,100.0,100.0);
}

fn mouse_moved(g:&mut GraphicsHandle){

}
