# Agrobot Multi-Agent Traffic Simulator
A discrete-event, graph-based traffic simulation system designed to coordinate agricultural autonomous mobile robots (agrobots) in constrained farming environments. 
## Overview
This project simulates fleets of agricultural robots moving across a field represented as a graph. It models the constraints of real-world farming operations:
- **Robots move along crop rows** → Represented as graph edges
- **Intersections between rows** → Represented as graph nodes
- **Robots must not collide** → Handled via conflict detection
- **Robots must coordinate tasks** → Managed by scheduling algorithms
- **Narrow lanes** → Modeled using capacity constraints on edges

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
