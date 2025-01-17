use iced::{
    application,
    mouse::Cursor,
    widget::{
        canvas::{Cache, Geometry, Path, Program},
        center, Canvas,
    },
    Color, Element, Point, Rectangle, Renderer, Theme,
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
        center(Canvas::new(&self.board).width(500).height(500)).into()
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
            let rectangle = Path::rectangle(Point::ORIGIN, bounds.size());
            frame.fill(&rectangle, Color::BLACK);
        });

        vec![geom]
    }
}
