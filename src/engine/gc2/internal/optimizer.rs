const LEARNING_RATE: f32 = 0.3;
const MOMENTUM: f32 = 0.3;

/// [`ConvexOptimizer`] is an optimizer than finds a local minimum for an
/// unknown convex function using an adaptive gradient descent algorithm.
pub struct ConvexOptimizer {
    x: f32,
    prev_x: f32,
    prev_y: f32,
    prev_change: f32,
    threshold: f32,
}

impl ConvexOptimizer {
    /// Create a new optimizer with an initial guess of `x` and a threshold of
    /// when to stop optimization based on the change of `x`.
    pub fn new(x: f32, threshold: f32) -> Self {
        Self {
            x,
            prev_x: 0.0,
            prev_y: 0.0,
            prev_change: 0.0,
            threshold,
        }
    }

    /// Return the last recommended value of `x`.
    pub fn x(&self) -> f32 {
        self.x
    }

    fn x_diff(&self) -> f32 {
        self.x - self.prev_x + 1e-8
    }

    fn gradient(&self, y: f32) -> f32 {
        (y - self.prev_y) / self.x_diff()
    }

    fn change(&self, y: f32) -> f32 {
        LEARNING_RATE * self.gradient(y)
    }

    fn accelerated_change(&self, y: f32) -> f32 {
        self.change(y) + MOMENTUM * self.prev_change
    }

    /// Step the optimizer forward by one iteration.
    /// Accepts the `y` value for the previous `x` value and yields a new `x`-value.
    pub fn step(&mut self, y: f32) -> f32 {
        // Early-exit if the change is too small.
        if self.change(y) < self.threshold {
            return self.x;
        }

        // Accelerate the change by the momentum and calculate the new x-value.
        let change = self.accelerated_change(y);
        let new_x = self.x - change;

        // Update state
        self.prev_x = self.x;
        self.x = new_x;
        self.prev_y = y;
        self.prev_change = change;

        new_x
    }
}
