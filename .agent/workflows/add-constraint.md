---
description: Add a new constraint type to the simulation
---

# Add New Constraint Workflow

Follow this pattern when adding a new constraint type to the physics simulation.

## Steps

1. **Create the constraint struct** in `src/simulation/constraints/`
   - Define a struct with particle indices and constraint parameters
   - Example fields: `particle_a: usize`, `particle_b: usize`, constraint-specific params

2. **Implement the `project()` method**
   - This method applies the constraint to particles
   - Signature: `pub fn project(&self, particles: &mut [Particle], compliance: f32, dt: f32)`
   - Calculate constraint violation and apply corrections to particle positions

3. **Create a custom vector wrapper**
   - Name pattern: `{ConstraintName}Vec` (e.g., `VolumeConstraintVec`)
   - Implement required methods:
     - `new()` - constructor
     - `update_counts(particles: &mut [Particle])` - update particle constraint counts
     - `solve(particles: &mut [Particle], compliance: f32, dt: f32)` - solve all constraints
     - `push(constraint: T)` - add a constraint
     - `clear()` - remove all constraints
     - `len()` - get count
     - `iter_mut()` - mutable iterator

4. **Add to Simulation struct** in `src/simulation/simulation.rs`
   - Add field: `{constraint_name}s: {ConstraintName}Vec`
   - Initialize in `new()`: `{constraint_name}s: {ConstraintName}Vec::new()`

5. **Update Simulation methods**
   - In `solve_constraints()`: call `self.{constraint_name}s.solve(&mut self.particles, compliance, dt)`
   - In `clear_constraints()`: call `self.{constraint_name}s.clear()`
   - Add getter if needed: `pub fn {constraint_name}s_mut(&mut self) -> &mut {ConstraintName}Vec`

6. **Update emitters if needed**
   - If emitters create these constraints, update:
     - `src/simulation/emitters/fluid_emitter.rs`
     - `src/simulation/emitters/open_smoke_emitter.rs`

7. **Test the constraint**
   - Build and run the project
   - Verify the constraint behaves as expected
   - Check performance impact

## Example Reference

See existing constraints:
- `src/simulation/constraints/distance_constraint.rs` - simple pairwise constraint
- `src/simulation/constraints/spring_constraint.rs` - spring mechanics
- `src/simulation/constraints/volume_constraint.rs` - multi-particle constraint

## Notes

- Avoid `Box<dyn Trait>` for memory coherence
- Custom vector wrappers provide better performance than generic collections
- Constraints are solved iteratively in the PBD solver
