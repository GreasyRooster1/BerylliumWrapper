use beryllium::events::Event;
use beryllium::init::InitFlags;
use beryllium::Sdl;
use beryllium::video::{CreateWinArgs, RendererFlags, RendererWindow};
use crate::color::{ColorBuilder, RGBAColor};
pub struct GraphicsHandle{
    window:RendererWindow,
    pub sdl:Sdl,
    fill:RGBAColor,
    stroke:RGBAColor,
    blend:i32,
}
impl GraphicsHandle{
    ///options \[0] - resizable
    ///
    ///options \[1] - borderless
    ///
    ///options \[3] - allow_high_dpi
    pub fn init(title:&str,width:i32,height:i32,options:[bool;3])->GraphicsHandle{
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        GraphicsHandle{
            window: sdl
                .create_renderer_window(
                    CreateWinArgs {
                        title,
                        width,
                        height,
                        resizable: options[0],
                        borderless: options[1],
                        allow_high_dpi: options[2],

                    },
                    RendererFlags::ACCELERATED_VSYNC,
                )
                .unwrap(),
            sdl,
            fill: RGBAColor::from_grayscale_u8(255),
            stroke: RGBAColor::from_grayscale_u8(0),
            blend: 0,
        }
    }
}
pub trait Coloring{
    fn fill(&mut self,r:f32,g:f32,b:f32){}
    fn stroke(&mut self,r:f32,g:f32,b:f32){}
    fn blend(&self,mode:i32){}
    fn internally_fill(&self){}
    fn internally_fill_stroke(&self){}
}
pub trait RectRendering{
    fn rect(&self,x:f32,y:f32,w:f32,h:f32){}
    fn rects(&mut self,rects:&[[f32;4]]){}
}
pub trait EllipseRendering{
    fn ellipse(&mut self,x:f32,y:f32,w:f32,h:f32){}
    fn circle(&mut self,x:f32,y:f32,r:f32){}
    fn ellipses(&mut self,ellipses:&[[f32;4]]){}
    fn circles(&mut self,circles:&[[f32;3]]){}
}
pub trait TriRendering{
    fn tri(&mut self,x0:f32,y0:f32,x1:f32,y1:f32,x2:f32,y2:f32){}
    fn tris(&mut self,tris:&[[f32;6]]){}
}

impl Coloring for GraphicsHandle{
    fn fill(&mut self, r:f32, g:f32, b:f32){
        self.fill = RGBAColor::from_rgb_f32(r,g,b);
    }
    fn stroke(&mut self, r:f32, g:f32, b:f32){
        self.stroke = RGBAColor::from_rgb_f32(r,g,b);
    }
    fn blend(&self,mode:i32){
        todo!()
    }
    fn internally_fill(&self){
        let (r,g,b,a) = self.fill.decompose();
        self.window.set_draw_color(r,g,b,a).unwrap();
    }
    fn internally_fill_stroke(&self){
        let (r,g,b,a) = self.stroke.decompose();
        self.window.set_draw_color(r,g,b,a).unwrap();
    }
}

impl RectRendering for GraphicsHandle{
    fn rect(&self,x:f32,y:f32,w:f32,h:f32){
        self.internally_fill();
        self.window.draw_rects(&[[x as i32,y as i32,w as i32,h as i32]]).unwrap()
    }
    fn rects(&mut self,rects:&[[f32;4]]){
        //self.window.draw_rects(rects).unwrap()
        todo!()
    }
}

impl EllipseRendering for GraphicsHandle{
    
}