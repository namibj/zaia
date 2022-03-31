const LEARNING_RATE: f32 = 0.5;

pub struct ConvexOptimizer {
    x: f32,
    px: f32,
    py: f32,
}

impl ConvexOptimizer {
    pub fn new(x: f32) -> Self {
        Self {
            x,
            px: 0.0,
            py: 0.0,
        }
    }

    pub fn step(&mut self, y: f32) -> f32 {
        let xd = self.x - self.px;
        let gradient = (y - self.py) / xd;
        self.px = self.x;
        self.x -= LEARNING_RATE * gradient;
        self.x
    }
}
