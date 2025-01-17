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

#[allow(dead_code)]
#[derive(Clone)]
enum PieceColor {
    Black,
    White,
}

#[allow(dead_code)]
#[derive(Clone)]
enum PieceKind {
    King,
    Queen,
    Knight,
    Rook,
    Bishop,
    Pawn,
}

#[derive(Clone)]
struct Piece {
    color: PieceColor,
    kind: PieceKind,
}

impl Piece {
    fn new(color: PieceColor, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    fn svg(&self) -> svg::Handle {
        let path = match (&self.color, &self.kind) {
            (PieceColor::White, PieceKind::King) => "sprites/Chess_klt45.svg",
            (PieceColor::White, PieceKind::Queen) => "sprites/Chess_qlt45.svg",
            (PieceColor::White, PieceKind::Knight) => "sprites/Chess_nlt45.svg",
            (PieceColor::White, PieceKind::Bishop) => "sprites/Chess_blt45.svg",
            (PieceColor::White, PieceKind::Rook) => "sprites/Chess_rlt45.svg",
            (PieceColor::White, PieceKind::Pawn) => "sprites/Chess_plt45.svg",
            (PieceColor::Black, PieceKind::King) => "sprites/Chess_kdt45.svg",
            (PieceColor::Black, PieceKind::Queen) => "sprites/Chess_qdt45.svg",
            (PieceColor::Black, PieceKind::Knight) => "sprites/Chess_ndt45.svg",
            (PieceColor::Black, PieceKind::Bishop) => "sprites/Chess_bdt45.svg",
            (PieceColor::Black, PieceKind::Rook) => "sprites/Chess_rdt45.svg",
            (PieceColor::Black, PieceKind::Pawn) => "sprites/Chess_pdt45.svg",
        };
        svg::Handle::from_path(path)
    }
}

struct BoardState {
    pieces: Vec<Vec<Option<Piece>>>,
}

impl BoardState {
    fn svg(&self, x: usize, y: usize) -> Option<svg::Handle> {
        Some(self.pieces[x][y].clone()?.svg())
    }
}

impl Default for BoardState {
    fn default() -> Self {
        let mut pieces = vec![vec![None; 8]; 8];

        pieces[0][0] = Some(Piece::new(PieceColor::Black, PieceKind::Rook));
        pieces[0][7] = Some(Piece::new(PieceColor::White, PieceKind::Rook));
        pieces[7][0] = Some(Piece::new(PieceColor::Black, PieceKind::Rook));
        pieces[7][7] = Some(Piece::new(PieceColor::White, PieceKind::Rook));

        pieces[1][0] = Some(Piece::new(PieceColor::Black, PieceKind::Knight));
        pieces[1][7] = Some(Piece::new(PieceColor::White, PieceKind::Knight));
        pieces[6][0] = Some(Piece::new(PieceColor::Black, PieceKind::Knight));
        pieces[6][7] = Some(Piece::new(PieceColor::White, PieceKind::Knight));

        pieces[2][0] = Some(Piece::new(PieceColor::Black, PieceKind::Bishop));
        pieces[2][7] = Some(Piece::new(PieceColor::White, PieceKind::Bishop));
        pieces[5][0] = Some(Piece::new(PieceColor::Black, PieceKind::Bishop));
        pieces[5][7] = Some(Piece::new(PieceColor::White, PieceKind::Bishop));

        pieces[3][0] = Some(Piece::new(PieceColor::Black, PieceKind::Queen));
        pieces[3][7] = Some(Piece::new(PieceColor::White, PieceKind::Queen));

        pieces[4][0] = Some(Piece::new(PieceColor::Black, PieceKind::King));
        pieces[4][7] = Some(Piece::new(PieceColor::White, PieceKind::King));

        pieces.iter_mut().for_each(|col| {
            col[1] = Some(Piece::new(PieceColor::Black, PieceKind::Pawn));
            col[6] = Some(Piece::new(PieceColor::White, PieceKind::Pawn));
        });

        Self { pieces }
    }
}

#[derive(Default)]
struct Board {
    cache: Cache,
}

impl Program<Message> for Board {
    type State = BoardState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            let square_size = bounds.size().width / 8.0;
            for x in 0..8 {
                for y in 0..8 {
                    let loc = Point::new(x as f32 * square_size, y as f32 * square_size);
                    let size = Size::new(square_size, square_size);
                    let rectangle_path = Path::rectangle(loc, size);
                    let rectangle = Rectangle::new(loc, size);
                    let color = if (x + y) % 2 == 0 {
                        Color::from_rgb8(157, 172, 255)
                    } else {
                        Color::from_rgb8(111, 115, 210)
                    };
                    frame.fill(&rectangle_path, color);
                    if let Some(svg_handle) = state.svg(x, y) {
                        frame.draw_svg(rectangle, &svg_handle);
                    }
                }
            }
        });

        vec![geom]
    }
}
