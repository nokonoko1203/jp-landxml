use jp_landxml::geometry::{ClothoidCurve, calculate_tin_area};
use jp_landxml::{Point2D};

#[test]
fn test_clothoid_curve_creation() {
    let start_point = Point2D { x: 0.0, y: 0.0 };
    let curve = ClothoidCurve::new(
        start_point,
        0.0, // 開始方向（ラジアン）
        100.0, // 長さ
        50.0, // クロソイドパラメータ
        None, // 開始半径
        Some(100.0), // 終了半径
    );
    
    assert_eq!(curve.start_point.x, 0.0);
    assert_eq!(curve.start_point.y, 0.0);
    assert_eq!(curve.length, 100.0);
    assert_eq!(curve.clothoid_param, 50.0);
}

#[test]
fn test_clothoid_point_calculation() {
    let start_point = Point2D { x: 0.0, y: 0.0 };
    let curve = ClothoidCurve::new(
        start_point,
        0.0,
        100.0,
        50.0,
        None,
        Some(100.0),
    );
    
    let point_at_start = curve.point_at_distance(0.0).expect("Failed to calculate point");
    assert!((point_at_start.x - 0.0).abs() < 0.001);
    assert!((point_at_start.y - 0.0).abs() < 0.001);
    
    let point_at_mid = curve.point_at_distance(50.0).expect("Failed to calculate point");
    assert!(point_at_mid.x > 0.0); // 前進している
    
    // 範囲外のテスト
    let result = curve.point_at_distance(-10.0);
    assert!(result.is_err());
    
    let result = curve.point_at_distance(150.0);
    assert!(result.is_err());
}

#[test]
fn test_tin_area_calculation() {
    // 三角形
    let triangle = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 10.0, y: 0.0 },
        Point2D { x: 5.0, y: 10.0 },
    ];
    
    let area = calculate_tin_area(&triangle);
    assert!((area - 50.0).abs() < 0.001); // 三角形の面積 = 底辺 × 高さ / 2 = 10 × 10 / 2 = 50
    
    // 正方形
    let square = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 10.0, y: 0.0 },
        Point2D { x: 10.0, y: 10.0 },
        Point2D { x: 0.0, y: 10.0 },
    ];
    
    let area = calculate_tin_area(&square);
    assert!((area - 100.0).abs() < 0.001); // 正方形の面積 = 10 × 10 = 100
    
    // 頂点が少ない場合
    let line = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 10.0, y: 0.0 },
    ];
    
    let area = calculate_tin_area(&line);
    assert_eq!(area, 0.0);
}

#[test]
fn test_empty_polygon_area() {
    let empty_polygon: Vec<Point2D> = vec![];
    let area = calculate_tin_area(&empty_polygon);
    assert_eq!(area, 0.0);
}