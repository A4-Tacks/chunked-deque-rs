Similar C++ `std::deque`, repr like `VecDeque<Box<[T; N]>>`

| /      | VecDeque | LinkedList | Deque             | c++ std::deque   |
| ---    | ---      | ---        | ---               | ---              |
| push   | realloc  | alloc      | opt alloc chunk   | opt alloc chunk  |
| pop    | no alloc | dealloc    | opt dealloc chunk | usually no alloc |

Level 2 buffer deque, like `LinkedList` timely release of reserves, like `VecDeque` cache friendly

This crate has not been fully optimized, I believe its performance can be improved

## Info
- Miri passed

## Benches (min struct)
- Due timely release of reserves, pop is a bit slow (about 1.5x to 2.5x, LinkedList is 10x)
- Due 'chunked', `get` methods slow (about 2x, LinkedList is 20x)
