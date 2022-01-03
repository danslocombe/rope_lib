#[derive(Debug, Copy, Clone, PartialEq)]
pub struct I2 {
    pub x: i32,
    pub y: i32,
}

impl I2 {
  pub fn new(x : i32, y : i32) -> Self {
    Self { x , y }
  }

  pub fn to_v2(self) -> crate::rope::Vec2 {
    crate::rope::Vec2::new(self.x as f32, self.y as f32)
  }
}

impl std::ops::Add for I2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct DenseGrid<T> {
  width : usize,
  rows : Vec<Vec<T>>
}

impl<T> Default for DenseGrid<T> {
  fn default() -> Self {
    Self {
      width: 0,
      rows: vec![],
    }
  }
}

fn expand_vec<T : Clone>(new_size : usize, old : &[T], val : &T) -> Vec<T> {
  let mut new = vec![];
  new.resize(new_size, val.clone());

  // Place old into new rows vec
  let offset = (new_size - old.len()) / 2;
  for (i, row) in old.iter().enumerate() {
    new[i+offset] = row.clone()
  }

  new
}

impl<T : Clone + Default> DenseGrid<T> {
  pub fn max_x(&self) -> i32 {
    (self.width / 2) as i32
  }

  pub fn max_y(&self) -> i32 {
    (self.rows.len() / 2) as i32
  }

  fn expand_x(&mut self) {
    let new_width = if (self.width == 0) {
      8
    }
    else {
      self.width * 2
    };

    //println!("Expanding x to {}", new_width);

    for row in &mut self.rows {
      let new_row = expand_vec(new_width as usize, row, &T::default());
      let _ = std::mem::replace(row, new_row);
    }

    self.width = new_width;
  }

  fn expand_y(&mut self) {
    let new_height = if (self.rows.len() == 0) {
      8
    }
    else {
      self.rows.len() * 2
    };

    //println!("Expanding y to {}", new_height);

    let mut template_empty_row = vec![];
    template_empty_row.resize(self.width, T::default());

    let new_rows = expand_vec(new_height as usize, &self.rows, &template_empty_row);
    let _ = std::mem::replace(&mut self.rows, new_rows);
  }

  fn ensure_contains(&mut self, pos : I2) {
    while (pos.y >= self.max_y() || pos.y <= -self.max_y()) {
      self.expand_y();
    }
    
    while (pos.x >= self.max_x() || pos.x <= -self.max_x()) {
      self.expand_x();
    }
  }

  pub fn get_mut(&mut self, pos : I2) -> &mut T{
    //println!("Getting {:?}", pos);
    self.ensure_contains(pos);

    let y_index = ((self.rows.len() / 2) as i32 + pos.y) as usize;
    let x_index = ((self.width / 2) as i32 + pos.x) as usize;

    &mut self.rows[y_index][x_index]
  }

  pub fn get(&mut self, pos : I2) -> T {
    self.get_mut(pos).clone()
  }
}