use egui::{
    ahash::HashMap, gui_zoom::kb_shortcuts, popup_below_widget, Button, Color32, DragValue, Id,
    Rect, ScrollArea, Ui, Vec2, Widget,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct TemplateApp {
    sim: Sim,
}

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
            sim_intrin_editor(ui, &mut self.sim.intrin);
            sim_rule_editor(ui, &mut self.sim.rules, &self.sim.intrin);
        });
    }
}

fn sim_intrin_editor(ui: &mut Ui, intrin: &mut SimIntrinsics) {
    ui.strong("Elements");
    ScrollArea::vertical().show(ui, |ui| {
        for element in &mut intrin.elements {
            ui.horizontal(|ui| {
                element_editor(ui, element);
            });
        }
    });

    let mut num_elem = intrin.elements.len();
    if ui
        .add(DragValue::new(&mut num_elem).prefix("# of elements: "))
        .changed()
    {
        intrin.elements.resize_with(num_elem, || Element::default());
    }
    ui.separator();

    ui.strong("Symmetry");
    ui.checkbox(&mut intrin.symmetry.horizontal, "Horizontal");
    ui.separator();
}

fn sim_rule_editor(
    ui: &mut Ui,
    rules: &mut HashMap<ElementIndexBlock, ElementIndexBlock>,
    intrin: &SimIntrinsics,
) {
    ui.strong("Rules");
    for (pat_in, pat_out) in rules {
        rule_editor(ui, pat_in, pat_out, &intrin.elements);
    }
    ui.separator();
}

fn element_editor(ui: &mut Ui, element: &mut Element) {
    ui.text_edit_singleline(&mut element.name);
    ui.color_edit_button_srgba(&mut element.color);
}

impl Default for Element {
    fn default() -> Self {
        Self {
            color: Color32::GREEN,
            name: "New element".into(),
        }
    }
}

fn rule_editor(
    ui: &mut Ui,
    input_pat: &ElementIndexBlock,
    output_pat: &mut ElementIndexBlock,
    elements: &[Element],
) {
    ui.add_sized(Vec2::new(200., 50.), |ui: &mut Ui| {
        ui.columns(3, |cols| {
            block_editor(&mut cols[0], &mut input_pat.clone(), elements);
            let ret = cols[1].centered_and_justified(|ui| ui.label(" -> "));
            block_editor(&mut cols[2], output_pat, elements);
            ret.inner
        })
    });
}

fn block_editor(ui: &mut Ui, block: &mut ElementIndexBlock, elements: &[Element]) {
    ui.columns(2, |cols| {
        for (col_idx, col) in cols.iter_mut().enumerate() {
            for row_idx in 0..2 {
                let block_idx = row_idx * 2 + col_idx;
                element_selector(col, &mut block[block_idx], elements);
            }
        }
    });
}

fn element_selector(ui: &mut Ui, selected_element: &mut usize, elements: &[Element]) {
    let resp = element_button(ui, &elements[*selected_element]);

    let ptr = selected_element as *const usize as u64;
    let popup_id = Id::new(("index_editor_popup", ptr));

    if resp.clicked() {
        ui.memory_mut(|mem| mem.open_popup(popup_id));
    }

    popup_below_widget(ui, popup_id, &resp, |ui| {
        ui.horizontal(|ui| {
            for (idx, element) in elements.iter().enumerate() {
                if element_button(ui, element).clicked() {
                    *selected_element = idx;
                }
            }
        });
    });
}

fn element_button(ui: &mut Ui, element: &Element) -> egui::Response {
    Button::new("")
        .fill(element.color)
        .wrap(false)
        .min_size(Vec2::splat(20.))
        .ui(ui)
        .on_hover_ui(|ui| {
            ui.label(&element.name);
        })
}

impl SimIntrinsics {}
