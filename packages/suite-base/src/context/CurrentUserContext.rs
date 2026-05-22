```rust
use std::rc::{Rc, RefCell};
use std::cell::Ref;

type User = Rc<RefCell<UserData>>;
type UserData = {
  id: String,
  avatar_image_url: Option<String>,
  email: String,
  org_id: String,
  org_display_name: Option<String>,
  org_slug: String,
  org_paid: Option<bool>,
  org: OrgData,
};

type OrgData = {
  id: String,
  slug: String,
  display_name: String,
  is_enterprise: bool,
  allows_uploads: bool,
  supports_edge_sites: bool,
};

type CurrentUserContextType = (Option<User>, impl FnOnce() -> (), Option<impl FnOnce()>);

pub struct CurrentUserContext {
    user: Rc<RefCell<User>>,
    sign_in: Option<Box<dyn FnOnce()>>,
    sign_out: Option<Box<dyn FnOnce() -> Result<(), String>>>,
}

impl CurrentUserContext {
    pub fn new(user: User) -> Self {
        CurrentUserContext {
            user,
            sign_in: None,
            sign_out: None,
        }
    }

    pub fn set_sign_in(&mut self, sign_in: impl FnOnce() + 'static) {
        self.sign_in = Some(Box::new(sign_in));
    }

    pub fn set_sign_out(&mut self, sign_out: impl FnOnce() -> Result<(), String> + 'static) {
        self.sign_out = Some(Box::new(sign_out));
    }
}

pub fn use_current_user() -> (Option<User>, impl FnOnce() -> (), Option<impl FnOnce()>) {
    let user_context = Rc::new(RefCell::new(CurrentUserContext {
        user: User::new(Rc::new(RefCell::new(UserData::default()))),
        sign_in: None,
        sign_out: None,
    }));

    return (
        user_context.clone(),
        move || {
            user_context.borrow_mut().user.borrow_mut().avatar_image_url = Some("https://example.com/avatar.png");
        },
        move || {
            user_context.borrow_mut().sign_in.take();
        },
    );
}

pub fn use_current_user_type() -> String {
    let (user, _, _) = use_current_user();

    if user.is_none() {
        return "unauthenticated";
    }

    if user.as_ref().borrow().org.is_enterprise {
        return "authenticated-enterprise";
    }

    if user.as_ref().borrow().org_paid.unwrap_or(false) {
        return "authenticated-team";
    }

    return "authenticated-free";
}

// ts-prune-ignore-next
pub fn default_current_user_context() -> CurrentUserContextType {
    (None, Box::new(|| {}), Box::new(|| Ok(())))
}
```