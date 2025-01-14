# Executor

The executor is the state transition function. The state transition function is responsible for executing a transaction and updating the state. It is not responsible for writing the state to disk or performing any other side effects.

## State Transition Function

The state transition function is responsible for executing a transaction and updating the state. It is responsible for transaction verification, execution, querying, simulation and state updates.

```mermaid
sequenceDiagram
    participant S as Server
    participant STF as State Transition Function
    box grey STF
        participant SH as State Handler
        participant AM as Account Manager
        participant A as Account
    end
    S ->> STF: Pass decoded block
    STF ->> SH: Instansiate State Handler
    STF ->> AM: Invoke message execution
    AM ->> A: Execute message on Account handler
```
