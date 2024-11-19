# IXC Storage

Storage abstracts the underlying storage mechanism for the framework. It handles caching and grouping of state changes before committing them to the underlying storage.

## Commit Flow

```mermaid
sequenceDiagram
    participant SR as Server
    box gray State Transition Function
    participant STF as Transaction Executor
    participant CSC as Snapshot Cache
    end
    participant S as Storage 
    participant CS as Commitment structure
    participant DB as Disk
    SR ->> STF: Provide decoded transactions
    STF ->> CSC: Output of tx execution
    CSC ->> SR: State changes 
    SR ->> S: Commit ChangeSet
    S ->> CS: Commit changes
    CS ->> DB: Write changes to disk 
```

## Query FLow

```mermaid
sequenceDiagram
    participant SR as Server
    participant STF as State Transition Function
    participant S as Storage 
    participant CS as Commitment structure
    participant DB as Disk
    SR ->> STF: Module Query 
    SR ->> S: Raw State Query 
    S ->> CS: (IF) query with proof
    CS ->> DB: Disk Lookup
    S ->> DB: (IF) query without proof
```

## Query Flow

## State Changes

State changes are the atomic units of state updates. They are grouped together and committed in a single transaction.

## Snapshots

Snapshots are a way to revert the state to a previous state. They are used to implement the `revert_to` method on the `StateObject` trait.

## Caches

Caches are used to store state changes and snapshots. They are used to implement the `revert_to` method on the `StateObject` trait.
