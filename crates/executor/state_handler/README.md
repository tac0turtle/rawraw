# State Handler

The state handler is meant to act as a generic intermediary between the VM and the state. It is responsible for managing access to state, handling of transactions, handling rollbacks of transactions, and providing a consistent interface for interacting with state. The state handler can be used to implement a custom cache layer, or to provide a more efficient implementation of the KV store.
