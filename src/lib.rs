#[derive(Clone, Copy, Debug)]
/// An enum representing the directions the snake can move in
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snake {
    segments: Vec<(u8, u8)>,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            segments: vec![(0, 0)],
        }
    }
}

impl Snake {
    /// Adds a new segment to the snake. Used after the snake eats an apple
    ///
    /// # Parameters
    ///
    /// - `apple_pos` The position of the apple, which becomes the new head
    ///
    /// # Returns
    ///
    /// A [Snake] with a new segment
    ///
    /// # Panics
    ///
    /// Panics if `apple_pos` is equal to the head
    pub fn add_segment(&self, apple_pos: (u8, u8)) -> Snake {
        assert_ne!(apple_pos, self.head());

        let mut segments = self.segments.clone();

        // The apple becomes the new head
        segments.insert(0, apple_pos);

        Snake { segments }
    }

    /// Gets the location of the snakes head
    ///
    /// # Returns
    ///
    /// A tuple in the format (x, y)
    pub fn head(&self) -> (u8, u8) {
        self.segments[0]
    }

    /// Moves the snake in the given direction. Used every tick of the game to move the snake
    ///
    /// # Parameters
    ///
    /// - `dir` The direction to shift the snake's head in
    ///
    /// # Returns
    ///
    /// Returns a new [Snake] after the move
    pub fn shift(&self, dir: Direction) -> Snake {
        // The only segments affected by the shift is the first head and the end tail segment:
        // We remove the end tail segement, then create a new head at the postion of the current,
        // then move it and append it at the start as the new head
        let mut segments = self.segments.clone();

        segments.remove(segments.len() - 1);

        let cur_head = self.head();
        let new_head = match dir {
            Direction::Up => (cur_head.0, cur_head.1 - 1),
            Direction::Down => (cur_head.0, cur_head.1 + 1),
            Direction::Left => (cur_head.0 - 1, cur_head.1),
            Direction::Right => (cur_head.0 + 1, cur_head.1),
        };

        segments.insert(0, new_head);

        Snake { segments }
    }
}

/// Gets the difference between two snakes. Used for figuring out what tiles to remove from the
/// render grid and which ones to add
///
/// # Parameters
///
/// - `old` The snake that is not being rendered
/// - `new` The snake that is being rendered
///
/// # Returns
///
/// A tuple of vecs. The first vec is tiles to remove, and the second is tiles to add
pub fn diff(old: Snake, new: Snake) -> (Vec<(u8, u8)>, Vec<(u8, u8)>) {
    let old_tiles = old
        .segments
        .iter()
        .filter(|x| !new.segments.contains(x))
        .map(|x| x.to_owned())
        .collect();
    let new_tiles = new
        .segments
        .iter()
        .filter(|x| !old.segments.contains(x))
        .map(|x| x.to_owned())
        .collect();

    (old_tiles, new_tiles)
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Snake, diff};

    #[test]
    fn test_add_segment() {
        let snake = Snake::default();

        let apple_pos = (0, 1);

        let wanted = vec![(0, 1), (0, 0)];

        assert_eq!(snake.add_segment(apple_pos).segments, wanted)
    }

    #[test]
    #[should_panic]
    fn test_add_segment_head_is_apple() {
        let snake = Snake::default();

        let apple_pos = (0, 0);

        snake.add_segment(apple_pos);
    }

    #[test]
    /// Testing the logic behind shifting the snake
    fn test_shift() {
        // s
        // s
        // s s s s
        //       s
        //       s
        let segments = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
            (3, 4),
        ];

        let snake = Snake { segments };

        // s s
        // s
        // s s s s
        //       s
        //
        let wanted = vec![
            (1, 0),
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
        ];

        assert_eq!(snake.shift(Direction::Right).segments, wanted);
    }

    #[test]
    fn test_diff() {
        // s
        // s
        // s s s s
        //       s
        //       s
        let old_snake = Snake { segments: vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
            (3, 4),
        ] };

        // s s
        // s
        // s s s s
        //       s
        //
        let new_snake = Snake { segments: vec![
            (1, 0),
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
        ] };

        let (old, new) = diff(old_snake, new_snake);

        assert_eq!(old, vec![(3,4)]);
        assert_eq!(new, vec![(1,0)]);
    }
}
