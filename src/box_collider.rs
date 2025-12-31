use glam::Vec2;

pub struct Collider {
    pub tl: Vec2,
    pub tr: Vec2,
    pub bl: Vec2,
    pub br: Vec2,
}

impl Collider {
    pub fn new(size: (f32, f32)) -> Self {
        Self {
            tl: Vec2::new(-size.0,  size.1),
            tr: Vec2::new( size.0,  size.1),
            bl: Vec2::new(-size.0, -size.1),
            br: Vec2::new( size.0, -size.1)
        }
    }

    pub fn collide(&self, other: &Collider) -> bool {
        if self.tr.x >= other.tl.x &&
            self.tl.x <= other.tr.x &&
            self.tl.y >= other.bl.y &&
            self.br.y <= other.tr.y {
            return true;
        };
        false
    }

    pub fn update(&mut self, position: Vec2) {
        let t = glam::Mat3::from_translation(position);
        self.tl = (t * self.tl.extend(1.0)).truncate();
        self.tr = (t * self.tr.extend(1.0)).truncate();
        self.bl = (t * self.bl.extend(1.0)).truncate();
        self.br = (t * self.br.extend(1.0)).truncate();
    }
}