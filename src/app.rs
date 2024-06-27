use egui::{gui_zoom::kb_shortcuts, Color32, DragValue, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    intrin: SimIntrinsics,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct SimIntrinsics {
    elements: Vec<Element>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Element {
    color: Color32,
    name: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            intrin: SimIntrinsics {
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
            },
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            sim_intrin_editor(ui, &mut self.intrin);
        });
    }
}

fn sim_intrin_editor(ui: &mut Ui, intrin: &mut SimIntrinsics) {
    ui.strong("Elements");
    for element in &mut intrin.elements {
        ui.horizontal(|ui| {
            element_editor(ui, element);
        });
    }

    let mut num_elem = intrin.elements.len();
    if ui.add(DragValue::new(&mut num_elem).prefix("# of elements: ")).changed() {
        intrin.elements.resize_with(num_elem, || Element::default());
    }

    ui.strong("Rules");
}

fn element_editor(ui: &mut Ui, element: &mut Element) {
    ui.text_edit_singleline(&mut element.name);
    ui.color_edit_button_srgba(&mut element.color);
}

impl Default for Element {
    fn default() -> Self {
        Self { color: Color32::GREEN, name: "New element".into() }
    }
}
