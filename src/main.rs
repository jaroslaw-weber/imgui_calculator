extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate meval;
use imgui::*;
mod support;

const CLEAR_COLOR: [f32; 4] = [114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 1.0];

fn main() {
    let mut state = CalculatorState::default();

    support::run(
        "Imgui Calculator".to_owned(),
        CLEAR_COLOR,
        |ui| {
            show_calculator(ui, &mut state);
            true
        },
        (230, 300),
    );
}
//calculator
struct CalculatorState {
    expression: String,
}

impl Default for CalculatorState {
    fn default() -> Self {
        CalculatorState {
            expression: "".to_owned(),
        }
    }
}

fn show_calculator(ui: &Ui, mut state: &mut CalculatorState) {
    ui.window(im_str!("Imgui Calculator"))
        .position((0.0, 0.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(
                "This is calculator.
You can make calculations",
            );
            ui.separator();
            ui.text(&state.expression);
            ui.separator();
            for i in 1..10 {
                let i_string = i.to_string();
                add_button(&ui, &mut state, &i_string);

                if i % 3 != 0 && i != 9 {
                    ui.same_line(0.0);
                }
            }
            //
            add_button(&ui, &mut state, "+");
            ui.same_line(0.0);
            add_button(&ui, &mut state, "-");
            ui.same_line(0.0);
            add_button(&ui, &mut state, "*");
            //
            add_button(&ui, &mut state, "/");
            ui.same_line(0.0);
            if ui.small_button(im_str!("=")) {
                let evaluated = meval::eval_str(&state.expression).unwrap(); //todo error handling
                state.expression = format!("{}", evaluated);
            }
        });
}

fn add_button(ui: &Ui, state: &mut CalculatorState, button_text: &str) {
    if ui.small_button(im_str!("{}", button_text)) {
        let new_expression = format!("{}{}", &state.expression, button_text);
        state.expression = new_expression.to_owned();
    }
}
