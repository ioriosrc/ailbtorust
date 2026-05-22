```rust
use std::rc::Rc;

pub type ForwardedAnalytics = Rc<dyn IAnalytics>;

fn use_forward_analytics() -> ForwardedAnalytics {
    let value = Rc::new( AnalyticsContext::get_value());
    let store = Rc::new(create_store(move || {
        Ok({ value })
    }));
    use_layout_effect(|| {
        let _ = store.subscribe(move |_| {
            store.update(|state| state.value = *value.as_ref());
        });
    }, [store, value]);
    store
}

fn ForwardAnalyticsContextProvider({
    forwarded_analytics,
    children,
}: React::PropsWithChildren<{ forwarded_analytics: ForwardedAnalytics }>): React::JSX.Element {
    let _ = Rc::new( AnalyticsContext::get_value());
    let store = Rc::new(create_store(move || {
        Ok({ value })
    }));
    use_effect(|| {
        let unsubscribe = forwarded_analytics.subscribe(move |_| {
            store.update(|state| state.value = *value.as_ref());
        });
        return unsubscribe;
    }, [forwarded_analytics, store]);
    let { value } = Rc::new(store.borrow().to_owned());
    <AnalyticsContextProvider value={value}>{children}</AnalyticsContextProvider>
}
```