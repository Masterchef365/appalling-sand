#![warn(clippy::all, rust_2018_idioms)]

use egui::{
    ahash::HashMap, gui_zoom::kb_shortcuts, popup_below_widget, Button, Color32, DragValue, Id,
    Rect, ScrollArea, Ui, Vec2, Widget,
};


mod app;
pub use app::SandApp;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Sim {
    intrin: SimIntrinsics,
    rules: HashMap<ElementIndexBlock, ElementIndexBlock>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SimIntrinsics {
    elements: Vec<Element>,
    symmetry: Symmetry,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Symmetry {
    /// Symmetry over the y axis
    horizontal: bool,
    //vertical: bool,
    //rotational: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Element {
    color: Color32,
    name: String,
}

pub type ElementIndexBlock = [usize; 4];

impl Default for SimIntrinsics {
    fn default() -> Self {
        Self {
            elements: vec![
                Element {
                    color: Color32::BLACK,
                    name: "Off".to_string(),
                },
                Element {
                    color: Color32::WHITE,
                    name: "On".to_string(),
                },
            ],
            symmetry: Symmetry { horizontal: true },
        }
    }
}


