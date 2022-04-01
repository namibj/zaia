const LEARNING_RATE: f32 = 0.1;
const MOMENTUM: f32 = 0.8;

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

    /// Calcuate the absolute change in the function input since the last step.
    fn x_diff(&self) -> f32 {
        self.x - self.prev_x
    }

    /// Approximate the derivative using the derivative definition.
    fn gradient(&self, y: f32) -> f32 {
        (y - self.prev_y) / (self.x_diff() + 1e-8)
    }

    /// Calculate an absolute base change based on the approximate gradient.
    fn change(&self, y: f32) -> f32 {
        LEARNING_RATE * self.gradient(y)
    }

    /// Accelerate the base change using previous momentum.
    fn accelerated_change(&self, y: f32) -> f32 {
        self.change(y) + MOMENTUM * self.prev_change
    }

    /// Step the optimizer forward by one iteration.
    /// Accepts the `y` value for the previous `x` value and yields a new `x`-value.
    pub fn step(&mut self, y: f32) -> f32 {
        // Compute the new momentum-accelerated change.
        let change = self.accelerated_change(y);

        // Early-exit if the change is too small.
        if change.abs() < self.threshold {
            return self.x;
        }

        // Update internal state with new data.
        self.prev_x = self.x;
        self.x -= change;
        self.prev_y = y;
        self.prev_change = change;

        self.x
    }
}
