use crate::core::math::vec2::Vec2;

#[derive(Clone, Debug)]
pub struct CubicBezierCurve {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl CubicBezierCurve {
    pub fn new(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
        Self { p0, p1, p2, p3 }
    }

    pub fn sample(&self, t: f32) -> Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        self.p0 * mt3 + self.p1 * (3.0 * mt2 * t) + self.p2 * (3.0 * mt * t2) + self.p3 * t3
    }
}

#[derive(Clone, Debug)]
pub struct BezierSpline {
    pub curves: Vec<CubicBezierCurve>,
}

impl BezierSpline {
    pub fn new() -> Self {
        Self { curves: Vec::new() }
    }

    pub fn add_curve(&mut self, curve: CubicBezierCurve) {
        self.curves.push(curve);
    }

    pub fn sample(&self, t: f32) -> Vec2 {
        if self.curves.is_empty() {
            return Vec2::zero();
        }
        let t = t.clamp(0.0, 1.0);
        let num_curves = self.curves.len();
        
        // Handle t=1.0 case explicitly to avoid index out of bounds or precision issues
        if t >= 1.0 {
            return self.curves.last().unwrap().sample(1.0);
        }

        let scaled_t = t * num_curves as f32;
        let index = scaled_t.floor() as usize;
        let local_t = scaled_t - index as f32;
        
        self.curves[index].sample(local_t)
    }
    
    // Helper to sample points at a fixed resolution
    pub fn get_points(&self, resolution: usize) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(resolution + 1);
        for i in 0..=resolution {
            let t = i as f32 / resolution as f32;
            points.push(self.sample(t));
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_curve_endpoints() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(1.0, 2.0);
        let p2 = Vec2::new(3.0, 2.0);
        let p3 = Vec2::new(4.0, 0.0);
        
        let curve = CubicBezierCurve::new(p0, p1, p2, p3);
        
        // At t=0, should return p0
        let start = curve.sample(0.0);
        assert!((start.x - p0.x).abs() < 0.001);
        assert!((start.y - p0.y).abs() < 0.001);
        
        // At t=1, should return p3
        let end = curve.sample(1.0);
        assert!((end.x - p3.x).abs() < 0.001);
        assert!((end.y - p3.y).abs() < 0.001);
    }

    #[test]
    fn test_bezier_spline_single_curve() {
        let curve = CubicBezierCurve::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(3.0, 0.0)
        );
        
        let mut spline = BezierSpline::new();
        spline.add_curve(curve);
        
        // Test endpoints
        let start = spline.sample(0.0);
        assert!((start.x - 0.0).abs() < 0.001);
        
        let end = spline.sample(1.0);
        assert!((end.x - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_bezier_spline_multiple_curves() {
        let curve1 = CubicBezierCurve::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(3.0, 0.0)
        );
        
        let curve2 = CubicBezierCurve::new(
            Vec2::new(3.0, 0.0),
            Vec2::new(4.0, -1.0),
            Vec2::new(5.0, -1.0),
            Vec2::new(6.0, 0.0)
        );
        
        let mut spline = BezierSpline::new();
        spline.add_curve(curve1);
        spline.add_curve(curve2);
        
        // t=0.5 should be at the junction between curves
        let mid = spline.sample(0.5);
        assert!((mid.x - 3.0).abs() < 0.001);
        assert!((mid.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_get_points() {
        let curve = CubicBezierCurve::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(3.0, 0.0)
        );
        
        let mut spline = BezierSpline::new();
        spline.add_curve(curve);
        
        let points = spline.get_points(10);
        assert_eq!(points.len(), 11); // 0..=10 inclusive
        
        // First and last points should match endpoints
        assert!((points[0].x - 0.0).abs() < 0.001);
        assert!((points[10].x - 3.0).abs() < 0.001);
    }
}
