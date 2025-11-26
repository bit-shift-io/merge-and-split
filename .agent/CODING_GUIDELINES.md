# Coding Guidelines

## General Rust Style

Follow standard Rust conventions:
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer explicit types in public APIs
- Use `rustfmt` for formatting

## Project-Specific Patterns

### 1. Vector Types

**DO:**
```rust
use crate::core::math::{Vec2, Vec4};

let position = Vec2::new(1.0, 2.0);
let color = Vec4::RED;  // Use color constants
```

**DON'T:**
```rust
let color = Vec4::new(1.0, 0.0, 0.0, 1.0);  // Prefer Vec4::RED
```

### 2. Constraint Implementation

**DO:**
```rust
// Custom vector wrapper pattern
pub struct MyConstraintVec {
    constraints: Vec<MyConstraint>,
}

impl MyConstraintVec {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }
    
    pub fn solve(&self, particles: &mut [Particle], compliance: f32, dt: f32) {
        for constraint in &self.constraints {
            constraint.project(particles, compliance, dt);
        }
    }
    
    // ... other required methods
}
```

**DON'T:**
```rust
// Avoid trait objects for constraints
pub struct Simulation {
    constraints: Vec<Box<dyn Constraint>>,  // NO - bad for memory coherence
}
```

### 3. Particle Access

**DO:**
```rust
// Direct indexing for known-valid indices
let particle = &mut particles[index];

// Bounds checking when index might be invalid
if let Some(particle) = particles.get_mut(index) {
    // ...
}
```

### 4. Constraint Projection

**Pattern:**
```rust
impl MyConstraint {
    pub fn project(&self, particles: &mut [Particle], compliance: f32, dt: f32) {
        // 1. Get particle references
        let (p1, p2) = get_two_particles_mut(particles, self.particle_a, self.particle_b);
        
        // 2. Calculate constraint violation
        let delta = p2.position - p1.position;
        let current_distance = delta.length();
        let violation = current_distance - self.rest_distance;
        
        // 3. Calculate correction with compliance
        let alpha = compliance / (dt * dt);
        let correction_magnitude = violation / (p1.inv_mass + p2.inv_mass + alpha);
        
        // 4. Apply corrections
        let correction = delta.normalize() * correction_magnitude;
        p1.position += correction * p1.inv_mass;
        p2.position -= correction * p2.inv_mass;
    }
}
```

### 5. Entity Pattern

**DO:**
```rust
pub struct MyEntity {
    particle_indices: Vec<usize>,
    // ... other fields
}

impl MyEntity {
    pub fn new(simulation: &mut Simulation) -> Self {
        let mut particle_indices = Vec::new();
        
        // Create particles
        for _ in 0..num_particles {
            let index = simulation.add_particle(/* ... */);
            particle_indices.push(index);
        }
        
        // Create constraints
        simulation.distance_constraints_mut().push(/* ... */);
        
        Self { particle_indices }
    }
    
    pub fn update(&mut self, simulation: &Simulation, dt: f32) {
        // Update entity state based on particle positions
    }
}
```

## Performance Guidelines

### 1. Avoid Allocations in Hot Paths

**DON'T:**
```rust
// In constraint solving loop
fn solve(&self, particles: &mut [Particle]) {
    for constraint in &self.constraints {
        let temp_vec = Vec::new();  // NO - allocation in hot path
        // ...
    }
}
```

### 2. Use Iterators Efficiently

**DO:**
```rust
// Prefer iterator chains
particles.iter_mut()
    .filter(|p| p.inv_mass > 0.0)
    .for_each(|p| p.apply_gravity(dt));
```

### 3. Minimize Particle Array Lookups

**DO:**
```rust
// Cache particle references when doing multiple operations
let particle = &mut particles[index];
particle.position += velocity * dt;
particle.velocity += acceleration * dt;
```

**DON'T:**
```rust
particles[index].position += velocity * dt;
particles[index].velocity += acceleration * dt;  // Redundant bounds check
```

## Module Organization

### File Naming
- `{concept}.rs` for single types (e.g., `distance_constraint.rs`)
- `mod.rs` for module re-exports
- Group related types in subdirectories

### Module Structure
```rust
// In mod.rs
mod distance_constraint;
mod spring_constraint;

pub use distance_constraint::*;
pub use spring_constraint::*;
```

## Documentation

### Public APIs
```rust
/// Creates a new distance constraint between two particles.
///
/// # Arguments
/// * `particle_a` - Index of the first particle
/// * `particle_b` - Index of the second particle
/// * `rest_distance` - The distance to maintain between particles
///
/// # Example
/// ```
/// let constraint = DistanceConstraint::new(0, 1, 10.0);
/// ```
pub fn new(particle_a: usize, particle_b: usize, rest_distance: f32) -> Self {
    // ...
}
```

### Complex Algorithms
```rust
// Explain the physics/math behind non-obvious code
// PBD constraint projection: C(x) = |x1 - x2| - d = 0
// Correction: Δx = -C(x) / (|∇C|² + α) * ∇C
let correction = violation / (inv_mass_sum + alpha) * gradient;
```

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance_constraint_maintains_distance() {
        // Arrange
        let mut particles = vec![
            Particle::new(Vec2::new(0.0, 0.0), 1.0),
            Particle::new(Vec2::new(10.0, 0.0), 1.0),
        ];
        let constraint = DistanceConstraint::new(0, 1, 5.0);
        
        // Act
        constraint.project(&mut particles, 0.0, 0.016);
        
        // Assert
        let distance = (particles[1].position - particles[0].position).length();
        assert!((distance - 5.0).abs() < 0.01);
    }
}
```

## Common Pitfalls

### 1. Forgetting to Update Constraint Counts
```rust
// REMEMBER: Call update_counts() after adding constraints
simulation.distance_constraints_mut().push(constraint);
// Later, before solving:
simulation.distance_constraints().update_counts(&mut simulation.particles);
```

### 2. Incorrect Particle Indexing
```rust
// CAREFUL: Ensure indices are valid before use
// Entities should track their particle indices
// Don't assume particles are contiguous
```

### 3. Compliance Values
```rust
// compliance = 0.0 → rigid constraint (infinite stiffness)
// compliance > 0.0 → soft constraint (lower stiffness)
// Typical range: 0.0 to 0.01
```

### 4. Time Step Dependency
```rust
// PBD is time-step dependent
// For stability, use fixed time steps or XPBD (future)
const FIXED_DT: f32 = 1.0 / 60.0;  // 60 FPS
```

## Code Review Checklist

- [ ] No `Box<dyn Trait>` for constraints
- [ ] Custom vector wrappers used for constraint collections
- [ ] Color constants used instead of hardcoded `Vec4::new()`
- [ ] Particle indices validated or guaranteed valid
- [ ] No allocations in constraint solving loops
- [ ] Constraint counts updated when adding constraints
- [ ] Public APIs documented
- [ ] Complex physics/math explained with comments
- [ ] Tests added for new constraints
- [ ] `rustfmt` applied
