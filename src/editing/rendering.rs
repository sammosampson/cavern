
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

    pub fn process_event(&mut self, event: &WindowEvent){
        println!("{:?}", event);
        self.egui.on_event(event);
    }

    pub fn render(
        &mut self,
        graph: &mut EditorGraph,
        event_producer: &mut SystemEventProducer,
        display: &Display,
        target: &mut Frame
    ) -> bool {        
        if !graph.editor_visible() {
            return false;
        }

        self.begin_frame(display);
        self.set_visuals();
        
        for node in graph.controls() {
            self.render_topmost(graph, &node, event_producer)
        }

        graph.clear_data();
        self.end_frame_and_paint(&display, target)
    }

    fn set_visuals(&mut self) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 220);
        self.egui.ctx().set_visuals(visuals);
    }

    fn begin_frame(&mut self, display: &Display) {
        self.egui.begin_frame(display);
    }

    fn end_frame_and_paint(&mut self, display: &Display, target: &mut Frame) -> bool {
        let (needs_repaint, shapes) = self.egui.end_frame(&display);
        self.egui.paint(&display, target, shapes);
        needs_repaint
    }

    fn render_topmost(
        &self,
        graph: &EditorGraph,
        node: &EditorGraphNode,
        event_producer: &mut SystemEventProducer
    ) {
        match node {
            EditorGraphNode::SideBar { name, children } => 
                self.render_side_panel(name, children, graph, event_producer),
            EditorGraphNode::Window { name, children } =>  
                self.render_window(name, children, graph, event_producer),
            _ => {}
        }
    }

    fn render_node(
        &self,
        ui: &mut egui::Ui,
        data: &HashMap<EditorGraphDataItem, EditorGraphData>,
        node: &EditorGraphNode,
        event_producer: &mut SystemEventProducer
    ) {
        self.render_data(ui, data, node, event_producer)
    }

    fn render_data(
        &self,
        ui: &mut egui::Ui,
        data: &HashMap<EditorGraphDataItem, EditorGraphData>,
        node: &EditorGraphNode,
        event_producer: &mut SystemEventProducer
    ) {
        match node {
            EditorGraphNode::Toggle { item, title, click_handler } => {
                self.render_toggle(data, item, &title, ui, event_producer, click_handler);
            },
            _ => {}
        }
    }

    fn render_side_panel(&self, name: &String, children: &Vec<EditorGraphNode>, graph: &EditorGraph, event_producer: &mut SystemEventProducer) {
        egui::SidePanel::left(name)
            .resizable(false)
            .show(self.egui.ctx(), |ui| {
                self.render_children(children, ui, graph, event_producer);
            });
    }
    
    fn render_window(&self, name: &String, children: &Vec<EditorGraphNode>, graph: &EditorGraph, event_producer: &mut SystemEventProducer) {
        if !graph.is_window_visible(name) {
            return;
        }
        egui::Window::new(name)
            .resizable(false)
            .show(self.egui.ctx(), |ui| {
                self.render_children(children, ui, graph, event_producer);
            });
    }

    fn render_children(&self, children: &Vec<EditorGraphNode>, ui: &mut egui::Ui, graph: &EditorGraph, event_producer: &mut SystemEventProducer) {
        for node in children {
            self.render_node(ui, &graph.data(), node, event_producer);
        }
    }

    fn render_toggle(
        &self,
        data: &HashMap<EditorGraphDataItem, EditorGraphData>,
        item: &EditorGraphDataItem,
        label: &str,
        ui: &mut egui::Ui,
        event_producer: &mut SystemEventProducer,
        click_handler: &Box<dyn Fn(bool) -> EditorEvent>
    ) {
        let data_item = if !data.contains_key(item) {
            &EditorGraphData::Boolean { value: false }
        } else {
            data.get(item).unwrap()
        };

        match data_item {
            EditorGraphData::Boolean { mut value } => {            
                if self.render_editable_bool(ui, label, &mut value) {
                    event_producer.push(SystemEvent::EditorChange(click_handler(value)));
                }
            }
        }
    }

    fn render_editable_bool(&self, ui: &mut egui::Ui, label: &str, value: &mut bool) -> bool {
        ui.add(egui::Checkbox::new(value, label)).changed()
    }
}