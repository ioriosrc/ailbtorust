```rust
use mockall::mock;
use test_log::*;

mod types {
    pub struct SharedConfig {
        topic_path: String,
        diff_method: String,
        diff_topic_path: String,
        diff_enabled: bool,
    }

    pub type UseSharedRawMessagesLogicProps<T> = crate::prelude::UseSharedRawMessagesLogicProps<T>;

    pub fn setup(input_override: Option<types::SharedConfig>) -> mock::Mock<Self> {
        let mut mock = mock::<Self>();

        if let Some(input) = input_override {
            mock.expect_set_config().with_args(&input).return_once(|input| {
                println!("Setting config to: {:?}", input);
            });
        }

        mock
    }
}

fn main() {
    use test_log::test;

    #[test]
    fn given_use_shared_raw_messages_logic_when_toggling_diff_then_enables_diff_when_currently_disabled() {
        let mut mock = types::setup(None);

        mock.expect_on_toggle_diff().with_any_args().return_once(|| {
            println!("Toggling diff to enabled");
        });

        // Call the function to be tested
        use_shared_raw_messages_logic::use_shared_raw_messages_logic(mock);

        assert_eq!(mock.call_count("set_config"), 1);
    }

    #[test]
    fn given_use_shared_raw_messages_logic_when_toggling_diff_then_disables_diff_when_currently_enabled() {
        let mut mock = types::setup(Some(types::SharedConfig {
            diff_enabled: true,
            ..Default::default()
        }));

        mock.expect_on_toggle_diff().with_any_args().return_once(|| {
            println!("Toggling diff to disabled");
        });

        // Call the function to be tested
        use_shared_raw_messages_logic::use_shared_raw_messages_logic(mock);

        assert_eq!(mock.call_count("set_config"), 1);
    }
}
```

Note: The above code assumes that `@lichtblick/suite-base` is a mockable crate and that the actual implementation of `useSharedRawMessagesLogic` calls methods like `onToggleDiff` and `setConfig`. You may need to adjust the test cases based on the actual implementation details.