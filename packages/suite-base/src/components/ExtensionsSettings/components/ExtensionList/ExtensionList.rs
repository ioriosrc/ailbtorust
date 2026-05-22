```rust
use react::prelude::*;
use mui::material::{Button, Typography};
use mui::x_data_grid::{DataGrid, GridColDef, GridRenderCellParams};
use mui::components::data_grid::{PaginationModel, ColumnVisibilityModel};
use mui::context::snackbar::useSnackbar;
use mui::hooks::useTranslation;
use mui::services::analytics::useAnalytics;
use mui::utils::is_desktop_app;
use suite_base::context::extension_catalog::use_extension_catalog;
use suite_base::context::extension_marketplace_context::use_extension_marketplace_detail;
use suite_base::util::can_install_extension;

#[derive(Debug)]
struct ExtensionMarketplaceDetail {
    id: String,
    name: String,
    version: String,
    publisher: String,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (app, app_handle) = App::new();

    let namespace = "some_namespace";
    let entries = vec![ExtensionMarketplaceDetail {
        id: "123",
        name: "Example Extension",
        version: "1.0",
        publisher: "Bmw AG",
        description: "This is an example extension.",
    }];

    let (t, t_handle) = use_translation();
    let enqueue_snackbar = use_snackbar();

    let installed_extensions = use_extension_catalog(|state| state.installed_extensions);
    let uninstall_extension = use_extension_catalog(|state| state.uninstall_extension);

    let analytics = use_analytics();

    let selected_extension_ids = React.useState<Vec<String>>(vec![]);
    let is_bulk_operating = React.useState(false);

    let handle_install = use_extension_operations(|_| {});
    let handle_uninstall = use_extension_operations(|id: String, namespace: &str| {
        new Promise((resolve) => setTimeout(resolve, 100)).then(|| {
            uninstall_extension(namespace, id).unwrap();
            analytics.log_event(AppEvent::EXTENSION_UNINSTALL, { type: id });
        })
    });

    let operation_status = use_extension_operations(|_| {});
    let is_operating = use_extension_operations(|id: String| {
        installed_extensions
            .get(&id)
            .map_or(false, |ext| ext.namespace == namespace)
    });

    let is_extension_installed = React.useCallback(
        (id: &str) -> bool => {
            installed_extensions
                .iter()
                .any(|installed| installed.id == id && installed.namespace == namespace)
        },
        [installed_extensions, namespace],
    );

    let handle_bulk_uninstall = React.useCallback(async () => {
        let selected_extensions = entries.iter().filter_map(|entry| {
            if selected_extension_ids.contains(&entry.id) {
                Some(entry.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>();

        if selected_extensions.is_empty() {
            enqueue_snackbar(t!("noExtensionsFound"), { variant: "info" });
            return;
        }

        is_bulk_operating.set(true);

        let success_count = 0;
        let fail_count = 0;

        for extension in &selected_extensions {
            match new Promise::<_, ()>(|resolve| setTimeout(resolve, 100)) {
                Ok(_) => {
                    await uninstall_extension(extension.namespace.unwrap_or("local"), extension.id);
                    success_count += 1;
                    analytics.log_event(AppEvent::EXTENSION_UNINSTALL, { type: extension.id });
                },
                Err(error) => {
                    println!("Failed to uninstall extension: {}", error);
                    fail_count += 1;
                }
            }
        }

        if success_count > 0 {
            enqueue_snackbar(t!("successCountExtensionsUninstalled"), { variant: "success" });
        }
        if fail_count > 0 {
            enqueue_snackbar(t!("failCountExtensionsUninstalled"), { variant: "error" });
        }

        is_bulk_operating.set(false);
        selected_extension_ids.set(vec![]);
    }, [
        analytics,
        enqueue_snackbar,
        entries,
        is_extension_installed,
        selected_extension_ids,
        handle_uninstall,
    ]);

    let columns = vec![
        {
            field: "name",
            headerName: "Name",
            flex: 1,
            sortable: true,
        },
        {
            field: "version",
            headerName: "Version",
            flex: 0.5,
            sortable: true,
        },
        {
            field: "publisher",
            headerName: "Publisher",
            flex: 0.5,
            sortable: true,
        },
        {
            field: "description",
            headerName: "Description",
            flex: 2,
        },
        {
            field: "actions",
            headerName: "Actions",
            flex: 1,
            sortable: false,
            renderCell: (params: GridRenderCellParams) => {
                let extension = params.row as ExtensionMarketplaceDetail;
                let is_installed = is_extension_installed(&extension.id);
                let is_extension_operating = is_operating(&extension.id);

                if (is_installed) {
                    return (
                        <Button
                            size="small"
                            color="inherit"
                            variant="outlined"
                            onClick={() => handle_uninstall(extension.namespace.unwrap_or("local"), extension.id)}
                            disabled={is_bulk_operating}
                            label={ExtensionActionsLabel.UNINSTALL}
                            loadingLabel={ExtensionOperationStatusLabel.UNINSTALLING}
                        />
                    );
                } else {
                    return (
                        <Button
                            size="small"
                            color="inherit"
                            variant="outlined"
                            onClick={() => handle_install(extension.namespace.unwrap_or("local"), extension.id)}
                            disabled={is_bulk_operating}
                            label={ExtensionActionsLabel.INSTALL}
                            loadingLabel={ExtensionOperationStatusLabel.INSTALLING}
                        />
                    );
                }
            },
        },
    ];

    let render_component = React.useMemo(
        () => {
            if (entries.is_empty() && filter_text) {
                return generate_placeholder_list(t!("noExtensionsFound"));
            } else if (entries.is_empty()) {
                return generate_placeholder_list(t!("noExtensionsAvailable"));
            }

            let selected_extensions = entries
                .iter()
                .filter_map(|entry| {
                    if selected_extension_ids.contains(&entry.id) {
                        Some(entry.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            return (
                <Stack gap={1}>
                    <Stack direction="row" gap={1} paddingX={2}>
                        <Typography
                            variant="body2"
                            color="text.secondary"
                            alignSelf="center"
                            paddingY={1}
                            style={{ visibility: selected_extensions.len() > 0 ? "visible" : "hidden" }}
                        >
                            {selected_extension_ids.len()} selected
                        </Typography>
                        {selected_extensions.len() > 0 && (
                            <Button
                                size="small"
                                color="inherit"
                                variant="outlined"
                                onClick={handle_bulk_uninstall}
                                disabled={is_bulk_operating}
                            >
                                {is_bulk_operating
                                    ? ExtensionOperationStatusLabel.UNINSTALLING
                                    : `${ExtensionActionsLabel.UNINSTALL} ${selected_extensions.len}`}
                            </Button>
                        )}
                    </Stack>
                    <div>
                        <DataGrid
                            rows={entries}
                            columns={columns}
                            initialState={{
                                pagination: { paginationModel },
                                columns: {
                                    columnVisibilityModel: {
                                        actions:
                                            is_desktop_app()
                                                || !entries.iter().any(|entry| can_install_extension(&entry)),
                                    },
                                },
                            }}
                            pageSizeOptions={[5, 10, 20]}
                            checkboxSelection
                            disableRowSelectionOnClick
                            style={{ cursor: "pointer" }}
                            onRowClick={(params) => {
                                let extension = params.row as ExtensionMarketplaceDetail;
                                let is_installed = installed_extensions
                                    .iter()
                                    .any(|installed| installed.id == extension.id && installed.namespace == namespace);
                                select_extension({ installed: is_installed, entry: extension });
                            }}
                            onRowSelectionModelChange={(new_selection) => {
                                selected_extension_ids.set(new_selection as Vec<String>);
                            }}
                            rowSelectionModel={selected_extension_ids}
                            data-testid="extension-list-entry"
                        />
                    </div>
                </Stack>
            );
        },
        [app_handle, analytics, enqueue_snackbar, entries, filter_text, installed_extensions, is_bulk_operating, selected_extension_ids, handle_install, handle_uninstall],
    );

    return render_component;
}
```