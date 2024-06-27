use egui::{gui_zoom::kb_shortcuts, Button, Color32, DragValue, Rect, Ui, Vec2};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    intrin: SimIntrinsics,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct SimIntrinsics {
    elements: Vec<Element>,
    rules: Vec<Rule>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Element {
    color: Color32,
    name: String,
}

pub type ElementIndexBlock = [usize; 4];

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Default)]
pub struct Rule(ElementIndexBlock, ElementIndexBlock);

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
                rules: vec![Rule([1, 0, 1, 0], [0, 0, 1, 1])],
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
    if ui
        .add(DragValue::new(&mut num_elem).prefix("# of elements: "))
        .changed()
    {
        intrin.elements.resize_with(num_elem, || Element::default());
    }
    ui.separator();

    ui.strong("Rules");
    for rule in &mut intrin.rules {
        rule_editor(ui, rule, &intrin.elements);
    }
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

fn rule_editor(ui: &mut Ui, rule: &mut Rule, elements: &[Element]) {
    let Rule(pat_in, pat_out) = rule;
    ui.add_sized(Vec2::new(200., 50.), |ui: &mut Ui| {
        ui.columns(3, |cols| {
            block_editor(&mut cols[0], pat_in, elements);
            let ret = cols[1].centered_and_justified(|ui| {
                ui.label(" -> ")
            });
            block_editor(&mut cols[2], pat_out, elements);
            ret.inner
        })
    });
}

fn block_editor(ui: &mut Ui, block: &mut ElementIndexBlock, elements: &[Element]) {
    ui.columns(2, |cols| {
        for (col_idx, col) in cols.iter_mut().enumerate() {
            for row_idx in 0..2 {
                let block_idx = row_idx * 2 + col_idx;
                index_editor(col, &mut block[block_idx], elements);
            }
        }
    });
}

fn index_editor(ui: &mut Ui, index: &mut usize, elements: &[Element]) {
    let button = Button::new("")
        .fill(elements[*index].color)
        .wrap(false)
        .min_size(Vec2::splat(20.));

    if ui.add(button).clicked() {}
}
