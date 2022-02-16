# Memory Management

Zaia implements a dialect of Lua, a language with automatic memory management.
Zaia preserves this aspect and implements a mark-and-sweep garbage collector to reclaim memory.

No specialized allocator is used, instead we defer to the `libc` allocator for memory management.
This is not the most performant system but it is a reasonable default and may be changed in the future.

The GC design used here is basic and does not allow for incremental collection or generational optimizations.
Instead, Zaia relies on effective optimizations such as allocation sinking to reduce the allocations made by regular code.

This is an area of improvement for Zaia but we're currently more focused on other areas. This may however be revisited in the future.