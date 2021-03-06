use crate::request::prelude::*;
use twilight_model::invite::Invite;

#[derive(Default)]
struct GetInviteFields {
    with_counts: bool,
}

pub struct GetInvite<'a> {
    code: String,
    fields: GetInviteFields,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
}

impl<'a> GetInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            fields: GetInviteFields::default(),
            fut: None,
            http,
        }
    }

    pub fn with_counts(mut self) -> Self {
        self.fields.with_counts = true;

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetInvite {
                    code: self.code.clone(),
                    with_counts: self.fields.with_counts,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetInvite<'_>, Invite);
