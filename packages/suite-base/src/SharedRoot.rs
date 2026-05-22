```rust
use std::rc::Rc;

fn shared_root(props: Rc<SharedRootProps>) -> ReactElement {
    let SharedRootProps {
        appBarLeftInset,
        appConfiguration,
        onAppBarDoubleClick,
        AppBarComponent,
        children,
        customWindowControlProps,
        dataSources,
        deepLinks,
        enableGlobalCss = false,
        enableLaunchPreferenceScreen,
        extensionLoaders,
        extraProviders,
    } = props;

    let app_configuration_context_provider =
        AppConfigurationContextProvider::new(app_configuration);

    let app_parameters_provider =
        AppParametersProvider::new();

    let color_scheme_theme_provider =
        ColorSchemeThemeProvider::new(
            appBarLeftInset,
            AppBarComponent,
            app_configuration,
            customWindowControlProps,
            dataSources,
            deepLinks,
            enableLaunchPreferenceScreen,
            extensionLoaders,
            extraProviders,
        );

    let css_baseline = CssBaseline::new();

    let error_boundary = ErrorBoundary::new();

    let shared_root_context_provider =
        SharedRootContextProvider::new(
            appBarLeftInset,
            AppBarComponent,
            app_configuration,
            customWindowControlProps,
            dataSources,
            deepLinks,
            enableLaunchPreferenceScreen,
            extensionLoaders,
            extraProviders,
            onAppBarDoubleClick,
        );

    <AppConfigurationContextProvider value={app_configuration_context_provider}>
        <AppParametersProvider>
            <ColorSchemeThemeProvider>
                {enable_global_css && <GlobalCss />}
                <CssBaseline>
                    <ErrorBoundary>
                        <SharedRootContextProvider
                            value={shared_root_context_provider}
                        >
                            {children}
                        </SharedRootContextProvider>
                    </ErrorBoundary>
                </CssBaseline>
            </ColorSchemeThemeProvider>
        </AppParametersProvider>
    </AppConfigurationContextProvider>
}
```

Note: This is a high-level Rust translation of the TypeScript/React code. It assumes that `SharedRootProps`, `ColorSchemeThemeProvider`, `CssBaseline`, `ErrorBoundary`, `AppConfigurationContext`, and `SharedRootContextProvider` are already defined in your Rust project. The actual implementation of these components would depend on the specific requirements and functionality you need for each part of the shared root configuration.