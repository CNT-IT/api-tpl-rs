use salvo::{async_trait, writing::Json, Depot, FlowCtrl, Handler, Request, Response};

use crate::internal::{result::code::Code, util::identity::Identity};

use super::auth_check;

pub struct Auth;

impl Auth {
    #[inline]
    pub fn new() -> Self {
        Auth {}
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Handler for Auth {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        resp: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let empty = Identity::empty();
        let id = req.extensions().get::<Identity>().unwrap_or(&empty);
        if let Err(e) = auth_check(id).await {
            resp.render(Json(Code::ErrAuth(Some(e.to_string())).to_reply()));
            ctrl.skip_rest();
            return;
        }
    }
}
