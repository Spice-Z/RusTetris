pub struct Tetrimino {
  pub w: usize,
  pub h: usize,
  pub block: Vec<Vec<bool>>,
}

impl Tetrimino {
  /// ■■■■
  pub fn i_tetrimino() -> Self {
    Self {
      w: 4,
      h: 1,
      block: vec![vec![true, true, true, true]],
    }
  }
  /// ■■
  /// ■■
  pub fn o_tetrimino() -> Self {
    Self {
      w: 2,
      h: 2,
      block: vec![vec![true, true], vec![true, true]],
    }
  }
  ///  ■■
  /// ■■
  pub fn s_tetrimino() -> Self {
    Self {
      w: 3,
      h: 2,
      block: vec![vec![false, true, true], vec![true, true, false]],
    }
  }
  /// ■■
  ///  ■■
  pub fn z_tetrimino() -> Self {
    Self {
      w: 3,
      h: 2,
      block: vec![vec![true, true, false], vec![false, true, true]],
    }
  }
  /// ■
  /// ■■■
  pub fn j_tetrimino() -> Self {
    Self {
      w: 3,
      h: 2,
      block: vec![vec![true, false, false], vec![true, true, true]],
    }
  }
  ///   ■
  /// ■■■
  pub fn l_tetrimino() -> Self {
    Self {
      w: 3,
      h: 2,
      block: vec![vec![false, false, true], vec![true, true, true]],
    }
  }
  ///  ■
  /// ■■■
  pub fn t_tetrimino() -> Self {
    Self {
      w: 3,
      h: 2,
      block: vec![vec![false, true, false], vec![true, true, true]],
    }
  }
}
