use iced::{
    application,
    mouse::Cursor,
    widget::{
        canvas::{Cache, Geometry, Path, Program},
        center, responsive, svg, Canvas,
    },
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
        responsive(|size| {
            let min_len = size.width.min(size.height);
            let board_size = min_len * 0.8;
            center(
                Canvas::new(&self.board)
                    .width(board_size)
                    .height(board_size),
            )
            .width(Fill)
            .height(Fill)
            .into()
        })
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
            let square_size = bounds.size().width / 8.0;
            for row in 0..8 {
                for col in 0..8 {
                    let loc = Point::new(row as f32 * square_size, col as f32 * square_size);
                    let size = Size::new(square_size, square_size);
                    let rectangle_path = Path::rectangle(loc, size);
                    let rectangle = Rectangle::new(loc, size);
                    let color = if (row + col) % 2 == 0 {
                        Color::from_rgb8(157, 172, 255)
                    } else {
                        Color::from_rgb8(111, 115, 210)
                    };
                    frame.fill(&rectangle_path, color);
                    if let Some(svg_handle) = get_svg((row, col)) {
                        frame.draw_svg(rectangle, &svg_handle);
                    }
                }
            }
        });

        vec![geom]
    }
}

fn get_svg((x, y): (usize, usize)) -> Option<svg::Handle> {
    let path = match (x, y) {
        // Pawns
        (_, 1) => Some("sprites/Chess_pdt45.svg"),
        (_, 6) => Some("sprites/Chess_plt45.svg"),
        // Rooks
        (0, 0) | (7, 0) => Some("sprites/Chess_rdt45.svg"),
        (0, 7) | (7, 7) => Some("sprites/Chess_rlt45.svg"),
        // Knights
        (1, 0) | (6, 0) => Some("sprites/Chess_ndt45.svg"),
        (1, 7) | (6, 7) => Some("sprites/Chess_nlt45.svg"),
        // Bishops
        (2, 0) | (5, 0) => Some("sprites/Chess_bdt45.svg"),
        (2, 7) | (5, 7) => Some("sprites/Chess_blt45.svg"),
        // Kings
        (4, 0) => Some("sprites/Chess_kdt45.svg"),
        (4, 7) => Some("sprites/Chess_klt45.svg"),
        // Queens
        (3, 0) => Some("sprites/Chess_qdt45.svg"),
        (3, 7) => Some("sprites/Chess_qlt45.svg"),
        _ => None,
    }?;
    Some(svg::Handle::from_path(path))
}
