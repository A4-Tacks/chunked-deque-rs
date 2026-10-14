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

| bench of VecDeque  | 100   | 1000  | 10000 | 100000 | 1000000 |
| ---                | ---   | ---   | ---   | ---    | ---     |
| `push_front`       | 1.01x | 1.02x | 1.01x | 0.93x  | 0.94x   |
| `push_back`        | 1.89x | 1.22x | 0.97x | 0.88x  | 0.97x   |
| `pop_front`        | 0.85x | 0.84x | 0.84x | 0.84x  | 0.84x   |
| `pop_back`         | 0.40x | 0.40x | 0.40x | 0.41x  | 0.41x   |
| `push_mixed`       | 0.99x | 0.93x | 0.91x | 0.84x  | 0.88x   |
| `pop_mixed`        | 0.77x | 0.40x | 0.75x | 1.02x  | 1.02x   |
| `drop`             | 0.30x | 0.30x | 0.30x | 0.30x  | 0.30x   |
| `push_back_string` | 1.07x | 1.18x | 1.25x | 1.29x  | 1.29x   |
| `push_back_medium` | 0.76x | 0.81x | 0.91x | 0.99x  | 0.65x   |
| `push_back_large`  | 0.71x | 0.75x | 0.95x | 1.29x  | 1.28x   |
| `pop_back_string`  | 0.80x | 0.80x | 0.80x | 0.81x  | 0.80x   |
| `pop_back_medium`  | 0.54x | 0.54x | 0.53x | 0.53x  | 0.55x   |
| `pop_back_large`   | 0.75x | 0.74x | 0.74x | 0.75x  | 0.75x   |
| `get`              | 1.06x | 0.78x | 0.59x | 0.67x  | 0.69x   |
| `read_front`       | 0.49x | 0.50x | 0.49x | 0.49x  | 0.49x   |
| `read_back`        | 0.49x | 0.49x | 0.49x | 0.49x  | 0.49x   |
| `iterate`          | 0.96x | 0.95x | 0.96x | 0.96x  | 0.96x   |
| `iterate_rev`      | 1.88x | 1.88x | 1.88x | 1.87x  | 1.88x   |
