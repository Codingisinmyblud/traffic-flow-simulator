# Agrobot Multi-Agent Traffic Simulator

A discrete-event, graph-based traffic simulation system designed to coordinate agricultural autonomous mobile robots (agrobots) in constrained farming environments. 

## Overview

This project simulates fleets of agricultural robots moving across a field represented as a graph. It models the constraints of real-world farming operations:

- **Robots move along crop rows** → Represented as graph edges
- **Intersections between rows** → Represented as graph nodes
- **Robots must not collide** → Handled via conflict detection
- **Robots must coordinate tasks** → Managed by scheduling algorithms
- **Narrow lanes** → Modeled using capacity constraints on edges

### Key Features
- **Discrete-Event Engine**: Fast simulation that processes events sequentially without tick-loops.
- **Path Planning**: A* and Dijkstra algorithms to find shortest valid paths through crop rows.
- **Coordination & Scheduling**: Token reservation tables, scheduling logic, and node/edge conflict detection.
- **Deadlock Detection**: Wait-For Graph representations that identify wait-cycles and prevent system freeze states when multiple robots need to cross the same intersection.

## Architecture

```
┌──────────────────────────────┐
│        Visualization         │
├──────────────────────────────┤
│       Traffic Analysis       │
├──────────────────────────────┤
│       Simulation Engine      │
├──────────────────────────────┤
│   Multi-Agent Coordination   │
├──────────────────────────────┤
│   Path Planning (Graph)      │
├──────────────────────────────┤
│       Map / Environment      │
└──────────────────────────────┘
```

## Running the Project

Ensure you have Rust and Cargo installed correctly on your system.

### Running Demonstrations

You can run the built-in examples that demonstrate the capabilities of the engine.

Run a basic demonstration:
```bash
cargo run --example simple_demo
```

Run a larger scale stress test (100 robots navigating a 10x10 field network over 1000 simulated ticks):
```bash
cargo run --example stress_test
```

### Running Tests

Execute the unit test suites covering the core scheduling, conflict detection, pathfinding and deadlock systems:
```bash
cargo test
```

## Project Structure

- `src/agents/`: State modules and robot struct definitions.
- `src/coordination/`: Logic for detecting collisions, finding deadlock cycles, and resolving path conflicts via the scheduler.
- `src/map/`: Graph node and edge representations representing the field layout.
- `src/metrics/`: Systems to evaluate throughput and congestion in the field.
- `src/planning/`: A* / Dijkstra pathfinders and Reservation Tables.
- `src/simulation/`: The discrete-event engine priority queues for system events.
- `src/visualization/`: Trajectory export systems for graphical renderings in JSON format.
