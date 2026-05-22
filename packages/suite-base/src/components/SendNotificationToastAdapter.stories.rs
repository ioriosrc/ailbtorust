```rust
use crate::components::SendNotificationToastAdapter;
use crate::util::send_notification;

fn fake_error() -> std::error::Error {
    let err = std::io::Error::new(std::io::ErrorKind::Other, "This error is on purpose - it comes from the story");
    err.set_stack_trace(Some(std::vec![std::stringify!({
        at http://localhost:49891/main.iframe.bundle.js:13051:22
        at finalStoryFn (http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:56275:32)
        at http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:53001:21
        at http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:54920:16
        at jsxDecorator (http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:48482:15)
        at http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:53001:21
        at http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:54884:12
        at http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:54920:16
        at withGrid (http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:45137:10)
        at http://localhost:49891/some_vendor_library-name_component-name-9a6f77.iframe.bundle.js:53001:21
    })])));
}

fn main() {
    // Example usage of the fake error
    send_notification("Something bad happened", fake_error(), "app", "error");

    // Other examples are handled in the SendNotificationToastAdapter component
}
```