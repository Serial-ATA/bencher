use bencher_json::JsonAuth;
use bencher_json::JsonLogin;

use dropshot::{endpoint, HttpError, RequestContext, TypedBody};
use http::StatusCode;
use slog::Logger;

use crate::endpoints::endpoint::CorsResponse;
use crate::endpoints::endpoint::Post;
use crate::endpoints::endpoint::ResponseAccepted;
use crate::endpoints::Endpoint;

use crate::error::issue_error;
use crate::{
    context::{ApiContext, Body, ButtonBody, Message},
    model::user::QueryUser,
};

use super::AUTH_TOKEN_TTL;
use super::TOKEN_ARG;

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/auth/login",
    tags = ["auth"]
}]
pub async fn auth_login_options(
    _rqctx: RequestContext<ApiContext>,
) -> Result<CorsResponse, HttpError> {
    Ok(Endpoint::cors(&[Post.into()]))
}

#[endpoint {
    method = POST,
    path = "/v0/auth/login",
    tags = ["auth"]
}]
pub async fn auth_login_post(
    rqctx: RequestContext<ApiContext>,
    body: TypedBody<JsonLogin>,
) -> Result<ResponseAccepted<JsonAuth>, HttpError> {
    let json = post_inner(&rqctx.log, rqctx.context(), body.into_inner()).await?;
    Ok(Post::pub_response_accepted(json))
}

async fn post_inner(
    log: &Logger,
    context: &ApiContext,
    json_login: JsonLogin,
) -> Result<JsonAuth, HttpError> {
    let conn = &mut *context.conn().await;

    let query_user = QueryUser::get_with_email(conn, &json_login.email)?;
    query_user.check_is_locked()?;
    if let Some(invite) = &json_login.invite {
        query_user.accept_invite(conn, &context.token_key, invite)?;
    }

    let token = context
        .token_key
        .new_auth(json_login.email.clone(), AUTH_TOKEN_TTL)
        .map_err(|e| {
            issue_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create auth JWT at login",
                &format!(
                    "Failed failed to create auth JWT ({json_login:?} | {AUTH_TOKEN_TTL}) at login"
                ),
                e,
            )
        })?
        .to_string();

    let body = Body::Button(Box::new(ButtonBody {
        title: "Confirm Bencher Login".into(),
        preheader: "Click the provided link to login.".into(),
        greeting: format!("Ahoy {},", query_user.name),
        pre_body: "Please, click the button below or use the provided code to login to Bencher."
            .into(),
        button_text: "Confirm Login".into(),
        button_url: context
            .endpoint
            .clone()
            .join("/auth/confirm")
            .map(|mut url| {
                #[cfg(feature = "plus")]
                if let Some(plan) = json_login.plan {
                    url.query_pairs_mut()
                        .append_pair(super::PLAN_ARG, plan.as_ref());
                }
                url.query_pairs_mut().append_pair(TOKEN_ARG, &token);
                url.into()
            })
            .unwrap_or_default(),
        clipboard_text: "Confirmation Code".into(),
        clipboard_target: token,
        post_body: String::new(),
        closing: "See you soon,".into(),
        signature: "The Bencher Team".into(),
        settings_url: context
            .endpoint
            .clone()
            .join("/help")
            .map(Into::into)
            .unwrap_or_default(),
    }));
    let message = Message {
        to_name: Some(query_user.name.into()),
        to_email: query_user.email.into(),
        subject: Some("Confirm Bencher Login".into()),
        body: Some(body),
    };
    context.messenger.send(log, message);

    Ok(JsonAuth {
        email: json_login.email,
    })
}
