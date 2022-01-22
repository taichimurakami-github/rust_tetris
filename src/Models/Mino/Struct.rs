struct Mino {
  shape: [[u8; 2]; 4],
  color: String,
}

/**
 * [0,1,0,0],
 * [1,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static T: Mino = Mino {
  shape: [[0, 1], [1, 0], [1, 1], [1, 2]],
  color: "PURPLE",
};

/**
 * [0,1,1,0],
 * [0,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static O: Mino = Mino {
  shape: [[0, 1], [0, 2], [1, 1], [1, 2]],
  color: "YELLOW",
};

/**
 * [0,0,0,0],
 * [1,1,1,1],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static I: Mino = Mino {
  shape: [[1, 0], [1, 1], [1, 2], [1, 3]],
  color: "LIGHTBLUE",
};

/**
 * [1,1,0,0],
 * [0,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static Z: Mino = Mino {
  shape: [[0, 0], [0, 1], [1, 1], [1, 2]],
  color: "RED",
};

/**
 * [0,1,1,0],
 * [1,1,0,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static S: Mino = Mino {
  shape: [[0, 1], [0, 2], [1, 0], [1, 1]],
  color: "GREEN",
};

/**
 * [0,0,1,0],
 * [1,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static L: Mino = Mino {
  shape: [[0, 2], [1, 0], [1, 1], [1, 2]],
  color: "ORANGE",
};

/*
 * [1,0,0,0],
 * [1,1,1,0],
 * [0,0,0,0],
 * [0,0,0,0],
 */
static J: Mino = Mino {
  shape: [[0, 0], [1, 0], [1, 1], [1, 2]],
  color: "BLUE",
};

enum Dir {
  North,
  East,
  South,
  West,
}

struct ControlledMino {
  pos: (i32, i32),
  mino: &'static Mino,
  dir: Dir,
}
