# async_study

Learning async Rust by implementing an educational async runtime.

This project aims to be a comprehensive learning resource for understanding Rust's async ecosystem by implementing key features found in production runtimes like Tokio, but with a focus on clarity and educational value.

## Chapters (Subject to change based on my knowledge level)

### ✅ Phase 1: Foundation (Completed)

1. **Naive Executor** - `exec::e01_naive`
   - Basic executor with busy loop
   - No-op waker implementation
   - Understanding the simplest event loop

2. **Awakenable Executor** - `exec::e02_wake`
   - Proper waker implementation
   - Conditional variable for efficient waiting
   - Understanding how wakers notify the executor

3. **Multi-task Executor** - `exec::e03_multitask`
   - Support for spawning multiple tasks
   - Task queue management
   - Task scheduling and execution

4. **Joinable Tasks** - `exec::e04_join`
   - Oneshot channel implementation (`util::oneshot`)
   - `JoinHandle<T>` with return values
   - Understanding how to retrieve task results
   - Task completion notification via wakers

### 🚧 Phase 2: Time Management (Next)

5. **Yield Now**
   - `yield_now()` - cooperative task yielding
   - Understanding the simplest custom Future
   - Preventing task starvation

6. **Timer System**
   - `sleep(duration)` - async sleep
   - Timer wheel or min-heap for deadline tracking
   - Integration with executor event loop

7. **Timeout**
   - `timeout(duration, future)` - racing against time
   - Understanding Future cancellation
   - Building on sleep infrastructure

### 📋 Phase 3: Future Composition

8. **Join Combinators**
   - `join(a, b)` - wait for all futures
   - `join_all(vec)` - dynamic future collection
   - Understanding concurrent execution vs sequential

9. **Select Combinators**
   - `select(a, b)` - wait for first future
   - Understanding Future racing
   - Pin projection in practice

### 💬 Phase 4: Inter-task Communication

10. **Async MPSC Channel**
    - Bounded and unbounded channels
    - Backpressure handling
    - Multiple waiting senders/receivers

11. **Broadcast Channel** (Optional)
    - One-to-many communication
    - Multiple receivers for same message

12. **Watch Channel** (Optional)
    - Latest-value semantic
    - Change notification

### 🔒 Phase 5: Synchronization Primitives

13. **Async Mutex**
    - Why `std::sync::Mutex` doesn't work across await
    - Fair lock implementation with waker queue
    - Lock guards and RAII

14. **Async RwLock** (Optional)
    - Multiple readers or single writer
    - Understanding reader/writer fairness

15. **Semaphore** (Optional)
    - Limiting concurrent access
    - Resource pooling

### 🎯 Phase 6: Task Management

16. **Task Cancellation**
    - `JoinHandle::abort()`
    - Understanding cancellation safety
    - Drop and async interaction

17. **Panic Handling**
    - `Result<T, JoinError>` from JoinHandle
    - Catching panics in tasks
    - Preventing permanent blocking

18. **Task Local Storage** (Optional)
    - Per-task data storage
    - Similar to thread_local but for tasks

### ⚡ Phase 7: Multi-threading

19. **Work-stealing Scheduler**
    - Multiple executor threads
    - Task stealing for load balancing
    - Thread-safe task queue

20. **Thread Pool**
    - `spawn_blocking()` for CPU-bound work
    - Offloading blocking operations
    - Worker thread management
