use std::sync::{Arc, Mutex};

use mlua::{ Function, UserData };

use crate::loader::Runtime;

// TODO: use this directly to handle things
pub struct MainModule;

impl UserData for MainModule {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_set("init", |lua, this, func: Function| {
            lua.set_named_registry_value("init_callback", func);
            Ok(())
        });
        fields.add_field_method_set("draw", |lua, this, func: Function| {
            lua.set_named_registry_value("draw_callback", func);
            Ok(())
        });
        
    }
} 
