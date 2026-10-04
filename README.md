Similar C++ `std::deque`, repr like `VecDeque<Box<[T; N]>>`

| /      | VecDeque | LinkedList | Deque             | c++ std::deque   |
| ---    | ---      | ---        | ---               | ---              |
| push   | realloc  | alloc      | opt alloc chunk   | opt alloc chunk  |
| pop    | no alloc | dealloc    | opt dealloc chunk | usually no alloc |

## Info
- Miri passed
- Bench missing
