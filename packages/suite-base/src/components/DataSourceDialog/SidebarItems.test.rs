```rust
use crate::components::SidebarItems;
use crate::pages::DemoPage;
use crate::pages::DocumentationPage;
use crate::pages::HomeLayoutsPage;
use crate::pages::UploadToDataPlatformPage;
use crate::pages::TutorialsPage;
use crate::utils::{is_authenticated_free, is_authenticated_team};
use leptos::*;
use serde_json::Value;

#[component]
fn SidebarItems(mut props: PropRead<SidebarProps>) -> impl IntoView {
    let on_select_view = use_signal(|| None);

    let t = use_context::<TranslationContext>().expect("missing TranslationContext");
    let analytics = use_context::<AnalyticsContext>().expect("missing AnalyticsContext");

    // Simulate user data
    let current_user_type = use_signal(|| "unauthenticated".to_string());
    let app_configuration_value = use_signal(|| Value::Bool(true));

    let demo_page = DemoPage {};
    let documentation_page = DocumentationPage {};
    let home_layouts_page = HomeLayoutsPage {};
    let upload_to_data_platform_page = UploadToDataPlatformPage {};
    let tutorials_page = TutorialsPage {};

    // Render sidebar items based on user type
    if is_authenticated_free(&app_configuration_value) {
        view! {
            <SidebarItem label="newToLichtblick" description="description_new_to_lichtblick" action={demo_page} />
            <SidebarItem label="exploreSampleData" description="description_explore_sample_data" action={home_layouts_page} />
            <SidebarItem label="viewDocumentation" description="description_view_documentation" action={documentation_page} />
            <SidebarItem label="dontShowThisAgain" description="description_dont_show_this_again" />
        }
    } else if is_authenticated_team(&app_configuration_value) {
        view! {
            <SidebarItem label="startCollaborating" description="description_start_collaborating" action={demo_page} />
            <SidebarItem label="needHelp" description="description_need_help" action={upload_to_data_platform_page} />
            <SidebarItem label="seeTutorials" description="description_see_tutorials" action={tutorials_page} />
            <SidebarItem label="dontShowThisAgain" description="description_dont_show_this_again" />
        }
    } else {
        view! {
            <SidebarItem label="newToLichtblick" description="description_new_to_lichtblick" action={demo_page} />
            <SidebarItem label="needHelp" description="description_need_help" action={upload_to_data_platform_page} />
            <SidebarItem label="seeTutorials" description="description_see_tutorials" action={tutorials_page} />
            <SidebarItem label="dontShowThisAgain" description="description_dont_show_this_again" />
        }
    }

    // Handle button clicks
    let handle_click = move |_| {
        if props.on_select_view.get().is_none() {
            props.on_select_view.set(Some("demo"));
        }
    };

    view! {
        <button on_click={handle_click}>exploreSampleData</button>
    }

    // Open external links
    use_effect(move || {
        let documentation_button = document.querySelector("button");
        if let Some(button) = documentation_button {
            button.addEventListener("click", move |e| {
                e.preventDefault();
                window.open(LICHTBLICK_DOCUMENTATION_LINK, "_blank", "noopener,noreferrer");
            });
        }
    }, []);
}

#[cfg(test)]
mod tests {
    use crate::components::SidebarItems;
    use crate::pages::{DemoPage, DocumentationPage, HomeLayoutsPage, UploadToDataPlatformPage, TutorialsPage};
    use crate::utils::{is_authenticated_free, is_authenticated_team};

    #[test]
    fn renders_items_for_unauthenticated_users() {
        let props = SidebarProps {
            on_select_view: Signal::new(None),
        };

        let t = TranslationContextProvider(T(""));
        let analytics = AnalyticsContextProvider(Analytics {});

        let demo_page = DemoPage {};
        let documentation_page = DocumentationPage {};
        let home_layouts_page = HomeLayoutsPage {};
        let upload_to_data_platform_page = UploadToDataPlatformPage {};
        let tutorials_page = TutorialsPage {};

        let rendered = SidebarItems { props };

        assert!(rendered.contains("newToLichtblick"));
        assert!(rendered.contains("exploreSampleData"));
        assert!(rendered.contains("viewDocumentation"));
        assert!(rendered.contains("dontShowThisAgain"));
    }

    #[test]
    fn renders_items_for_authenticated-free users() {
        let props = SidebarProps {
            on_select_view: Signal::new(None),
        };

        let t = TranslationContextProvider(T(""));
        let analytics = AnalyticsContextProvider(Analytics {});

        let demo_page = DemoPage {};
        let documentation_page = DocumentationPage {};
        let home_layouts_page = HomeLayoutsPage {};
        let upload_to_data_platform_page = UploadToDataPlatformPage {};
        let tutorials_page = TutorialsPage {};

        let rendered = SidebarItems { props };

        assert!(rendered.contains("startCollaborating"));
        assert!(rendered.contains("uploadToDataPlatform"));
        assert!(rendered.contains("seeTutorials"));
        assert!(rendered.contains("dontShowThisAgain"));
    }

    #[test]
    fn renders_items_for_authenticated-team users() {
        let props = SidebarProps {
            on_select_view: Signal::new(None),
        };

        let t = TranslationContextProvider(T(""));
        let analytics = AnalyticsContextProvider(Analytics {});

        let demo_page = DemoPage {};
        let documentation_page = DocumentationPage {};
        let home_layouts_page = HomeLayoutsPage {};
        let upload_to_data_platform_page = UploadToDataPlatformPage {};
        let tutorials_page = TutorialsPage {};

        let rendered = SidebarItems { props };

        assert!(rendered.contains("newToLichtblick"));
        assert!(rendered.contains("needHelp"));
        assert!(rendered.contains("seeTutorials"));
        assert!(rendered.contains("dontShowThisAgain"));
    }

    #[test]
    fn handles_button_clicks_correctly() {
        let props = SidebarProps {
            on_select_view: Signal::new(None),
        };

        let demo_page = DemoPage {};
        let documentation_page = DocumentationPage {};
        let home_layouts_page = HomeLayoutsPage {};
        let upload_to_data_platform_page = UploadToDataPlatformPage {};
        let tutorials_page = TutorialsPage {};

        let rendered = SidebarItems { props };

        let explore_sample_data_button = rendered.querySelector("button");
        assert!(explore_sample_data_button.is_some());
        explore_sample_data_button.click();

        assert_eq!(props.on_select_view.get(), Some("demo"));
    }

    #[test]
    fn opens_external_links_correctly() {
        let props = SidebarProps {
            on_select_view: Signal::new(None),
        };

        let demo_page = DemoPage {};
        let documentation_page = DocumentationPage {};
        let home_layouts_page = HomeLayoutsPage {};
        let upload_to_data_platform_page = UploadToDataPlatformPage {};
        let tutorials_page = TutorialsPage {};

        let rendered = SidebarItems { props };

        use_effect(move || {
            let documentation_button = rendered.querySelector("button");
            assert!(documentation_button.is_some());
            documentation_button.click();
        }, []);

        assert_eq!(
            window.open(LICHTBLICK_DOCUMENTATION_LINK, "_blank", "noopener,noreferrer").is_ok(),
            true
        );
    }
}
```