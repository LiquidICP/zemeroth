use mq as macroquad;
use mq::experimental::graphics;
use zgui as ui;

mod common;

#[derive(Clone, Copy, Debug)]
enum Message {
    AddOrRemove,
}

fn make_gui(font: graphics::Font) -> ui::Result<ui::Gui<Message>> {
    let font_size = 64;
    let mut gui = ui::Gui::new();
    let anchor = ui::Anchor(ui::HAnchor::Right, ui::VAnchor::Bottom);
    let text = ui::Drawable::text("Button", font, font_size);
    let button = ui::Button::new(text, 0.2, gui.sender(), Message::AddOrRemove)?;
    gui.add(&ui::pack(button), anchor);
    Ok(gui)
}

fn make_label(assets: &common::Assets) -> ui::Result<ui::RcWidget> {
    let texture = ui::Drawable::Texture(assets.texture);
    let label = ui::Label::new(texture, 0.3)?;
    Ok(ui::pack(label))
}

struct State {
    assets: common::Assets,
    gui: ui::Gui<Message>,
    label: Option<ui::RcWidget>,
}

impl State {
    fn new(assets: common::Assets) -> ui::Result<Self> {
        let gui = make_gui(assets.font)?;
        let label = None;
        Ok(Self { assets, gui, label })
    }

    fn remove_label(&mut self) {
        println!("Removing...");
        if let Some(ref label) = self.label.take() {
            self.gui.remove(label);
        }
        println!("Removed.");
    }

    fn add_label(&mut self) {
        println!("Adding...");
        let label = make_label(&self.assets).expect("Can't make a label");
        let anchor = ui::Anchor(ui::HAnchor::Left, ui::VAnchor::Top);
        self.gui.add(&label, anchor);
        self.label = Some(label);
        println!("Added.");
    }

    fn handle_message(&mut self, message: Option<Message>) {
        if let Some(Message::AddOrRemove) = message {
            if self.label.is_some() {
                self.remove_label();
            } else {
                self.add_label();
            }
        }
    }
}

#[mq::main("ZGui: Remove Widget Demo")]
async fn main() {
    let assets = common::Assets::load().await;
    let mut state = State::new(assets).expect("Can't create the game state");
    loop {
        // Update the camera and the GUI.
        let aspect_ratio = common::aspect_ratio();
        let camera = common::make_and_set_camera(aspect_ratio);
        state.gui.resize_if_needed(aspect_ratio);
        // Handle cursor updates.
        let pos = common::get_world_mouse_pos(&camera);
        state.gui.move_mouse(pos);
        if mq::input::is_mouse_button_pressed(mq::input::MouseButton::Left) {
            let message = state.gui.click(pos);
            println!("{:?}", message);
            state.handle_message(message);
        }
        // Draw the GUI.
        mq::window::clear_background(graphics::colors::WHITE);
        state.gui.draw();
        mq::window::next_frame().await;
    }
}
