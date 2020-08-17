const RADIUS_ANIMATION_SPEED: f64 = 0.3;
const BUBBLE_RADUIS_RANGE: f64 = 40.0;
const BUBBLE_SPEED_RANGE: f64 = 3.0;
const MIN_BUBBLE_RADIUS: f64 = 30.0;
const MIN_BUBBLE_SPEED: f64 = 2.0;


pub struct Bubble {
    pub x: f64,
    pub y: f64,
    pub r1: f64,
    pub r2: f64,
    pub speed: f64,
    pub original_r1: f64,
    pub original_r2: f64,
    pub r1_asc: bool,
    pub r2_asc: bool,
    pub r1_target: f64,
    pub r2_target: f64
}

impl Bubble {
    pub fn new(x: f64, y: f64, r1: f64, r2: f64, speed: f64) -> Bubble {
        let mut bubble = Bubble {
            x,
            y,
            r1: r1.clone() * BUBBLE_RADUIS_RANGE + MIN_BUBBLE_RADIUS,
            r2: r2.clone() * BUBBLE_RADUIS_RANGE + MIN_BUBBLE_RADIUS,
            speed: speed * BUBBLE_SPEED_RANGE + MIN_BUBBLE_SPEED,
            original_r1: r1.clone() * BUBBLE_RADUIS_RANGE + MIN_BUBBLE_RADIUS,
            original_r2: r2.clone() * BUBBLE_RADUIS_RANGE + MIN_BUBBLE_RADIUS,
            r1_asc: false,
            r2_asc: false,
            r1_target: r2 * BUBBLE_RADUIS_RANGE + MIN_BUBBLE_RADIUS,
            r2_target: r1 * BUBBLE_RADUIS_RANGE + MIN_BUBBLE_RADIUS,
        };
        if bubble.r1 < bubble.r2 {
            bubble.r1_asc = true;
        } else {
            bubble.r2_asc = true;
        }
        bubble
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
        if self.y < -self.r1 && self.y < self.r2 {
            self.y = 600.0;
        }

        if !(self.r1 == self.r2 && self.r1_asc == self.r2_asc) {
            let r_dff = (self.original_r1 - self.original_r2).abs();
            if self.r1_asc {
                self.r1 += RADIUS_ANIMATION_SPEED;
                if self.r1 >= self.r1_target {
                    self.r1_target -= r_dff;
                    self.r1_asc = !self.r1_asc;
                }
            } else {
                self.r1 -= RADIUS_ANIMATION_SPEED;
                if self.r1 <= self.r1_target {
                    self.r1_target += r_dff;
                    self.r1_asc = !self.r1_asc;
                }
            }
            if self.r2_asc {
                self.r2 += RADIUS_ANIMATION_SPEED;
                if self.r2 >= self.r2_target {
                    self.r2_target -= r_dff;
                    self.r2_asc = !self.r2_asc;
                }
            } else {
                self.r2 -= RADIUS_ANIMATION_SPEED;
                if self.r2 <= self.r2_target {
                    self.r2_target += r_dff;
                    self.r2_asc = !self.r2_asc;
                }
            }
        }
    }
}