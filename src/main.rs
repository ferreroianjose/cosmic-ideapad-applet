mod ideapad_laptop;
mod window;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<crate::window::Window>(())
}
