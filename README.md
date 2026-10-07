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
- On a small struct, `push` methods is comparable in speed to `VecDeque` (LinkedList is 0.1x slow)
- Due timely release of reserves, `pop` methods is a bit slow (about 0.4x to 1x, LinkedList is 0.05x)
- Due 'chunked', `get` methods slow (about 0.5x)

| bench of VecDeque  | scale |
| ---                | ---   |
| `push_front`       | 1.25x |
| `push_back`        | 1.11x |
| `pop_front`        | 0.82x |
| `pop_back`         | 0.40x |
| `push_mixed`       | 1.08x |
| `pop_mixed`        | 1.00x |
| `push_back_string` | 0.97x |
| `push_back_medium` | 0.75x |
| `push_back_large`  | 0.99x |
| `pop_back_string`  | 0.79x |
| `pop_back_medium`  | 0.54x |
| `pop_back_large`   | 0.75x |
| `get`              | 0.59x |
| `read_front`       | 0.50x |
| `read_back`        | 0.50x |
| `iterate`          | 0.94x |
| `iterate_rev`      | 1.24x |
