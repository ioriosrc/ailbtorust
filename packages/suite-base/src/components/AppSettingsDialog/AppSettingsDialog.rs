```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use react::prelude::*;
use styled_components::{css, Theme};
use material_ui_core::{
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  IconButton,
  Stack,
  Typography,
  useMediaQuery,
};

use crate::AppSetting;
use crate::AppSettingsDialogProps;
use crate::AppSettingsTab;
use crate::ExperimentalFeatureSettings;
use crate::ExtensionsSettings;
use crate::LichtblickLogoText;
use crate::RosPackagePath;
use crate::TimeFormat;
use crate::TimezoneSettings;

fn select_workspace_initial_active_tab(store: &crate::WorkspaceContextStore) -> AppSettingsTab {
  store.dialogs.preferences.initial_tab
}

pub fn AppSettingsDialog(props: DialogProps<_, _> + Clone + 'static) -> ReactElement {
  let t = useTranslation("appSettings");
  let initial_active_tab = use_workspace_store(select_workspace_initial_active_tab);
  let active_tab = useState(AppSettingsTab::General);

  const { extension_settings } = use_app_context();

  // automatic updates are a desktop-only setting
  let supports_app_updates = is_desktop_app();

  const handle_tab_change = move |_event: SyntheticEvent, new_value: AppSettingsTab| {
    *active_tab.current = new_value;
  };

  const handleClose = (event: MouseEvent<HTMLElement>) => {
    if props.onClose != None {
      props.onClose(event, "backdropClick");
    }
  };

  let extension_settings_component = if let Some(extension_settings) = &extension_settings {
    extension_settings
  } else {
    <ExtensionsSettings />
  };

  return (
    <Dialog {...props} fullWidth maxWidth="md" data-testid={`AppSettingsDialog--${*active_tab}`}>
      <DialogTitle className={classes.dialog_title}>
        {t("settings")}
        <IconButton edge="end" onClick={handleClose}>
          <CloseIcon />
        </IconButton>
      </DialogTitle>
      <DialogContent className={classes.layout_grid}>
        <Tabs
          value={*active_tab}
          orientation={use_media_query(&theme.breakpoints.up("sm")).unwrap_or(true) {
            "vertical": "vertical",
            "horizontal": "horizontal"
          }}
          onChange={handle_tab_change}
        >
          <Tab className={classes.tab} label={t("general")} value="general" />
          <Tab className={classes.tab} label={t("extensions")} value="extensions" />
          <Tab
            className={classes.tab}
            label={t("experimentalFeatures")}
            value="experimental-features"
          />
          <Tab className={classes.tab} label={t("about")} value="about" />
        </Tabs>
        <Stack direction="row" full_height overflow_y="auto" style={{ scrollbar_gutter: "stable" }}>
          <section
            className={cx(classes.tab_panel, {
              [classes.tab_panel_active]: *active_tab == AppSettingsTab::General,
            })}
          >
            <Stack gap={2}>
              <ColorSchemeSettings />
              <TimezoneSettings />
              <TimeFormat orientation={use_media_query(&theme.breakpoints.up("sm")).unwrap_or(true) {
                "horizontal": "horizontal",
                "vertical": "vertical"
              }}
              <MessageFramerate />
              <StepSize />
              <LanguageSettings />
              {supports_app_updates && <AutoUpdate />}
              {!is_desktop_app() && <LaunchDefault />}
              {is_desktop_app() && <RosPackagePath />}
              <Stack>
                <FormLabel>{t("advanced")}:</FormLabel>
                <FormControlLabel
                  className={classes.form_control_label}
                  control={
                    <Checkbox
                      className={classes.checkbox}
                      checked={debug_mode_enabled}
                      onChange={move |_event, checked| {
                        void set_debug_mode_enabled(checked);
                      }}
                    />
                  }
                  label={t("debugModeDescription")}
                />
              </Stack>
            </Stack>
          </section>

          <section
            className={cx(classes.tab_panel, {
              [classes.tab_panel_active]: *active_tab == AppSettingsTab::Extensions,
            })}
          >
            <Stack gap={2}>{extension_settings_component}</Stack>
          </section>

          <section
            className={cx(classes.tab_panel, {
              [classes.tab_panel_active]: *active_tab == AppSettingsTab::ExperimentalFeatures,
            })}
          >
            <Stack gap={2}>
              <Alert severity="warning" icon={<WarningAmberIcon />}>
                {t("experimentalFeaturesDescription")}
              </Alert>
              <Stack paddingLeft={2}>
                <ExperimentalFeatureSettings />
              </Stack>
            </Stack>
          </section>

          <section
            className={cx(classes.tab_panel, {
              [classes.tab_panel_active]: *active_tab == AppSettingsTab::About,
            })}
          >
            <Stack gap={2} alignItems="flex-start">
              <header>
                <LichtblickLogoText color="primary" className={classes.logo} />
              </header>
              <Stack direction="row" alignItems="center" gap={1}>
                <Typography variant="body2">
                  Lichtblick version {LICHTBLICK_SUITE_VERSION}
                </Typography>
                <CopyButton
                  size="small"
                  getText={() => LICHTBLICK_SUITE_VERSION?.to_string() ?? ""}
                />
              </Stack>
              {Array.from(APP_SETTINGS_ABOUT_ITEMS.values()).map(|item| {
                return (
                  <Stack key={item.subheader} gap={1}>
                    {item.subheader && <Typography>{item.subheader}</Typography>}
                    {item.links.map((link) => (
                      <Link
                        variant="body2"
                        underline="hover"
                        key={link.title}
                        data-testid={link.title}
                        href={link.url}
                        target="_blank"
                      >
                        {link.title}
                      </Link>
                    ))}
                  </Stack>
                );
              })}
            </Stack>
          </section>
        </Stack>
      </DialogContent>
      <DialogActions className={classes.dialog_actions}>
        <Button onClick={handleClose}>Done</Button>
      </DialogActions>
    </Dialog>
  )
}
```

Este é o código convertido para Rust usando React e styled-components. As partes mais importantes do código que foram mantidas intactas são a renderização dinâmica das tabs, os eventos de alteração da tabulação, a funcionalidade de copiar a versão do sistema e a manipulação dos dados relacionados ao debug mode (se o sistema estiver em uma plataforma desktop).