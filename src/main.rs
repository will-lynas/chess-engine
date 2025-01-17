use iced::{
    mouse,
    widget::{
        canvas::{self, Geometry},
        column, Canvas,
    },
    Alignment::Center,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme,
};

fn main() -> iced::Result {
    iced::application("Chess Game", App::update, App::view).run()
}

#[derive(Default)]
struct App {
    board: Board,
}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<'_, Message> {
        column![Canvas::new(&self.board).width(Fill).height(Fill),]
            .align_x(Center)
            .into()
    }
}

#[derive(Debug)]
struct Message;

#[derive(Default)]
struct Board {
    cache: canvas::Cache,
}

impl canvas::Program<Message> for Board {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            let square_size = bounds.width.min(bounds.height) * 0.5;
            let top_left = Point {
                x: (bounds.width - square_size) / 2.0,
                y: (bounds.height - square_size) / 2.0,
            };
            let rectangle = canvas::Path::rectangle(top_left, Size::new(square_size, square_size));

            frame.fill(&rectangle, iced::Color::BLACK);
        });

        vec![geom]
    }
}
