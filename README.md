Similar C++ `std::deque`, repr like `VecDeque<Box<[T; N]>>`

| /      | VecDeque | LinkedList | Deque             | c++ std::deque   |
| ---    | ---      | ---        | ---               | ---              |
| push   | realloc  | alloc      | opt alloc chunk   | opt alloc chunk  |
| pop    | no alloc | dealloc    | opt dealloc chunk | usually no alloc |

Level 2 buffer deque, like `LinkedList` timely release of reserves, like `VecDeque` cache friendly

This crate has not been fully optimized, I believe its performance can be improved

## Info
- Miri passed
- No 'move elements' methods, like `insert` `remove` `drain` `retain`

## Benches (small struct)
- On a small struct, `push` methods is comparable in speed to `VecDeque` (LinkedList is 10x slow)
- Due timely release of reserves, `pop` methods is a bit slow (about 1.5x to 2.5x, LinkedList is 20x)
- Due 'chunked', `get` methods slow (about 2x)
