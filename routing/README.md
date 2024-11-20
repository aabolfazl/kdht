# Routing Module:

[![Build Status](https://github.com/aabolfazl/kdht/actions/workflows/rust.yml/badge.svg)](https://github.com/aabolfazl/kdht/actions/workflows/rust.yml)

Implement Rust's core DHT routing logic, including node communication and efficient key-based lookups.

Here’s an overview of the current layers:

![Untitled-2023-06-25-2202](https://github.com/user-attachments/assets/c3354937-f90c-4263-b703-0ba0482378d9)
1. **Communication Layer**  
   - **Role**: Manages all low-level networking, using TCP/UDP sockets to send and receive packets.
   - **Responsibilities**:
     - Listens for incoming messages.
     - Handles non-blocking I/O and error handling specific to the protocol.
     - Passes received binary data to the next layer.
     - Sends binary data to the network.
     - Handle retries or error reporting if a node cannot be reached.

2. **Serialization/Deserialization Layer**  
   - **Role**: Converts raw binary data into structured messages that the RPC layer can interpret and vice versa.
   - **Responsibilities**:
     - Decodes the binary data to identify message types (`PING`, `FIND_NODE`, etc.).
     - Serializes structured responses back into binary format for UDP transmission.
     - Modularizing your serializer/deserializer for extensibility. This will allow us to easily support different message formats (like JSON for testing or debugging, or integrating with other DHT implementations).

3. **RPC Layer**  
   - **Role**: Routes structured messages to appropriate handlers based on the message type.
   - **Responsibilities**:
     - Calls the correct function based on the message type (`PING`, `FIND_NODE`, `STORE`, `FIND_VALUE`).
     - Manages responses by forwarding them to the communication layer.
     - **rate limiting or throttling** under high traffic. This can prevent overwhelming the routing layer with requests during load spikes or potential attacks.

4. **Routing Layer**  
   - **Role**: Implements core DHT functions and handles requests that require node lookups or modifications in the routing table.
   - **Responsibilities**:
     - Manages the routing table and k-buckets by implementing LRU (least recently used) based on paper suggestions.
     - **cache** mechanism for frequent lookups. For example, if a node repeatedly requests a specific key, caching the response can reduce lookup times.
     - Responds to `PING` requests, updates the routing table, and answers `FIND_NODE` requests by retrieving the closest nodes.
     - Forwards requests to the `storage` module when relevant (for `FIND_VALUE` and `STORE` requests).

### Layers Overview

1. **Communication Layer**: Listens on UDP, and handles low-level I/O.
2. **Serialization/Deserialization Layer**: Converts binary to/from structured messages, and maintains versioning.
3. **RPC Layer**: Routes messages to handlers, centralizes error handling and rate limiting.
4. **Routing Layer**: Manages routing table and k-buckets, interfaces with storage for specific requests.
