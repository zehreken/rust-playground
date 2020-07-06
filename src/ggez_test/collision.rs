pub struct RectCollider {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub fn check_rect_collision(a: &RectCollider, b: &RectCollider) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}

pub struct CircleCollider {
    x: f32,
    y: f32,
    radius: f32,
}

pub fn check_circle_collision(a: &CircleCollider, b: &CircleCollider) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let distance = (dx * dx + dy * dy).sqrt();

    distance < a.radius + b.radius
}
