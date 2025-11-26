# Architecture Documentation

## System Overview

The merge-and-split project is structured in three main layers:

```
┌─────────────────────────────────────┐
│          Game Layer                 │
│  (Entities, Game Logic)             │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│       Simulation Layer              │
│  (Particles, Constraints, Physics)  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│        Engine Layer                 │
│  (Rendering, Window Management)     │
└─────────────────────────────────────┘
```

## Layer Details

### Core Layer (`src/core/`)

Provides fundamental math types and utilities used across all layers.

**Key Types:**
- `Vec2` - 2D vector with math operations
- `Vec4` - 4D vector, used for colors (RGBA) and 4D math
  - Color constants: `Vec4::RED`, `Vec4::BLUE`, `Vec4::WHITE`, etc.

### Engine Layer (`src/engine/`)

Handles all rendering and graphics pipeline management using wgpu.

**Key Components:**
- **Renderer**: Main rendering system
- **Instance Renderer**: Efficient rendering of multiple instances
- **Pipeline Management**: Shader compilation and GPU state

**Responsibilities:**
- Window creation and event handling (via winit)
- GPU resource management
- Draw call optimization
- Shader management

### Simulation Layer (`src/simulation/`)

Implements the Position-Based Dynamics (PBD) physics simulation.

**Key Components:**

#### Particles
- Basic unit of simulation
- Properties: position, velocity, mass, inverse mass
- Constraint count tracking for solver stability

#### Constraints
Constraints maintain physical relationships between particles:

1. **DistanceConstraint** - Maintains fixed distance between two particles
   - Used for: rigid connections, structural integrity
   
2. **SpringConstraint** - Spring-like behavior between particles
   - Used for: soft bodies, elastic materials
   
3. **VolumeConstraint** - Maintains volume of particle groups
   - Used for: preventing collapse, maintaining shape
   
4. **BoundaryConstraint** - Collision with world boundaries
   - Includes friction (static and kinetic)
   
5. **RigidContactConstraint** - Rigid body collision response

#### Constraint Vector Pattern

All constraints use custom vector wrappers instead of raw `Vec<T>`:

```rust
pub struct DistanceConstraintVec {
    constraints: Vec<DistanceConstraint>,
}

impl DistanceConstraintVec {
    pub fn new() -> Self { ... }
    pub fn update_counts(&self, particles: &mut [Particle]) { ... }
    pub fn solve(&self, particles: &mut [Particle], compliance: f32, dt: f32) { ... }
    pub fn push(&mut self, constraint: DistanceConstraint) { ... }
    pub fn clear(&mut self) { ... }
    pub fn len(&self) -> usize { ... }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut DistanceConstraint> { ... }
}
```

**Benefits:**
- Better memory coherence (no trait objects)
- Type-safe constraint management
- Centralized constraint solving logic
- Easy to add constraint-specific optimizations

#### Emitters
Generate particles and constraints:
- **FluidEmitter** - Creates fluid simulations
- **OpenSmokeEmitter** - Creates smoke effects

#### Shape Builders
Helper utilities for creating particle shapes:
- **Circle** - Creates circular particle arrangements
  - Supports different space distributions

### Game Layer (`src/game/`)

High-level game objects and logic built on top of the simulation.

**Key Components:**

#### Entity System
Manages game entities with lifecycle:
- Creation, update, destruction
- Integration with simulation layer

#### Entities
- **CarEntity** - Vehicle with wheel physics
  - Wheels: surface particles + hub + constraints
  - Current issue: wheel collapse at high speeds
  - Solution: VolumeConstraint implementation

## Data Flow

### Initialization
```
main.rs → App → Plugin → EntitySystem + Simulation
```

### Update Loop
```
1. Game Layer: Entity.update()
   ↓
2. Simulation Layer: 
   - Apply forces
   - Predict positions
   - Solve constraints (iterative)
   - Update velocities
   ↓
3. Engine Layer: Render particles/entities
```

### Constraint Solving (PBD)
```
For each iteration:
  1. Update constraint counts
  2. For each constraint type:
     - Calculate constraint violation
     - Compute correction
     - Apply correction to particles
  3. Update particle velocities from position changes
```

## Design Patterns

### 1. Custom Vector Wrappers
**Problem**: Generic `Vec<T>` doesn't provide constraint-specific operations  
**Solution**: Wrap `Vec<T>` with type-specific methods

### 2. Particle-Based Everything
**Problem**: Need unified physics for different materials  
**Solution**: All physics objects are particle collections with different constraints

### 3. Constraint Projection
**Problem**: Maintain physical relationships  
**Solution**: Iteratively project particles to satisfy constraints

### 4. Entity-Component Pattern
**Problem**: Manage complex game objects  
**Solution**: Entities own particles/constraints in simulation, updated each frame

## Performance Considerations

### Current (CPU)
- Constraint solving is O(n*iterations) where n = constraint count
- Custom vector wrappers avoid virtual dispatch overhead
- Memory coherence through concrete types (no `Box<dyn Trait>`)

### Future (GPU)
- Plan to port to compute shaders
- Parallel constraint solving
- Spatial hashing for collision detection

## Friction System

### Static Friction (`s_friction`)
- Applied when relative velocity is below threshold
- Prevents objects from sliding when at rest
- Higher values = more "sticky" surfaces

### Kinetic Friction (`k_friction`)
- Applied when object is sliding
- Opposes motion direction
- Usually lower than static friction (easier to keep sliding than to start)

## Extension Points

### Adding New Constraints
1. Create constraint struct in `src/simulation/constraints/`
2. Implement `project()` method
3. Create custom vector wrapper
4. Add to `Simulation` struct
5. Update `solve_constraints()` and `clear_constraints()`

### Adding New Entities
1. Create entity struct in `src/game/entity/entities/`
2. Implement entity lifecycle methods
3. Register with `EntitySystem`
4. Create particles and constraints in simulation

### Adding New Emitters
1. Create emitter in `src/simulation/emitters/`
2. Implement particle generation logic
3. Set up initial constraints
4. Integrate with simulation

## Future Architecture Changes

### XPBD Migration
- Add time-step independence
- Improve stability
- Gradient-based constraint formulation

### GPU Acceleration
- Move particle updates to compute shaders
- Parallel constraint solving
- Spatial data structures on GPU
