const LEARNING_RATE: f32 = 0.3;

/// [`ConvexOptimizer`] is an optimizer than finds a local minimum for an unknown
/// convex function using a gradient descent algorithm.
pub struct ConvexOptimizer {
    x: f32,
    px: f32,
    py: f32,
    threshold: f32
}

impl ConvexOptimizer {
    /// Create a new optimizer with an initial guess of `x` and a threshold of when to stop
    /// optimization based on the change of `x`.
    pub fn new(x: f32, threshold: f32) -> Self {
        Self {
            x,
            px: 0.0,
            py: 0.0,
            threshold,
        }
    }

    /// Step the optimizer forward by one iteration.
    /// Accepts the `y` value for the previous `x` value and yields a new `x` value.
    pub fn step(&mut self, y: f32) -> f32 {
        let mut xd = self.x - self.px;
        if xd.abs() < 1e-6 {
            xd = 1e-6;
        }

        let gradient = (y - self.py) / xd;
        let change = LEARNING_RATE * gradient;
        if change.abs() < self.threshold {
            return self.x;
        }

        self.px = self.x;
        self.py = y;
        self.x -= change;
        self.x
    }
}
