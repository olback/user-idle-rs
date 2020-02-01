# Get a users idle time

| OS      | Supported               |
| ------- | ----------------------- |
| Linux   | :heavy_check_mark:      |
| Windows | :x: (wip)               |
| MacOS   | :question: (not tested) |

### Example

```rust
use user_idle::UserIdle;

let idle = UserIdle::get_time().unwrap();

let idle_seconds = idle.as_seconds();
let idle_minutes = idle.as_minutes();
// Check the documentation for more methods
```
