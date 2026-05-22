```rust
use std::f64;
use reactome::{components::Stack, types::FormattedKeyValue};
use reactome::panels::DiagnosticStatusProps;

// component to display a single diagnostic status
pub fn DiagnosticTable(props: &DiagnosticStatusProps) -> JSX.Element {
    let { info, numeric_precision, onChange_split_fraction, open_sibling_panel, split_fraction = 0.5, topic_to_render } = props;

    // Render the header row with fixed column widths
    let table_header_row = (
        <TableRow style={{ height: 0 }}>
            <TableCell padding="none" style={{ width: format!("100%"), border_right: "none" }} />
            <TableCell padding="none" style={{ border_left: "none" }} />
        </TableRow>
    );

    // Render the key-value sections
    let render_key_value_sections = move || {
        let formatted_key_vals: Vec<FormattedKeyValue> = get_formatted_key_values(info.status);

        formatted_key_vals.iter().map(|kv| {
            let value_path = format!(
                "{}/status[:]{hardware_id==\"{}\"}{name==\"{}\"}.values[:]{key==\"{}\"}.value",
                topic_to_render,
                info.status.hardware_id,
                info.status.name,
                kv.key
            );

            let open_plot_panel_icon_elem: Option<JSX.Element> = if !kv.value.is_empty() {
                if kv.value.parse::<f64>().is_ok() {
                    Some(
                        <IconButton
                            className="icon-button"
                            title="Open in Plot panel"
                            color="inherit"
                            size="small"
                            data-testid="open-plot-button"
                            onClick={
                                move |_| open_sibling_panel(&open_sibling_panel, value_path.clone())
                            }
                        >
                            <ShowChartIcon fontSize="inherit" />
                        </IconButton>,
                    )
                } else {
                    Some(
                        <IconButton
                            className="icon-button"
                            title="Open in State Transitions panel"
                            color="inherit"
                            size="small"
                            data-testid="open-state-transitions-button"
                            onClick={
                                move |_| open_sibling_panel(&open_sibling_panel, value_path.clone())
                            }
                        >
                            <PowerInputIcon fontSize="inherit" />
                        </IconButton>,
                    )
                }
            } else {
                None
            };

            (
                <TableRow key={kv.key} hover>
                    <TableCell colSpan={2} padding="checkbox">
                        <Stack
                            direction="row"
                            flex="auto"
                            alignItems="center"
                            justifyContent="space-between"
                            gap={1}
                        >
                            <Typography
                                flex="auto"
                                color={MESSAGE_COLORS[info.status.level]}
                                variant="inherit"
                                fontWeight={800}
                            >
                                {kv.key}
                            </Typography>
                            {open_plot_panel_icon_elem}
                        </Stack>
                    </TableCell>
                </TableRow>,
            )
        })
    };

    // Render the table
    <div>
        <div
            className="resize-handle"
            style={{
                left: format!("{}%", 100.0 * split_fraction),
            }}
            onMouseDown={props.resizeMouseDown}
            data-testid="DiagnosticTable-resizeHandle"
        />
        <Table className="table" size="small">
            <TableBody>
                {table_header_row}
                {render_key_value_sections()}
            </TableBody>
        </Table>
    </div>
}
```