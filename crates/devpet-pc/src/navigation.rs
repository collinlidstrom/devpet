//! Pure pointer geometry shared by rendering and input.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Target {
    pub fn contains(self, point: (f32, f32)) -> bool {
        point.0 >= self.x
            && point.0 < self.x + self.w
            && point.1 >= self.y
            && point.1 < self.y + self.h
    }
}

pub const BACK: Target = Target {
    x: 3.,
    y: 1.,
    w: 110.,
    h: 12.,
};
pub const PREVIOUS: Target = Target {
    x: 5.,
    y: 45.,
    w: 25.,
    h: 25.,
};
pub const NEXT: Target = Target {
    x: 130.,
    y: 45.,
    w: 25.,
    h: 25.,
};

pub fn home(index: usize) -> Target {
    Target {
        x: 3. + index as f32 * 39.,
        y: 96.,
        w: 36.,
        h: 13.,
    }
}

pub fn row(baseline: f32) -> Target {
    Target {
        x: 10.,
        y: baseline - 8.,
        w: 140.,
        h: 10.,
    }
}

/// Reject letterbox clicks and use the same integer viewport as the renderer.
pub fn logical_pointer(width: f32, height: f32, mouse: (f32, f32)) -> Option<(f32, f32)> {
    let scale = (width / 160.).min(height / 144.).floor().max(1.);
    let point = (
        (mouse.0 - (width - 160. * scale) / 2.) / scale,
        (mouse.1 - (height - 144. * scale) / 2.) / scale,
    );
    Target {
        x: 0.,
        y: 0.,
        w: 160.,
        h: 144.,
    }
    .contains(point)
    .then_some(point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_maps_at_multiple_scales_and_letterboxes() {
        for scale in [1., 2., 4., 5.] {
            assert_eq!(
                logical_pointer(160. * scale, 144. * scale, (80. * scale, 72. * scale)),
                Some((80., 72.))
            );
        }
        assert_eq!(logical_pointer(800., 600., (80., 12.)), Some((0., 0.)));
        assert_eq!(logical_pointer(800., 600., (79., 100.)), None);
        assert_eq!(logical_pointer(800., 600., (720., 100.)), None);
    }

    #[test]
    fn every_home_button_is_hit_at_its_center() {
        for index in 0..4 {
            let target = home(index);
            let point = (target.x + target.w / 2., target.y + target.h / 2.);
            assert!(target.contains(point));
            assert!((0..4).filter(|other| home(*other).contains(point)).count() == 1);
        }
    }

    #[test]
    fn adjacent_rows_do_not_overlap() {
        assert!(!row(99.).contains((20., 101.)));
        assert!(row(109.).contains((20., 101.)));
        assert!(!BACK.contains((120., 5.)));
        assert!(!PREVIOUS.contains((140., 55.)));
        assert!(NEXT.contains((140., 55.)));
    }
}
