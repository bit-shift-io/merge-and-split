# Merge & Split - Project Overview

## Project Description

A Rust-based particle physics simulation using Position-Based Dynamics (PBD), with plans to migrate to XPBD and eventually GPU acceleration. The project implements unified particle physics for real-time applications.

## Technology Stack

- **Language**: Rust (Edition 2021)
- **Graphics**: wgpu 26.0.1 (WebGPU API)
- **Windowing**: winit 0.30
- **Math**: cgmath 0.18
- **Random**: rand 0.9.2, rand_pcg, rand_seeder

## Project Structure

```
src/
├── core/          # Core math and utility types (Vec2, Vec4, etc.)
├── engine/        # Rendering engine (wgpu-based renderer, instance rendering)
├── game/          # Game entities and systems (car_entity, entity_system)
└── simulation/    # Physics simulation (particles, constraints, emitters)
```

### Key Modules

#### `core/`
- Mathematical primitives (Vec2, Vec4, etc.)
- Core utility types

#### `engine/`
- Renderer implementation using wgpu
- Instance rendering system
- Graphics pipeline management

#### `game/`
- Entity system for game objects
- Car entity with wheel physics
- Entity lifecycle management

#### `simulation/`
- Particle system implementation
- Constraint types:
  - `DistanceConstraint` - maintains fixed distances
  - `SpringConstraint` - spring mechanics
  - `VolumeConstraint` - prevents volume collapse
  - `BoundaryConstraint` - collision with boundaries
  - `RigidContactConstraint` - rigid body contacts
- Emitters:
  - `FluidEmitter`
  - `OpenSmokeEmitter`
- Shape builders (Circle, etc.)

## Physics Implementation

### Current: Position-Based Dynamics (PBD)
The project currently implements PBD based on the paper "Unified Particle Physics for Real-Time Applications" by Macklin et al.

### Future: XPBD Migration
Plans to migrate to Extended Position-Based Dynamics (XPBD) for improved stability and time-step independence.

### Long-term: GPU Acceleration
Goal to port the particle system to GPU using wgpu compute shaders.

## Common Patterns

### Constraint Pattern
All constraints follow a similar pattern:
1. Define a constraint struct with particle indices and parameters
2. Implement a `project()` method that applies the constraint
3. Store constraints in custom vector wrappers (e.g., `DistanceConstraintVec`)
4. Custom vectors implement: `new()`, `update_counts()`, `solve()`, `push()`, `clear()`, `len()`, `iter_mut()`

### Entity Pattern
Entities are managed through the `EntitySystem`:
1. Entities have lifecycle methods (update, etc.)
2. Entities interact with the simulation through particle and constraint management
3. Example: `CarEntity` creates wheels using particles and constraints

## Known Issues & Current Work

### Car Wheel Collapse
- **Problem**: Wheels collapse when hitting surfaces at speed
- **Current Design**: Surface particles + hub connected via `DistanceConstraints`
- **Solution**: Implementing `VolumeConstraint` to maintain wheel shape

### Friction System
- **Static Friction** (`s_friction`): Prevents sliding when velocity is low
- **Kinetic Friction** (`k_friction`): Applied when object is sliding

## Build & Run

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run in release mode (recommended for physics performance)
cargo run --release
```

## Key References

- [Unified Particle Physics Paper](https://mmacklin.com/uppfrta_preprint.pdf)
- [Reference Implementation](https://github.com/ebirenbaum/ParticleSolver)
- [Ten Minute Physics](https://matthias-research.github.io/pages/tenMinutePhysics/index.html)
- [wgpu Tutorials](https://sotrh.github.io/learn-wgpu/)

## Code Style Notes

- Use `Vec2`, `Vec4` types from `core::math` for vectors
- Color constants available on `Vec4` (e.g., `Vec4::RED`, `Vec4::BLUE`)
- Avoid `Box<dyn Trait>` for constraints (prefer concrete types for memory coherence)
- Custom vector wrappers used instead of raw `Vec<T>` for constraint collections
