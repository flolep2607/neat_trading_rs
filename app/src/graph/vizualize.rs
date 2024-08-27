use crate::graph::main::Graph;
use eframe;
use egui::FontFamily::Proportional;
use egui::FontId;

const CIRCLE_RADIUS: f32 = 20.0;

pub struct GraphApp {
    pub graph: Graph,
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let font_id = FontId::new(20.0, Proportional);
            ui.heading("Graph Visualization");

            for (vertex, edges) in &self.graph.adjacency_list {
                ui.label(format!("{}: {:?}", vertex, edges));
            }

            // Drawing the graph
            let painter = ui.painter();
            let rect = ui.max_rect();
            let center = rect.center();

            let radius = 50.0;
            let mut vertices = vec![];
            for (i, vertex) in self.graph.adjacency_list.keys().enumerate() {
                let angle = i as f32 * std::f32::consts::PI * 2.0 / self.graph.adjacency_list.len() as f32;
                let pos = center + egui::vec2(angle.cos(), angle.sin()) * radius;
                vertices.push((vertex, pos));
            }

            // Drawing the vertices
            for (vertex, pos) in &vertices {
                painter.circle_filled(*pos, CIRCLE_RADIUS, egui::Color32::LIGHT_BLUE);
                painter.text(
                    *pos,
                    egui::Align2::CENTER_CENTER,
                    vertex,
                    font_id.clone(),
                    egui::Color32::BLACK,
                );
            }

            // Drawing the edges
            for (from, to_list) in &self.graph.adjacency_list {
                if let Some((_, from_pos)) = vertices.iter().find(|(v, _)| *v == from) {
                    for to in to_list {
                        if let Some((_, to_pos)) = vertices.iter().find(|(v, _)| *v == to) {
                            // start on the edge of the circle
                            let from_point =
                                *from_pos + (*to_pos - *from_pos).normalized() * CIRCLE_RADIUS;
                            // arrow to the side of the circle
                            let to_point =
                                *to_pos - (*to_pos - *from_pos).normalized() * CIRCLE_RADIUS;
                            let vec_to = to_point - from_point;
                            painter.arrow(
                                from_point,
                                vec_to,
                                egui::Stroke::new(2.0, egui::Color32::RED),
                            );
                        }
                    }
                }
            }
        });
    }
}
