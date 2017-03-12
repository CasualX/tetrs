/*!
Tetris game engine.
*/

extern crate rand;

mod bot;
pub use self::bot::{Weights, PlayI, Play};

mod bag;
pub use self::bag::{Bag, OfficialBag, BestBag, WorstBag};

mod pt;
pub use self::pt::Point;

mod piece;
pub use self::piece::{Mesh, Piece};

mod rot;
pub use self::rot::Rot;

mod player;
pub use self::player::Player;

mod well;
pub use self::well::{Well, Line, ParseWellError, MAX_WIDTH, MAX_HEIGHT};

mod tile;
pub use self::tile::{Tile, TileTy, TILE_BG0, TILE_BG1, TILE_BG2};

mod scene;
pub use self::scene::{Scene};

mod state;
pub use self::state::{State};
