use imgui::*;

mod http;
mod main_menu_bar;
mod structs;
mod support;

use structs::State;

fn main() {
    let mut state = State::default();
    let mut system = support::init(file!());
    // Change capture to pass dimension as captured variable
    system.main_loop(|run, ui, dimensions| {
        show_main_app(ui, &mut state, run, dimensions);
        //show_test_window(ui);
    });
}

fn show_main_app(ui: &Ui, state: &mut State, _opened: &mut bool, dimensions: (u32, u32)) {
    if state.show_app_main_menu_bar {
        main_menu_bar::show_app_main_menu_bar(ui, state);
        show_main_app_window(ui, state, dimensions);
    }
}

fn show_main_app_window(ui: &Ui, state: &mut State, dimensions: (u32, u32)) {
    ui.window(im_str!("Main"))
        .position([0.0, 15.0], Condition::Always)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .collapsible(false)
        .no_bring_to_front_on_focus(true)
        .size(
            [dimensions.0 as f32, (dimensions.1 as f32) - 15.0],
            Condition::Always,
        )
        .build(|| {
            // Print frame dimensions
            ui.text(im_str!("Current frame dimensions: {:?}", dimensions));

            // state.main_body_text is first default string, then it's whatever is defined via http below
            // calling im_str!() has some nuances; see https://github.com/Gekkio/imgui-rs/issues/7
            // This will mostly use dynamically-allocated Strings (i.e., those not known at compile time)
            // So, use ImString to print Strings rather than string slices (i.e., type &str)
            let string_to_display = ImString::new(&state.main_body_text);
            
            // TODO: Consider checking ImStrings using ImString::from_utf8_unchecked()

            // Note that 'chars' aren't well-defined in Rust due to encoding relations
            let string_length_by_chars = state.main_body_text.chars().count();
            let string_capacity_by_bytes = state.main_body_text.capacity();
            let im_string_capacity_by_bytes = string_to_display.capacity();
            let im_string_capacity_by_bytes_with_nul = string_to_display.capacity_with_nul();
            ui.text_wrapped(&im_str!("Length of String (by chars): {}\n", string_length_by_chars));
            ui.text_wrapped(&im_str!("Capacity of String (by bytes): {}\n", string_capacity_by_bytes));
            ui.text_wrapped(&im_str!("Capacity of ImString (by bytes w/o nul terminator): {}\n", im_string_capacity_by_bytes));
            ui.text_wrapped(&im_str!("Capacity of ImString (by bytes w/ nul terminator): {}\n", im_string_capacity_by_bytes_with_nul));
            
            //ui.text_wrapped(&string_to_display);

            ui.text(im_str!("Press the green square to pull sample html:"));
            if ui
                .color_button(im_str!("Green color"), [0.0, 1.0, 0.0, 1.0])
                .size([100.0, 50.0])
                .build()
            {
                // Get dynamically allocated text via reqwest, store in state.main_body_text
                // TODO: Add error handling (unwrapping an error will crash the program in its current state)
                state.main_body_text = http::get_text(&state.url_to_get).unwrap();
            }
        });
}

fn show_test_window(ui: &Ui) {
    ui.window(im_str!("Microll"))
        .size([0.0, 0.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello, world!"));
            ui.text(im_str!("Example program:"));
            ui.text(im_str!("Microll by Tom Hightower"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
}
