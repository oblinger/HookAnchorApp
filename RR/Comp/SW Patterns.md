

#comp

**Creational Patterns**;;Singleton, Factory, Abstract Factory, Builder, Prototype <!--SR:!2025-01-10,2,248-->
**Structural Patterns**;;Adapter, Decorator, Facade, Proxy, Composite, Bridge <!--SR:!2025-02-21,1,228-->
**Behavioral Patterns**;;Observer, Strategy, Command, Iterator, State, Mediator, Chain of Responsibility <!--SR:!2025-02-21,1,208-->
**Concurrency Patterns**;;Thread Pool, Producer-Consumer, Monitor Object <!--SR:!2025-01-10,2,248-->
### 1. **Creational Patterns**
Singleton, Factory, Abstract Factory, Builder, Prototype
-?-
- **Singleton**: Ensures a class has only one instance and provides a global point of access to it.
- **Factory Method**: Creates objects without specifying the exact class of the object that will be created.
- **Abstract Factory**: Provides an interface for creating families of related or dependent objects without specifying their concrete classes. (pegboard)
- **Builder**: Constructs a complex object step by step.
- **Prototype**: Creates new objects by copying an existing object.
<!--SR:!2025-01-11,3,248-->

### 2. **Structural Patterns**
Adapter, Decorator, Facade, Proxy, Composite, Bridge
-?-
- **Adapter**: Bridges incompatible interfaces by wrapping an object. (translating)
- **Decorator**: Adds additional functionality to an object dynamically.
- **Facade**: Provides a simplified interface to a complex subsystem.
- **Proxy**: Provides a placeholder or surrogate for another object to control access to it.
- **Composite**: Composes objects into tree structures to represent part-whole hierarchies.
- **Bridge**: Separates abstraction from implementation so they can vary independently.
<!--SR:!2025-01-13,5,230-->

### 3. **Behavioral Patterns**
Observer, Strategy, Command, Iterator, State, Mediator, Chain of Responsibility
-?-
- **Observer**: Defines a one-to-many dependency so that when one object changes state, its dependents are notified.
- **Strategy**: Defines a family of algorithms, encapsulates each one, and makes them interchangeable.
- **Command**: Encapsulates a request as an object, allowing for parameterization and queuing of requests.
- **Iterator**: Provides a way to access elements of a collection sequentially without exposing its underlying representation.
- **State**: Allows an object to alter its behavior when its internal state changes.
- **Mediator**: Defines an object that encapsulates how a set of objects interact.
- **Chain of Responsibility**: Passes requests along a chain of handlers where each handler decides whether to handle the request or pass it along.
<!--SR:!2025-01-10,2,230-->

### 4. **Concurrency Patterns**
Thread Pool, Producer-Consumer, Monitor Object
-?-
- **Thread Pool**: Manages a pool of threads for executing tasks efficiently.
- **Producer-Consumer**: Coordinates a set of producer threads that create data and consumer threads that process it.
- **Monitor Object**: Ensures that only one thread at a time can execute a critical section of code. <!--SR:!2025-03-11,19,268-->