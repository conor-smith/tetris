pub enum Block {
    Empty,
    Teal,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red
}

pub enum Tetromino {
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock
}

pub struct Coordinate {
    x: i8,
    y: i8
}

pub trait TetrisGame {
    fn get_play_field(&self) -> &[&[Block]];
    fn get_length(&self) -> &u8;
    fn get_height(&self) -> &u8;
    fn get_score(&self) -> &u64;
    fn get_level(&self) -> &u16;
    fn is_game_over(&self) -> &bool;

    fn get_active_tetromino(&self) -> &[Coordinate; 4];
    fn get_saved_tetromino(&self) -> &Option<Tetromino>;
    fn get_next_tetrominoes(&self) -> &[Tetromino; 3];

    fn update(&self) -> GameUpdate;
    fn move_left(&self);
    fn move_right(&self);
    fn move_down(&self);
    fn rotate_left(&self);
    fn rotate_right(&self);
    fn place(&self);
    fn save(&self);
}

pub trait Renderer {
    fn render_game<T: TetrisGame>(&self, game: T, update: GameUpdate);
}

pub enum GameUpdate<'a> {
    NoUpdate,
    GameOver,
    // One a tetromino is placed
    // One frame passes until a new active tetromino appears
    // The slice contains all rows that will be cleared upon call to update()
    // Will have length 0 if no rows are cleared
    TetrominoPlaced(&'a [u8]),
    // Returning this state allows us to play a sound if this event occurs
    UserActionFailed
}

pub enum Rotation {
    NotRotated,
    OneClockwise,
    TwoClockwise,
    ThreeClockwise
}

impl Rotation {
    pub fn clockwise(&self) -> Rotation {
        match self {
            Rotation::NotRotated => Rotation::OneClockwise,
            Rotation::OneClockwise => Rotation::TwoClockwise,
            Rotation::TwoClockwise => Rotation::ThreeClockwise,
            Rotation::ThreeClockwise => Rotation::NotRotated,
        }
    }

    pub fn anti_clockwise(&self) -> Rotation {
        match self {
            Rotation::NotRotated => Rotation::ThreeClockwise,
            Rotation::OneClockwise => Rotation::NotRotated,
            Rotation::TwoClockwise => Rotation::OneClockwise,
            Rotation::ThreeClockwise => Rotation::TwoClockwise,
        }
    }
}

impl Tetromino {
    pub fn get_block_type(&self) -> Block {
        match self {
            Tetromino::IBlock => Block::Teal,
            Tetromino::JBlock => Block::Blue,
            Tetromino::LBlock => Block::Orange,
            Tetromino::OBlock => Block::Yellow,
            Tetromino::SBlock => Block::Green,
            Tetromino::TBlock => Block::Purple,
            Tetromino::ZBlock => Block::Red,
        }
    }

    pub fn get_length(&self) -> u8 {
        match self {
            Tetromino::IBlock => 1,
            Tetromino::JBlock => 2,
            Tetromino::LBlock => 2,
            Tetromino::OBlock => 2,
            Tetromino::SBlock => 3,
            Tetromino::TBlock => 3,
            Tetromino::ZBlock => 3,
        }
    }

    pub fn get_height(&self) -> u8 {
        match self {
            Tetromino::IBlock => 4,
            Tetromino::JBlock => 3,
            Tetromino::LBlock => 3,
            Tetromino::OBlock => 2,
            Tetromino::SBlock => 2,
            Tetromino::TBlock => 2,
            Tetromino::ZBlock => 2,
        }
    }

    // Where x=0 and y=0 are the top left corner
    pub fn get_coordinates(&self) -> [Coordinate; 4] {
        match self {
            Tetromino::IBlock => [Coordinate{x: 0,y: 0}, Coordinate{x: 0,y: 1}, Coordinate{x: 0,y: 2}, Coordinate{x: 0,y: 3}],
            Tetromino::JBlock => [Coordinate{x: 1,y: 0}, Coordinate{x: 1,y: 1}, Coordinate{x: 0,y: 2}, Coordinate{x: 1,y: 2}],
            Tetromino::LBlock => [Coordinate{x: 0,y: 0}, Coordinate{x: 0,y: 1}, Coordinate{x: 0,y: 2}, Coordinate{x: 1,y: 2}],
            Tetromino::OBlock => [Coordinate{x: 0,y: 0}, Coordinate{x: 0,y: 1}, Coordinate{x: 1,y: 0}, Coordinate{x: 1,y: 1}],
            Tetromino::SBlock => [Coordinate{x: 1,y: 0}, Coordinate{x: 2,y: 0}, Coordinate{x: 0,y: 1}, Coordinate{x: 1,y: 1}],
            Tetromino::TBlock => [Coordinate{x: 1,y: 0}, Coordinate{x: 0,y: 1}, Coordinate{x: 1,y: 1}, Coordinate{x: 2,y: 1}],
            Tetromino::ZBlock => [Coordinate{x: 0,y: 0}, Coordinate{x: 1,y: 0}, Coordinate{x: 1,y: 1}, Coordinate{x: 2,y: 1}],
        }
    }

    pub fn get_rotation_point(&self) -> (bool, Coordinate) {
        // If bool = true, rotation point is top left of coordinate. If false, point is in centre
        match self {
            Tetromino::IBlock => (true, Coordinate{x: 0, y: 2}),
            Tetromino::JBlock => (false, Coordinate{x: 1, y: 1}),
            Tetromino::LBlock => (false, Coordinate{x: 0, y: 1}),
            Tetromino::OBlock | Tetromino::SBlock | Tetromino::ZBlock => (true, Coordinate{x: 1, y: 1}),
            Tetromino::TBlock => (false, Coordinate{x: 1, y: 1}),
        }
    }
}