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
    /// A snake with a new segment
    ///
    /// # Panics
    ///
    /// Panics if the apple_pos is equal to the head
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
    /// Returns a new snake after the move
    pub fn shift(&self, dir: Direction) -> Snake {
        let segments = self
            .segments
            .iter()
            .enumerate()
            .map(|(i, seg)| {
                // The head is the only thing that gets a new position, everything else takes an old
                // position
                if i == 0 {
                    return match dir {
                        Direction::Up => (seg.0, seg.1 - 1),
                        Direction::Down => (seg.0, seg.1 + 1),
                        Direction::Left => (seg.0 - 1, seg.1),
                        Direction::Right => (seg.0 + 1, seg.1),
                    };
                }

                // Getting the position of the previous segment
                self.segments[i - 1]
            })
            .collect();

        Snake { segments }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Snake, Direction};

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
            (3, 4)
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
            (3, 3)
        ];

        assert_eq!(snake.shift(Direction::Right).segments, wanted);
    }
}
