```rust
use react::prelude::*;

use super::{TableCellProps, sanitize_accessor_path};
use crate::table::types::CellValue;

const ICON_SIZE: f32 = 16.0;

#[function_component]
pub fn TableCell(props: PropsWithChildren<TableCellProps>) -> Html {
    let { row, accessor_path } = props;
    let [is_expanded, set_is_expanded] = use_state(false);
    let toggle_is_expanded = use_callback(|| set_is_expanded(!is_expanded), []);

    if row.is_expanded() || is_expanded {
        html! {
            <div>
                {if is_expanded {
                    html! {
                        <IconButton size="small" onClick={toggle_is_expanded} class="icon-button">
                            <MinusIcon fontSize="small" />
                        </IconButton>
                    }
                } else {
                    // Render the default cell content
                    render_cell_content(accessor_path, row)
                }}
            </div>
        }
    } else {
        html! {
            <span
                class="object-cell"
                data-testid={`expand-cell-${sanitize_accessor_path(accessor_path)}-${row.index}`}
                onClick={toggle_is_expanded}
            >
                Object
            </span>
        }
    }
}

fn render_cell_content(accessor_path: &str, row: &Row<CellValue>) -> Html {
    // Implement the logic to render the cell content based on the accessor path and data in the row
    html! { /* Your implementation here */ }
}
```