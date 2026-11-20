# SigmaOS-7-days-to-die Component Wiki

## Overview
SigmaOS-7-days-to-die is a SigmaOS component providing integration with the 7 Days to Die game server and client functionalities. It demonstrates game server integration and multiplayer support within the SigmaOS gaming ecosystem.

## Principles
- Single Responsibility
- Game Server Abstraction
- Fail-Fast on Server Connection Errors
- Deterministic Server Initialization
- Clear Abstraction Over Game Server APIs
- Composition over Inheritance for server modules

## Architecture
SigmaOS-7-days-to-die follows a game server integration pattern:
- **Server Manager**: Handles 7 Days to Die server lifecycle (start, stop, restart)
- **Player Connection**: Manages player connections and session state
- **World State**: Tracks world modifications and entity updates
- **Configuration**: Manages server settings, difficulty, and mod support

## API
The component provides the following public APIs:
- `server::start(config)` - Start a 7 Days to Die server instance
- `server::stop(server_id)` - Stop a running server instance
- `server::status(server_id)` - Get server status and player count
- `player::connect(address, credentials)` - Connect a player to the server
- Error types for server failures, connection errors, or configuration issues

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific server configuration
2. **Act**: Execute the code under test (server start/stop, player connection)
3. **Assert**: Verify the expected server results

Example:
```rust
#[test]
fn test_server_start_aaa() {
    // Arrange
    let config = server::Config { host: "localhost".to_string(), port: 26900 };
    
    // Act
    let server_id = server::start(config);
    
    // Assert
    assert!(server::status(server_id).is_ok());
    assert!(server::status(server_id).unwrap().is_running());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Server initialization is best-effort; gracefully degrades on port conflicts
- Comprehensive error types for all server-related failure scenarios
- Unit tests provided for server configuration and status checks
- Integration tests for full server lifecycle with test environment

## TODO
- Add mod support and workshop integration
- Expand player management and authentication
- Add world state synchronization APIs
- Create integration tests with actual 7 Days to Die server
- Implement server query and status reporting