# DHT KernelSpace Routing with Userspace Storage(Rust)

## Overview

This project implements a **Distributed Hash Table (DHT)** using a split architecture: **routing logic** in the kernel space and **storage functionality** in userspace. This design allows efficient packet routing directly in the kernel while leveraging Rust in userspace for safe, flexible data management.

## Concept

Using a **Kademlia-based DHT**, the kernel module manages node discovery, routing, and packet forwarding based on an XOR distance metric, allowing efficient data lookup across distributed nodes. The kernel handles the DHT’s routing independently, guiding requests through the network to locate data on other nodes when necessary.

If a request arrives for data stored locally, the kernel module calls on a Rust-based userspace application to manage the data storage and retrieval. This separation allows clients to seamlessly locate data within the DHT without implementing their own DHT logic.

## Key Components

1. **Kernel space (Routing)**: The kernel module intercepts packets, manages the DHT routing table, and uses XOR-based lookups to route requests across the network.

2. **Userspace (Storage)**: The Rust application in userspace handles data storage for locally-responsible keys, supporting the kernel's routing layer without interfering with routing logic.

## Benefits

- **High Performance**: Kernel-level routing minimizes latency in DHT lookups.
- **Modularity**: Decoupling routing and storage allows each layer to scale or improve independently.
- **Safety**: Rust in userspace ensures safe, concurrent data handling for reliable storage management.

This project provides a robust, modular DHT implementation with efficient routing and safe, manageable storage, suited for distributed data access without centralization.
