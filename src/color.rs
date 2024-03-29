use std::cmp::min;

pub struct RGBAColor{
    r:u8,
    g:u8,
    b:u8,
    a:u8,
}

pub trait ColorBuilder{
    fn from_grayscale_u8(v:u8)->Self;
    fn from_grayscale_i32(v:i32)->Self;
    fn from_grayscale_f32(v:f32)->Self;
    fn from_rgb_u8(r:u8,g:u8,b:u8)->Self;
    fn from_rgb_i32(r:i32,g:i32,b:i32)->Self;
    fn from_rgb_f32(r:f32,g:f32,b:f32)->Self;
    fn from_rgba_u8(r:u8,g:u8,b:u8,a:u8)->Self;
    fn from_rgba_i32(r:i32,g:i32,b:i32,a:i32)->Self;
    fn from_rgba_f32(r:f32,g:f32,b:f32,a:f32)->Self;
    fn decompose(&self) ->(u8, u8, u8, u8);
}

impl ColorBuilder for RGBAColor{
    fn from_grayscale_u8(v:u8)->RGBAColor{
        RGBAColor{
            r: v,
            g: v,
            b: v,
            a: 255,
        }
    }
    fn from_grayscale_i32(v:i32)->RGBAColor{
        let v_u8 = min(v, 255) as u8;
        RGBAColor{
            r: v_u8,
            g: v_u8,
            b: v_u8,
            a: 255,
        }
    }
    fn from_grayscale_f32(v:f32)->RGBAColor{
        let v_u8 = f32::min(v, 255.0) as u8;
        RGBAColor{
            r: v_u8,
            g: v_u8,
            b: v_u8,
            a: 255,
        }
    }
    fn from_rgb_u8(r:u8,g:u8,b:u8)->RGBAColor{
        RGBAColor{
            r,
            g,
            b,
            a: 255,
        }
    }
    fn from_rgb_i32(r:i32,g:i32,b:i32)->RGBAColor{
        RGBAColor{
            r: min(r, 255) as u8,
            g: min(g, 255) as u8,
            b: min(b, 255) as u8,
            a: 255,
        }
    }
    fn from_rgb_f32(r:f32,g:f32,b:f32)->RGBAColor{
        RGBAColor{
            r: f32::min(r, 255.0) as u8,
            g: f32::min(g, 255.0) as u8,
            b: f32::min(b, 255.0) as u8,
            a: 255,
        }
    }
    fn from_rgba_u8(r:u8,g:u8,b:u8,a:u8)->RGBAColor{
        RGBAColor{
            r,
            g,
            b,
            a,
        }
    }
    fn from_rgba_i32(r:i32,g:i32,b:i32,a:i32)->RGBAColor{
        RGBAColor{
            r: min(r, 255) as u8,
            g: min(g, 255) as u8,
            b: min(b, 255) as u8,
            a: min(a, 255) as u8,
        }
    }
    fn from_rgba_f32(r:f32,g:f32,b:f32,a:f32)->RGBAColor{
        RGBAColor{
            r: f32::min(r, 255.0) as u8,
            g: f32::min(g, 255.0) as u8,
            b: f32::min(b, 255.0) as u8,
            a: f32::min(a, 255.0) as u8,
        }
    }

    fn decompose(&self)->(u8,u8,u8,u8){
        (self.r,self.g,self.b,self.a)
    }
}