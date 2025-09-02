use std::sync::{ Arc, Mutex };

use mlua::{ AnyUserData, Number, UserData, Vector };
use sdl2::{ rect::Rect, render::Canvas, video::Window };

use crate::loader::Runtime;

pub struct GraphicsModule(pub Arc<Mutex<Canvas<Window>>>);

impl UserData for GraphicsModule {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("rectangle", |lua, this, args: (Vector, Vector, AnyUserData)| {
            let rect = Rect::new(
                args.0.x() as i32,
                args.0.y() as i32,
                args.1.x().abs() as u32,
                args.1.y().abs() as u32
            );
            let color = args.2.borrow::<ColorUserData>()?;

            let mut lock = this.0.lock().unwrap();

            let canvas = &mut lock.canvas;
            canvas.set_draw_color((color.r, color.g, color.b, color.a));
            canvas.draw_rect(rect).unwrap();

            Ok(())
        });
    }
}

pub struct ColorUserData {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl UserData for ColorUserData {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut(
            "new",
            |_, args: (Option<Number>, Option<Number>, Option<Number>, Option<Number>)| {
                let transform_num = |x: Option<Number>, default: u8|
                    x.map(|x| (x * 255.0).round() as u8).unwrap_or(default);
                Ok(ColorUserData {
                    r: transform_num(args.0, 0),
                    g: transform_num(args.1, 0),
                    b: transform_num(args.2, 0),
                    a: transform_num(args.3, 1),
                })
            }
        );
        methods.add_function_mut(
            "rgb",
            |_, args: (Option<Number>, Option<Number>, Option<Number>, Option<Number>)| {
                let transform_num = |x: Option<Number>, default: u8|
                    x.map(|x| x as u8).unwrap_or(default);
                Ok(ColorUserData {
                    r: transform_num(args.0, 0),
                    g: transform_num(args.1, 0),
                    b: transform_num(args.2, 0),
                    a: transform_num(args.0, 255),
                })
            }
        );
        methods.add_function("unpack", |_, args: AnyUserData| {
            let color = args.borrow::<ColorUserData>()?;
            Ok((color.r, color.g, color.b, color.a))
        });
    }
}
