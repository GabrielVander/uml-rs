mod infra;
use std::collections::HashMap;

use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Size, Theme,
    mouse::Cursor,
    widget::{
        Canvas,
        canvas::{self, Cache, Geometry, Path, Stroke, Text},
    },
};
use lib_core::domain::entities::diagram::{Diagram, Edge, EdgeStyle, Node, NodeType};

struct DiagramVisualizer {
    diagram: Diagram,
    // We need a way to map Node IDs to X,Y coordinates.
    // In a real app, you'd use a layout algorithm (like Force-Directed) to calculate these.
    positions: HashMap<String, Point>,
    cache: Cache, // Optimizes drawing performance
}

impl Default for DiagramVisualizer {
    fn default() -> Self {
        // Create Dummy Data
        let nodes = vec![
            Node::new("1".into(), NodeType::Component("Service A".into())),
            Node::new("2".into(), NodeType::Component("Database".into())),
            Node::new("3".into(), NodeType::Component("Cache".into())),
        ];
        let edges = vec![
            Edge {
                from_id: "1".into(),
                to_id: "2".into(),
                style_from: EdgeStyle::Solid,
                style_to: EdgeStyle::Arrow,
                label: None,
            },
            Edge {
                from_id: "1".into(),
                to_id: "3".into(),
                style_from: EdgeStyle::Solid,
                style_to: EdgeStyle::Arrow,
                label: None,
            },
        ];

        let diagram = Diagram::new(Some("Architecture".into()), nodes, edges);

        Self::new(diagram)
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl DiagramVisualizer {
    fn new(diagram: Diagram) -> Self {
        // Simple auto-layout: Grid layout for demonstration
        let mut positions = HashMap::new();
        let mut x = 50.0;
        let mut y = 50.0;

        for (i, node) in diagram.nodes.iter().enumerate() {
            positions.insert(node.id.clone(), Point::new(x, y));
            x += 150.0;
            if (i + 1) % 3 == 0 {
                // New row every 3 nodes
                x = 50.0;
                y += 150.0;
            }
        }

        Self {
            diagram,
            positions,
            cache: Cache::default(),
        }
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, _message: Message) {
        // Handle messages if needed
    }
}

impl canvas::Program<Message> for DiagramVisualizer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // 1. Draw Edges (First, so they are behind nodes)
            // for edge in &self.diagram.edges {
            //     if let (Some(start), Some(end)) = (
            //         self.positions.get(&edge.from_id),
            //         self.positions.get(&edge.to_id),
            //     ) {
            //         // Draw the line
            //         let path = Path::new(|b| {
            //             b.move_to(*start + Vector::new(50.0, 25.0)); // Center of node
            //             b.line_to(*end + Vector::new(50.0, 25.0));
            //         });
            //
            //         frame.stroke(&path, Stroke::default().with_width(2.0));
            //     }
            // }

            // 2. Draw Nodes
            for node in &self.diagram.nodes {
                if let Some(pos) = self.positions.get(&node.id) {
                    let size = Size::new(100.0, 50.0);

                    // Draw Box
                    let node_rect = Path::rectangle(*pos, size);
                    frame.fill(&node_rect, Color::from_rgb8(100, 149, 237)); // Cornflower Blue
                    frame.stroke(&node_rect, Stroke::default());

                    // Draw Text Label
                    if let NodeType::Component(name) = &node.r#type {
                        frame.fill_text(Text {
                            content: name.clone(),
                            position: *pos + Vector::new(50.0, 25.0), // Center text
                            color: Color::WHITE,
                            horizontal_alignment: iced::alignment::Horizontal::Center,
                            vertical_alignment: iced::alignment::Vertical::Center,
                            ..Text::default()
                        });
                    }
                }
            }
        });

        vec![geometry]
    }
}

// --- [Run Application] ---
use iced::Vector;

use crate::infra::iced_application::UmlRsIcedDesktopApplication;

pub fn main() -> iced::Result {
    let app: UmlRsIcedDesktopApplication = UmlRsIcedDesktopApplication::default();

    iced::application(
        "Uml-rs",
        |app: &mut UmlRsIcedDesktopApplication, message| app.update(message),
        UmlRsIcedDesktopApplication::view,
    )
    .run()
}
