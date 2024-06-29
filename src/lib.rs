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

}

#[cfg(test)]
mod tests {
    use crate::Snake;

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
}
