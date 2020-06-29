pub enum VerticalAnchor {
    Top,
    Middle,
    Bottom,
}

pub enum HorizontalAnchor {
    Left,
    Middle,
    Right,
}

pub struct Anchor {
    vertical: VerticalAnchor,
    horizontal: HorizontalAnchor,
}

impl Anchor {
    pub fn new(vertical: VerticalAnchor, horizontal: HorizontalAnchor) -> Self {
        Self { vertical, horizontal }
    }
}

