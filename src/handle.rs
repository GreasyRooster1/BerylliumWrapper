use beryllium::events::Event;
use beryllium::init::InitFlags;
use beryllium::Sdl;
use beryllium::video::{CreateWinArgs, RendererFlags, RendererWindow};
use crate::color::{ColorBuilder, RGBAColor};

pub struct GraphicsHandle{
    pub window:RendererWindow,
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

    //coloring
    pub fn fill(&mut self, r:f32, g:f32, b:f32){
        self.fill = RGBAColor::from_rgb_f32(r,g,b);
    }
    pub fn stroke(&mut self, r:f32, g:f32, b:f32){
        self.stroke = RGBAColor::from_rgb_f32(r,g,b);
    }
    pub fn blend(&self,mode:i32){
        todo!()
    }
    pub fn internally_fill(&mut self){
        let (r,g,b,a) = self.fill.decompose();
        self.window.set_draw_color(r,g,b,a).unwrap();
    }
    pub fn internally_fill_stroke(&mut self){
        let (r,g,b,a) = self.stroke.decompose();
        self.window.set_draw_color(r,g,b,a).unwrap();
    }

    //rendering
    pub fn background(&self, r:f32, g:f32, b:f32){
        let (r1,g1,b1,a1) = RGBAColor::from_rgb_f32(r,g,b).decompose();
        self.window.set_draw_color(r1,g1,b1,a1).unwrap();
        self.window.clear().unwrap()
    }
    pub fn present(&self){
        self.window.present();
    }

    //rect
    pub fn rect(&mut self, x:f32, y:f32, w:f32, h:f32){
        self.internally_fill();
        self.window.draw_rects(&[[x as i32,y as i32,w as i32,h as i32]]).unwrap()
    }
    pub fn rects(&mut self,rects:&[[f32;4]]){
        //self.window.draw_rects(rects).unwrap()
        todo!()
    }

    //ellipse
    pub fn ellipse(&mut self,x:f32,y:f32,w:f32,h:f32){}
    pub fn circle(&mut self,x:f32,y:f32,r:f32){}
    pub fn ellipses(&mut self,ellipses:&[[f32;4]]){}
    pub fn circles(&mut self,circles:&[[f32;3]]){}

    //tri
    pub fn tri(&mut self,x0:f32,y0:f32,x1:f32,y1:f32,x2:f32,y2:f32){}
    pub fn tris(&mut self,tris:&[[f32;6]]){}
}