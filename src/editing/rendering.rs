
use crate::prelude::*;

use egui_glium::*;

pub struct EditorRenderer {
    egui: EguiGlium
}

impl EditorRenderer {
    pub fn new(display: &glium::backend::glutin::Display) -> Self {
        Self {
            egui: EguiGlium::new(display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }
}