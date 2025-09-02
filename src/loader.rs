use std::{ fs, path::PathBuf, sync::{ Arc, Mutex } };

use mlua::{ AnyUserData, FromLuaMulti, Function, IntoLua, Lua, Number, Table, UserData, Vector };
use mlua_luau_scheduler::{ Functions, Scheduler };
use sdl2::{ rect::Rect, render::Canvas, video::Window };

use crate::modules::index::MainModule;
use crate::modules::graphics::GraphicsModule;

use tokio::sync::mpsc::{Sender, Receiver};

pub struct Runtime {
    pub canvas: Arc<Mutex<Canvas<Window>>>,
    luau: Lua,
    scheduler: Scheduler,
    fns: Functions,

    channel: Receiver<Event>
}

enum Event {
    Init,
    Draw,
    Update(u64),
}

// TODO: use async
impl Runtime {
    pub fn new(canvas: Arc<Mutex<Canvas<Window>>>, event_channel: Receiver<Event>) -> Self {
        let result = Lua::new();

        let sched = Scheduler::new(result.clone());
        let fns = Functions::new(result.clone()).unwrap();

        let my = Self {
            canvas: canvas,
            luau: result.clone(),
            scheduler: sched,
            fns: fns,
        };

        let instance = Arc::new(Mutex::new(my));
        
        result.register_module("@hatred/graphics", GraphicsModule(Arc::clone(&instance))).unwrap();
        result.register_module("@hatred/hatred", MainModule(Arc::clone(&instance))).unwrap();
 
        result.sandbox(true).unwrap();

        instance
    }

    pub fn run_file(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let contents = fs::read_to_string(path.into())?;
        self.luau.load(contents);
        Ok(())
    }
    pub async fn handle_event(&self, event: Event) {
        match event {
            Event::Draw => {
                if let Some(func) = &self.draw_func {
                    let result: () = func.call(()).unwrap();
                }
            }
            Event::Init => {
                if let Some(func) = &self.init_func {
                    let result: () = func.call(()).unwrap();
                }
            }
            Event::Update(dt) => {
                if let Some(func) = &self.update {
                    let result: () = func.call(dt).unwrap();
                }
            }
        }
    }
}
