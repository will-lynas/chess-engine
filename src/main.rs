use iced::{
    widget::{button, column, text, Column},
    Alignment::Center,
};

fn main() -> iced::Result {
    iced::run("Counter!", Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(69),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(16)
        .align_x(Center)
    }
}

#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
}
