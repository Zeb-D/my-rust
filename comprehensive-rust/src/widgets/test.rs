#[cfg(test)]
mod tests {
    use crate::widgets::button::Button;
    use crate::widgets::label::Label;
    use crate::widgets::widgets::Widget;
    use crate::widgets::window::Window;


    #[test]
    fn it_works() {
        let mut window = Window::new("Rust GUI Demo 1.23");
        window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
        window.add_widget(Box::new(Button::new("Click me!")));
        window.draw();
    }
}
