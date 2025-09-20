use cgmath::InnerSpace;

pub(crate) type Vec2 = cgmath::Vector2<f32>;

pub fn reflect_vector_a_around_b(a: Vec2, b: Vec2) -> Vec2 {
    // 1. Find a vector perpendicular to b
    let b_perp = Vec2::new(-b.y, b.x); // Perpendicular vector

    // Handle case where b or b_perp are zero vectors
    if b_perp.magnitude2() == 0.0 || b.magnitude2() == 0.0 {
        return a; // Or handle as an error
    }

    // 2. Calculate the projection of a onto b_perp
    // projection = (a dot b_perp / b_perp dot b_perp) * b_perp
    let dot_a_b_perp = a.dot(b_perp);
    let dot_b_perp_b_perp = b_perp.dot(b_perp);
    let projection = b_perp * (dot_a_b_perp / dot_b_perp_b_perp);

    // 3. Apply the reflection formula: a_reflected = 2 * projection - a
    let a_reflected = 2.0 * projection - a;

    a_reflected
}