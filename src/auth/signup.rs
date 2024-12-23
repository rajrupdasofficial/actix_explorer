use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use regex::Regex;

use crate::db::db::establish_connection;
use crate::models::models::{SignupRequest, User};
use crate::schema::schema::users;

pub async fn signup(signup_data: web::Json<SignupRequest>) -> impl Responder {
    // Validation checks
    if signup_data.password != signup_data.confirm_password {
        return HttpResponse::BadRequest().json("Passwords do not match");
    }

    if signup_data.password.len() < 8 {
        return HttpResponse::BadRequest().json("Password must be at least 8 characters");
    }

    // Email validation
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(&signup_data.email) {
        return HttpResponse::BadRequest().json("Invalid email format");
    }

    let connection = &mut establish_connection();

    // Check if email already exists
    let existing_user = users::table
        .filter(users::email.eq(&signup_data.email))
        .first::<User>(connection)
        .optional();

    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json("Email already registered");
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json("Database error");
        }
        Ok(None) => {}
    }

    // Hash password
    let password_hash = match hash(&signup_data.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json("Password hashing failed");
        }
    };

    // Create new user
    let new_user = SignupRequest {
        name: signup_data.name.clone(),
        email: signup_data.email.clone(),
        password: password_hash,
        confirm_password: password_hash.clone(),
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Created().json("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}
