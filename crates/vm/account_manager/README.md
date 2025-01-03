# Account Manager Overview

The Account Manager is a core component responsible for managing account lifecycle, message execution, and state management in the VM system. It provides:

1. Account creation and destruction
2. Message execution and routing
3. Gas metering and limits
4. State management and storage access
5. Call stack management

## Key Components

### AccountManager

The main struct that coordinates all account-related operations. It holds:

- Code Manager (VM) - Resolves and executes handlers
- Call Stack - Tracks execution context
- Gas Stack - Manages gas consumption
- State Handler - Manages state access

### Execution Context

Manages the execution of messages including:

- Message routing
- Gas metering
- State transactions
- Call stack management

### State Handler

Provides an interface for:

- Key-value storage operations
- Account storage management  
- Transaction management
- Event emission

## Architecture

```mermaid
graph TD
    A[AccountManager] --> B[Code Manager]
    A --> C[State Handler]
    A --> D[Call Stack]
    A --> E[Gas Stack]
    
    B --> F[Handler Resolution]
    B --> G[Handler Execution]
    
    C --> H[Storage Operations]
    C --> I[Account Storage]
    C --> J[Transactions]
    
    D --> K[Execution Context]
    E --> L[Gas Metering]
```

# Storage Architecture

The Account Manager uses a layered storage architecture to manage state.

## Storage Layers

1. **State Handler Interface**
   - Defines core storage operations
   - Manages transactions
   - Handles account storage lifecycle

2. **Standard State Handler**
   - Implements state handler interface
   - Adds gas metering
   - Provides standard KV operations

3. **State Manager**
   - Backend storage implementation
   - Handles raw storage operations
   - Manages account namespacing

## Storage Operations

### Key-Value Operations

```mermaid
sequenceDiagram
    participant C as Client
    participant SH as StateHandler
    participant SM as StateManager
    participant S as Storage

    C->>SH: kv_get(account, key)
    activate SH
    SH->>SH: check gas
    SH->>SM: kv_get(account, scope, key)
    activate SM
    SM->>S: read(namespace + key)
    S-->>SM: value
    SM-->>SH: value
    deactivate SM
    SH->>SH: charge gas
    SH-->>C: value
    deactivate SH
```

### Account Storage Management

```mermaid
sequenceDiagram
    participant C as Client
    participant AM as AccountManager
    participant SH as StateHandler
    participant S as Storage

    C->>AM: create_account()
    activate AM
    AM->>SH: create_account_storage()
    activate SH
    SH->>S: init_namespace()
    S-->>SH: ok
    SH-->>AM: ok
    deactivate SH
    AM-->>C: account_id
    deactivate AM
```

# Message Execution

The Account Manager handles message execution through a series of coordinated steps.

## Message Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant AM as AccountManager
    participant EC as ExecContext
    participant VM as CodeManager
    participant H as Handler
    participant S as StateHandler

    C->>AM: invoke_msg(message)
    activate AM
    AM->>EC: new(state, gas)
    activate EC
    
    EC->>EC: push_call_frame()
    EC->>EC: begin_tx()
    
    EC->>VM: resolve_handler()
    VM-->>EC: handler
    
    EC->>H: handle_msg()
    activate H
    
    H->>S: update_state()
    S-->>H: result
    
    H-->>EC: response
    deactivate H
    
    EC->>EC: commit_tx()
    EC->>EC: pop_call_frame()
    
    EC-->>AM: response
    deactivate EC
    AM-->>C: response
    deactivate AM
```

## Key Steps

1. **Message Receipt**
   - Validate message format
   - Check target account exists
   - Initialize execution context

2. **Handler Resolution**
   - Look up account's handler ID
   - Resolve handler through VM
   - Prepare handler context

3. **Execution**
   - Push call frame
   - Begin transaction
   - Execute handler
   - Handle response/errors
   - Commit/rollback transaction
   - Pop call frame

4. **State Management**
   - Track gas usage
   - Manage state access
   - Handle storage operations
   - Emit events

# Step-by-Step Walkthrough

This guide walks through the key operations in the Account Manager.

## Account Creation

1. Client sends create message to root account
2. Account Manager:
   - Generates new account ID
   - Initializes account storage
   - Sets handler ID
   - Calls handler's on_create

```mermaid
sequenceDiagram
    participant C as Client
    participant AM as AccountManager
    participant IDG as IDGenerator
    participant SH as StateHandler
    participant H as Handler

    C->>AM: create(handler_id, init_data)
    activate AM
    
    AM->>IDG: new_account_id()
    IDG-->>AM: id
    
    AM->>SH: create_account_storage(id)
    SH-->>AM: ok
    
    AM->>SH: set_handler_id(id, handler_id)
    SH-->>AM: ok
    
    AM->>H: on_create(init_data)
    H-->>AM: ok
    
    AM-->>C: account_id
    deactivate AM
```

## Message Execution

1. Client sends message to account
2. Account Manager:
   - Pushes call frame
   - Resolves handler
   - Executes message
   - Manages state access
   - Returns response

```mermaid
sequenceDiagram
    participant C as Client
    participant AM as AccountManager
    participant CS as CallStack
    participant VM as CodeManager
    participant H as Handler
    participant SH as StateHandler

    C->>AM: invoke_msg(target, msg)
    activate AM
    
    AM->>CS: push(target)
    
    AM->>VM: resolve_handler(target)
    VM-->>AM: handler
    
    AM->>H: handle_msg(msg)
    activate H
    
    H->>SH: update_state()
    SH-->>H: result
    
    H-->>AM: response
    deactivate H
    
    AM->>CS: pop()
    
    AM-->>C: response
    deactivate AM
```

## State Access

1. Handler requests state access
2. Account Manager:
   - Validates permissions
   - Meters gas
   - Performs operation
   - Returns result

```mermaid
sequenceDiagram
    participant H as Handler
    participant AM as AccountManager
    participant G as GasMeter
    participant SH as StateHandler
    participant S as Storage

    H->>AM: update_state(key, value)
    activate AM
    
    AM->>G: consume_gas()
    G-->>AM: ok
    
    AM->>SH: kv_set(account, key, value)
    activate SH
    
    SH->>S: write(key, value)
    S-->>SH: ok
    
    SH-->>AM: ok
    deactivate SH
    
    AM-->>H: ok
    deactivate AM
```

This documentation provides a comprehensive overview of the Account Manager's functionality, architecture and key workflows. Let me know if you would like me to expand on any particular area or add additional diagrams.
