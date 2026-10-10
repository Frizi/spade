use crate::HasPosition;

impl HasPosition for glam::Vec2 {
    type Scalar = f32;

    fn position(&self) -> crate::Point2<Self::Scalar> {
        (*self).into()
    }
}

impl From<glam::Vec2> for crate::Point2<f32> {
    fn from(value: glam::Vec2) -> Self {
        crate::Point2::new(value.x, value.y)
    }
}

impl From<crate::Point2<f32>> for glam::Vec2 {
    fn from(value: crate::Point2<f32>) -> Self {
        glam::Vec2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl HasPosition for glam::DVec2 {
    type Scalar = f64;

    fn position(&self) -> crate::Point2<Self::Scalar> {
        (*self).into()
    }
}

impl From<glam::DVec2> for crate::Point2<f64> {
    fn from(value: glam::DVec2) -> Self {
        crate::Point2::new(value.x, value.y)
    }
}

impl From<crate::Point2<f64>> for glam::DVec2 {
    fn from(value: crate::Point2<f64>) -> Self {
        glam::DVec2 {
            x: value.x,
            y: value.y,
        }
    }
}
