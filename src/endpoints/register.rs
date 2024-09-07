use actix_web::{
    get, post,
    web::{self, Data, Form},
    HttpResponse,
};
use askama::Template;
use deadpool_redis::Pool;
use log::{debug, error, info};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    auth,
    endpoints::templates::{ErrorPage, RegisterPage},
    models::mongo::{MongoRepo, User},
    utils::emails::send_multipart_email,
};

use super::templates::Index;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateNewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[get("/registration")]
pub async fn registration() -> HttpResponse {
    info!("Rendering registration page");
    let template = RegisterPage {
        title: "Registration",
    };

    let body = match template.render() {
        Ok(body) => body,
        Err(err) => {
            error!("Error rendering template: {err:#?}");
            let template = ErrorPage {
                title: "Login Error",
                code: 500,
                message: "Invalid username or password",
                error: &err.to_string(),
            };

            let body = match template.render() {
                Ok(body) => body,
                Err(err) => {
                    error!("Error rendering template: {err:#?}");
                    return HttpResponse::InternalServerError().finish();
                }
            };

            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(body);
        }
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

#[post("/register")]
pub async fn register(
    pool: Data<MongoRepo>,
    Form(new_user): Form<User>,
    redis_pool: Data<Pool>,
) -> HttpResponse {
    // new_user.password = hash_pw(&new_user.password.as_bytes()).await;

    let user_id = match pool.create_user(new_user.clone()).await {
        Ok(user_id) => {
            info!("User created successfully");
            user_id.inserted_id
        }
        Err(err) => {
            error!("Error creating user: {err}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut redis_conn = redis_pool
        .get()
        .await
        .map_err(|err| {
            error!("Error getting redis connection: {err}");
            let error = ErrorPage {
                title: "Internal Server Error",
                code: 500,
                message: "Unable to activate your account at this time. Please try again later.",
                error: &err.to_string(),
            };

            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(error.render().expect("Error rendering template"))
        })
        .expect("Error getting redis connection");

    send_multipart_email(
        String::from("AJ's study site - Let's get you verified"),
        user_id.as_object_id().expect("Error getting object id"),
        new_user.email.clone(),
        new_user.first_name.clone(),
        new_user.last_name.clone(),
        "verification_email.html",
        &mut redis_conn,
    )
    .await
    .expect("Error sending email");

    HttpResponse::Ok().content_type("text/html").body(
        "<h1>Registration successful</h1> <p>Please check your email to verify your account</p>",
    )
}

#[derive(Deserialize, Serialize, Debug)]
struct Parameters {
    token: String,
}

#[get("/register/confirm")]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: Data<MongoRepo>,
    redis_pool: Data<Pool>,
) -> HttpResponse {
    info!("Register confirm endpoint hit");

    debug!("Getting redis connection");
    let mut redis_conn = redis_pool
        .get()
        .await
        .map_err(|err| {
            error!("Error getting redis connection: {err}");
            let error = ErrorPage {
                title: "Internal Server Error",
                code: 500,
                message: "Unable to activate your account at this time. Please try again later.",
                error: &err.to_string(),
            };

            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(error.render().expect("Error rendering template"))
        })
        .expect("Error getting redis connection");

    info!("Verifying token");
    let confirmation_token = match auth::tokens::verify_confirmation_token_pasetor(
        &parameters.token.clone(),
        &mut redis_conn,
        None,
    )
    .await
    {
        Ok(token) => token,
        Err(err) => {
            error!("Error verifying token: {err}");
            let error = ErrorPage {
                title: "Internal Server Error",
                code: 500,
                message: "Unable to activate your account at this time. Please try again later.",
                error: &err,
            };

            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(error.render().expect("Error rendering template"));
        }
    };

    info!("Activating user");
    match activate_new_user(&pool, confirmation_token.user_id).await {
        Ok(()) => {
            info!("User activated successfully");
            let template = Index { title: "Home" };

            let body = template.render().expect("Error rendering template");

            HttpResponse::Ok().content_type("text/html").body(body)
        }

        Err(err) => {
            error!("Error activating user: {err}");
            let error = ErrorPage {
                title: "Internal Server Error",
                code: 500,
                message: "Unable to activate your account at this time. Please try again later.",
                error: &err.to_string(),
            };

            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(error.render().expect("Error rendering template"))
        }
    }
}

async fn activate_new_user(
    pool: &MongoRepo,
    user_id: ObjectId,
) -> Result<(), mongodb::bson::extjson::de::Error> {
    info!("Activate new user method hit");
    match pool.get_user(Some(user_id), None).await {
        Ok(mut user) => {
            debug!("User found");
            user.is_active = Some(true);
            pool.update_user(user_id, user)
                .await
                .expect("Error activating user");
        }
        Err(err) => {
            error!("Marking user active: {err}");
            return Err(err);
        }
    }
    Ok(())
}
