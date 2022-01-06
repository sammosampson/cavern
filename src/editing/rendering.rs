
use crate::prelude::*;

use egui_glium::*;

pub fn create_editor_renderer(display: &Display) -> EditorRenderer {
    EditorRenderer::new(display)
}

pub struct EditorRenderer {
    egui: EguiGlium
}

impl EditorRenderer {
    pub fn new(display: &Display) -> Self {
        Self {
            egui: EguiGlium::new(display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }

    pub fn render(
        &mut self,
        graph: &mut EditorGraph,
        event_producer: &mut SystemEventProducer,
        display: &Display,
        target: &mut Frame
    ) {        
        if !graph.editor_visible() {
            return;
        }

        self.begin_frame(display);
        self.set_visuals();
        
        for node in graph.controls() {
            self.render_topmost(graph, &node, event_producer)
        }

        graph.clear_data();
        self.end_frame_and_paint(&display, target);
    }

    fn set_visuals(&mut self) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 220);
        self.egui.ctx().set_visuals(visuals);
    }

    fn render_topmost(
        &self,
        graph: &EditorGraph,
        node: &EditorGraphNode,
        event_producer: &mut SystemEventProducer
    ) {
        match node {
            EditorGraphNode::SideBar { name, children } => {
                egui::SidePanel::left(name)
                    .resizable(false)
                    .show(self.egui.ctx(), |ui| {
                        for node in children {
                            //self.render_node(ui, &graph.data(), node, event_producer);
                        }
                    });
            },
            _ => {}
        }
    }
    

    fn begin_frame(&mut self, display: &Display) {
        self.egui.begin_frame(display);
    }

    fn end_frame_and_paint(&mut self, display: &Display, target: &mut Frame) {
        let (_needs_repaint, shapes) = self.egui.end_frame(&display);
        self.egui.paint(&display, target, shapes);
    }
}