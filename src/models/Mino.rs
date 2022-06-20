use crate::models::Commons::Coodinate;

use crate::models::Commons::MinoColors as Colors;

pub struct Mino {
    shape: [Coodinate; 4],
    color: Colors,
}

impl Mino {
    fn is_able_to_move() {
        println!("true");
    }
}

/**
 * [0,1,0,0],
 * [1,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
pub static T: Mino = Mino {
    shape: [
        Coodinate { x: 0, y: 1 },
        Coodinate { x: 1, y: 0 },
        Coodinate { x: 1, y: 1 },
        Coodinate { x: 1, y: 2 },
    ],
    color: Colors::Purple,
};

/**
 * [0,1,1,0],
 * [0,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
pub static O: Mino = Mino {
    shape: [
        Coodinate { x: 0, y: 1 },
        Coodinate { x: 0, y: 2 },
        Coodinate { x: 1, y: 1 },
        Coodinate { x: 1, y: 2 },
    ],
    color: Colors::Yellow,
};

/**
 * [0,0,0,0],
 * [1,1,1,1],
 * [0,0,0,0],
 * [0,0,0,0],
 */
pub static I: Mino = Mino {
    shape: [
        Coodinate { x: 1, y: 0 },
        Coodinate { x: 1, y: 1 },
        Coodinate { x: 1, y: 2 },
        Coodinate { x: 1, y: 3 },
    ],
    color: Colors::Lightblue,
};

/**
 * [1,1,0,0],
 * [0,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
pub static Z: Mino = Mino {
    shape: [
        Coodinate { x: 0, y: 0 },
        Coodinate { x: 0, y: 1 },
        Coodinate { x: 1, y: 1 },
        Coodinate { x: 1, y: 2 },
    ],
    color: Colors::Red,
};

/**
 * [0,1,1,0],
 * [1,1,0,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
pub static S: Mino = Mino {
    shape: [
        Coodinate { x: 0, y: 1 },
        Coodinate { x: 0, y: 2 },
        Coodinate { x: 1, y: 0 },
        Coodinate { x: 1, y: 1 },
    ],
    color: Colors::Green,
};

/**
 * [0,0,1,0],
 * [1,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
pub static L: Mino = Mino {
    shape: [
        Coodinate { x: 0, y: 2 },
        Coodinate { x: 1, y: 0 },
        Coodinate { x: 1, y: 1 },
        Coodinate { x: 1, y: 2 },
    ],
    color: Colors::Orange,
};
