use iced::{
    application,
    mouse::Cursor,
    widget::{
        canvas::{Cache, Geometry, Path, Program},
        column, Canvas,
    },
    Alignment::Center,
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme,
};

fn main() -> iced::Result {
    application("Chess Game", App::update, App::view).run()
}

#[derive(Default)]
struct App {
    board: Board,
}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        column![Canvas::new(&self.board).width(Fill).height(Fill),]
            .align_x(Center)
            .into()
    }
}

#[derive(Debug)]
struct Message;

#[derive(Default)]
struct Board {
    cache: Cache,
}

impl Program<Message> for Board {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            let square_size = bounds.width.min(bounds.height) * 0.5;
            let top_left = Point {
                x: (bounds.width - square_size) / 2.0,
                y: (bounds.height - square_size) / 2.0,
            };
            let rectangle = Path::rectangle(top_left, Size::new(square_size, square_size));

            frame.fill(&rectangle, Color::BLACK);
        });

        vec![geom]
    }
}
