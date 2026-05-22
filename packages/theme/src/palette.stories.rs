```rust
use crate::theme::*;
use styled::*;

pub struct ThemePalette;

impl ThemePalette {
    pub fn render() -> impl FnOnce(&'static str) + 'static {
        move |bgcolor| {
            let theme = &get_default_theme();
            let color = if bgcolor.starts_with("grey") {
                theme.palette.grey[bgcolor.to_uppercase()]
            } else {
                theme.palette.background[bgcolor.to_uppercase()]
            };

            Box::new(format!(
                r#"
            <Stack height="100%" width="100%" flexWrap="wrap" padding={2} gap={6} bgcolor="{color}">
                <Stack direction="row" gap={6}>
                    <Stack gap={1}>
                        <Typography variant="overline">Palette</Typography>
                        <Stack direction="row" alignItems="center" gap={1}>
                            {["dark", "main", "light"].iter().map(|variant| {
                                Box::new(format!(
                                    r#"
                                        <Box
                                            display="flex"
                                            key={variant}
                                            width={32}
                                            alignItems="center"
                                            justifyContent="center">
                                                {variant}
                                            </Box>
                                        "#,
                                    )
                                ))
                            }).collect::<Vec<_>>()}
                        </Stack>
                        {["primary", "secondary", "error", "warning", "info", "success"].iter().map(|color| {
                            Box::new(format!(
                                r#"
                                        <Stack key={color} direction="row" alignItems="center" gap={1}>
                                            <Box
                                                display="flex"
                                                key={`${color}.${variant}`}
                                                width={32}
                                                height={32}
                                                bgcolor={`${color}.${variant}`}
                                                color={`${color}.contrastText`}
                                                alignItems="center"
                                                justifyContent="center">
                                                    Aa
                                                </Box>
                                            {color}
                                        </Stack>
                                    "#,
                                    )
                                ))
                            }).collect::<Vec<_>>()}
                    </Stack>

                    <Stack gap={1}>
                        <Typography variant="overline">Action</Typography>
                        {["hover", "focus", "selected", "disabled", "active"].iter().map(|color| {
                            Box::new(format!(
                                r#"
                                        <Stack direction="row" key={color} alignItems="center" gap={1}>
                                            <Box
                                                display="flex"
                                                width={32}
                                                height={32}
                                                bgcolor={`action.${color}`}
                                                alignItems="center"
                                                justifyContent="center">
                                                    Aa
                                                </Box>
                                                {color}
                                            </Stack>
                                        "#,
                                    )
                                ))
                            }).collect::<Vec<_>>()}
                    </Stack>

                    <Stack gap={1}>
                        <Typography variant="overline">Background</Typography>
                        {Object.keys(theme.palette.background).iter().map(|bgcolor| {
                            Box::new(format!(
                                r#"
                                        <Box
                                            display="flex"
                                            width={32}
                                            height={32}
                                            bgcolor={`background.${bgcolor}`}
                                            alignItems="center"
                                            justifyContent="center"
                                            border="1px solid"
                                            borderColor="divider"
                                        >
                                            Aa
                                        </Box>
                                        <Box
                                            display="flex"
                                            width={32}
                                            height={32}
                                            bgcolor={`background.${bgcolor}`}
                                            alignItems="center"
                                            justifyContent="center"
                                            boxShadow={8}
                                        >
                                            Aa
                                        </Box>
                                        {bgcolor}
                                    "#,
                                    )
                                ))
                            }).collect::<Vec<_>>()}
                    </Stack>
                </Stack>

                <Stack gap={1}>
                    <Typography variant="overline">Grey (with Divider border)</Typography>
                    <Stack gap={1} direction="row" alignItems="center">
                        {Object.keys(theme.palette.grey).iter().map(|key| {
                            Box::new(format!(
                                r#"
                                        <Box
                                            display="flex"
                                            width={32}
                                            height={32}
                                            bgcolor={`grey.${key}`}
                                            alignItems="center"
                                            justifyContent="center"
                                            border="1px solid"
                                            borderColor="divider"
                                        >
                                            Aa
                                        </Box>
                                        {key}
                                    "#,
                                    )
                                ))
                            }).collect::<Vec<_>>()}
                    </Stack>
                </Stack>
            </Stack>
        "#",
                bgcolor
            ))
        }
    }
}
```