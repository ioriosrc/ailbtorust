```rust
use std::fmt::{self, Display};
use actix_web::{web, HttpResponse, Responder};
use crate::routes::{AppEvent, UserData};

#[derive(Debug)]
enum CurrentUserType {
    Unauthenticated,
    AuthenticatedFree,
    AuthenticatedTeam,
    AuthenticatedEnterprise,
}

impl Display for CurrentUserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CurrentUserType::Unauthenticated => write!(f, "unauthenticated"),
            CurrentUserType::AuthenticatedFree => write!(f, "authenticated-free"),
            CurrentUserType::AuthenticatedTeam => write!(f, "authenticated-team"),
            CurrentUserType::AuthenticatedEnterprise => write!(f, "authenticated-enterprise"),
        }
    }
}

#[derive(Debug)]
struct SidebarItem {
    id: String,
    title: String,
    text: String,
    actions: Option<Vec<ButtonAction>>,
}

impl Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n\tid: \"{}\",\n\ttitle: \"{}\",\n\ttext: \"{}\",\n\tactions: {:?}\n}}",
            self.id,
            self.title,
            self.text,
            self.actions.as_ref()
                .map(|actions| actions.iter().map(String::from).collect::<Vec<String>>())
        )
    }
}

#[derive(Debug)]
struct ButtonAction {
    href: String,
    target: String,
    variant: String,
    onClick: fn(&CurrentUserType) -> Option<AppEvent>,
}

impl Display for ButtonAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n\thref: \"{}\",\n\ttarget: \"{}\",\n\tvariant: \"{}\",\n\tonClick: {:?}\n}}",
            self.href,
            self.target,
            self.variant,
            self.onClick
                .as_ref()
                .map(|action| action(CurrentUserType::Unauthenticated))
        )
    }
}

async fn sidebar_items(
    user_data: web::Data<UserData>,
) -> impl Responder {
    let current_user_type = &user_data.user_type;
    let analytics = &user_data.analytics;

    let sidebar_items = match current_user_type {
        CurrentUserType::Unauthenticated => vec![
            SidebarItem {
                id: "new".to_string(),
                title: t!("newToLichtblick"),
                text: t!("newToLichtblickDescription"),
                actions: Some(vec![
                    ButtonAction {
                        href: LICHTBLICK_DOCUMENTATION_LINK.to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_SELECT_VIEW),
                    },
                    ButtonAction {
                        href: "https://console.foxglove.dev/recordings"
                            .to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_CLICK_CTA),
                    },
                ]),
            },
        ],
        CurrentUserType::AuthenticatedFree => vec![
            SidebarItem {
                id: "new".to_string(),
                title: t!("newToLichtblick"),
                text: t!("newToLichtblickDescription"),
                actions: Some(vec![
                    ButtonAction {
                        href: LICHTBLICK_DOCUMENTATION_LINK.to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_SELECT_VIEW),
                    },
                    ButtonAction {
                        href: "https://foxglove.dev/tutorials"
                            .to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_CLICK_CTA),
                    },
                ]),
            },
        ],
        CurrentUserType::AuthenticatedTeam => vec![
            SidebarItem {
                id: "start-collaborating".to_string(),
                title: t!("startCollaborating"),
                text: t!("startCollaboratingDescription"),
                actions: Some(vec![
                    ButtonAction {
                        href: "https://console.foxglove.dev/recordings"
                            .to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_CLICK_CTA),
                    },
                    ButtonAction {
                        href: "https://docs.foxglove.dev/docs/visualization/layouts#team-layouts"
                            .to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_CLICK_CTA),
                    },
                ]),
            },
        ],
        CurrentUserType::AuthenticatedEnterprise => vec![
            SidebarItem {
                id: "start-collaborating".to_string(),
                title: t!("startCollaborating"),
                text: t!("startCollaboratingDescription"),
                actions: Some(vec![
                    ButtonAction {
                        href: "https://console.foxglove.dev/recordings"
                            .to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_CLICK_CTA),
                    },
                    ButtonAction {
                        href: "https://docs.foxglove.dev/docs/visualization/layouts#team-layouts"
                            .to_string(),
                        target: "_blank".to_string(),
                        variant: "outlined",
                        onClick: Some(|_: &CurrentUserType| AppEvent::DIALOG_CLICK_CTA),
                    },
                ]),
            },
        ],
    };

    HttpResponse::Ok().json(sidebar_items)
}
```