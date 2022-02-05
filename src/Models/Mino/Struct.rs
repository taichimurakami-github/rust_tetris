mod Common;
use Common::Coodinate;

mod Mino {
  pub struct Mino {
    shape: [[Coodinate; 2]; 4],
    color: String,
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
    color: "PURPLE",
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
    color: "YELLOW",
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
    color: "LIGHTBLUE",
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
    color: "RED",
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
    color: "GREEN",
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
    color: "ORANGE",
  };

  /*
   * [1,0,0,0],
   * [1,1,1,0],
   * [0,0,0,0],
   * [0,0,0,0],
   */
  pub static J: Mino = Mino {
    shape: [
      Coodinate { x: 0, y: 0 },
      Coodinate { x: 1, y: 0 },
      Coodinate { x: 1, y: 1 },
      Coodinate { x: 1, y: 2 },
    ],
    color: "BLUE",
  };

  pub enum Dir {
    North,
    East,
    South,
    West,
  }
}

pub struct ControlledMino {
  pos: Coodinate,
  mino: &'static Mino,
  dir: Dir,
}
