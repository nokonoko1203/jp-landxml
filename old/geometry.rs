use crate::error::{LandXMLError, Result};
use crate::models::Point2D;

pub struct ClothoidCurve {
    pub start_point: Point2D,
    pub start_direction: f64,
    pub length: f64,
    pub clothoid_param: f64,
    pub radius_start: Option<f64>,
    pub radius_end: Option<f64>,
}

impl ClothoidCurve {
    pub fn new(
        start_point: Point2D,
        start_direction: f64,
        length: f64,
        clothoid_param: f64,
        radius_start: Option<f64>,
        radius_end: Option<f64>,
    ) -> Self {
        Self {
            start_point,
            start_direction,
            length,
            clothoid_param,
            radius_start,
            radius_end,
        }
    }

    pub fn point_at_distance(&self, distance: f64) -> Result<Point2D> {
        if distance < 0.0 || distance > self.length {
            return Err(LandXMLError::GeometryError {
                message: format!("Distance {} is out of range [0, {}]", distance, self.length),
            });
        }

        // クロソイド曲線の簡易計算
        let t = distance / self.clothoid_param;
        let x = distance * (1.0 - t * t / 10.0 + t * t * t * t / 216.0);
        let y = distance * (t / 3.0 - t * t * t / 42.0);

        let cos_dir = self.start_direction.cos();
        let sin_dir = self.start_direction.sin();

        Ok(Point2D {
            x: self.start_point.x + x * cos_dir - y * sin_dir,
            y: self.start_point.y + x * sin_dir + y * cos_dir,
        })
    }
}

pub fn calculate_tin_area(points: &[Point2D]) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    let n = points.len();

    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].x * points[j].y;
        area -= points[j].x * points[i].y;
    }

    area.abs() / 2.0
}
