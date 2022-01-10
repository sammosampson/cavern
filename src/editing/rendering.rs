
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
    ) -> bool {        
        if !graph.editor_visible() {
            return false;
        }
        
        self.begin_frame(display);
        self.set_visuals();
        
        self.render_windows(graph, event_producer);

        self.end_frame_and_paint(display, target)
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

    fn render_windows(
        &self,
        graph: &EditorGraph,
        event_producer: &mut SystemEventProducer
    ) {
        for window in graph.windows().iter() {
            match window.style() {
                EditorGraphWindowStyle::SideBar => 
                    self.render_side_panel(&window_name(window.id()), window, graph, event_producer),
                EditorGraphWindowStyle::Window =>  
                    self.render_window(&window_name(window.id()), window, graph, event_producer)
            }
        }
    }

    fn render_node(
        &self,
        ui: &mut egui::Ui,
        window: &EditorGraphWindow,
        graph: &EditorGraph,
        node: &EditorGraphNode,
        event_producer: &mut SystemEventProducer
    ) {
        match node {
            EditorGraphNode::Container { children } =>
                self.render_container(ui, window, children, graph, event_producer),
            EditorGraphNode::Seperator => 
                render_separator(ui),
            EditorGraphNode::ScrollArea { children } =>
                self.render_scroll_area(ui, window, children, graph, event_producer),
            EditorGraphNode::EntityListItems { item } => 
                render_entity_list_items(ui, window, graph, item, event_producer),
            EditorGraphNode::Vector { item, title, } => 
                self.render_entity_data(ui, window, graph, item, &title, event_producer),
            EditorGraphNode::Float { item, title, } => 
                self.render_entity_data(ui, window, graph, item, &title, event_producer),
        }
    }

    fn render_children(
        &self,
        ui: &mut egui::Ui,
        window: &EditorGraphWindow,
        children: &Vec<EditorGraphNode>,
        graph: &EditorGraph,
        event_producer: &mut SystemEventProducer
    ) {
        for node in children {
            self.render_node(ui, window, graph, node, event_producer);
        }
    } 

    fn render_side_panel(
        &self, 
        name: &str,
        window: &EditorGraphWindow,
        graph: &EditorGraph,
        event_producer: &mut SystemEventProducer
    ) {
        egui::SidePanel::left(name)
            .resizable(false)
            .show(self.egui.ctx(), |ui| {
                self.render_children(ui, window, window.controls(), graph, event_producer);
            });
    }
    
    fn render_window(
        &self,
        name: &str,
        window: &EditorGraphWindow,
        graph: &EditorGraph,
        event_producer: &mut SystemEventProducer
    ) {
        egui::Window::new(name)
            .resizable(false)
            .show(self.egui.ctx(), |ui| {
                self.render_children(ui, window, window.controls(), graph, event_producer);
            });
    }  

    fn render_container(
        &self, 
        ui: &mut egui::Ui,
        window: &EditorGraphWindow,
        children: &Vec<EditorGraphNode>,
        graph: &EditorGraph,
        event_producer: &mut SystemEventProducer
    ) {
        self.render_children(ui, window, children, graph, event_producer)
    }
    
    fn render_scroll_area(
        &self,
        ui: &mut egui::Ui,
        window: &EditorGraphWindow,
        children: &Vec<EditorGraphNode>,
        graph: &EditorGraph,
        event_producer: &mut SystemEventProducer
    ) {
        egui::ScrollArea::auto_sized()
            .show(ui, |ui| {
                for group_child_node in children {
                    self.render_node(ui, window, graph, group_child_node, event_producer);
                }    
            });
    }

    fn render_entity_data(
        &self,
        ui: &mut egui::Ui,
        window: &EditorGraphWindow,
        graph: &EditorGraph,
        item: &EditorGraphDataItem,
        label: &str,
        event_producer: &mut SystemEventProducer
    ) {
        if let Some(selected_entity) = window.selected_entity() {
            if let Some(data_item) = graph.entity_data().get(&(selected_entity, *item)) {
                match data_item {
                    EditorGraphData::EntityVector { entity, mut value } => {
                        ui.horizontal(|ui| {
                            ui.label(label);
                            if render_float(ui, "x:", &mut value.x) || render_float(ui, "y:", &mut value.y) {
                                event_producer.push(SystemEvent::EditorChange(EditorEvent::VectorChanged(*item, *entity, value)))
                            }
                        });
                    },
                    EditorGraphData::EntityFloat { entity, mut value } => {
                        ui.horizontal(|ui| {
                            if render_float(ui, label, &mut value) {
                                event_producer.push(SystemEvent::EditorChange(EditorEvent::FloatChanged(*item, *entity, value)))
                            }
                        });
                    },
                    _ => {}
                }
            }
        }
    }
}

fn render_entity_list_items(
    ui: &mut egui::Ui,
    window: &EditorGraphWindow,
    graph: &EditorGraph,
    item: &EditorGraphDataItem,
    event_producer: &mut SystemEventProducer
) {
    for ((_, list_item_data_item), list_item_data) in graph.entity_data() {
        if item != list_item_data_item {
            continue;
        }
        match list_item_data {
            EditorGraphData::EntityString { entity, value } => {
                if ui.selectable_label(window.is_selected_entity(entity), value).clicked() {
                    event_producer.push(SystemEvent::EditorChange(EditorEvent::EntitySelected(*entity, window.id())));
                }
            }
            _ => {},
        }
    }
}

fn render_float(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
    ui.label(label);
    ui.add(egui::DragValue::new(value)).changed()
}

fn render_separator(ui: &mut egui::Ui) {
    ui.separator();
}

fn window_name(id: u8) -> String {
    format!("window_{}", id)
}