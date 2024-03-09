use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Error};
use sea_orm::Set;
use sea_orm::DatabaseConnection;
use crate::models::contact_model::Entity as ContactEntity;
use crate::models::contact_model::ActiveModel as ContactActiveModel;
use crate::models::contact_model::Model as ContactModel;
use url::Url;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::ActiveModelTrait;




/// `contacts_index` is a GET endpoint that retrieves all contacts.
/// It returns a JSON array of contacts.
#[get("/contacts")]
async fn contacts_index(pool: web::Data<DatabaseConnection>) -> impl Responder {

    let contacts: Vec<serde_json::Value> = ContactEntity::find()
    .into_json()
    .all(&pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(contacts)
}

/// `contacts_show` is a GET endpoint that retrieves a contact by its ID.
/// It returns a JSON object of the contact if found, otherwise it returns a 404 error.
#[get("/contacts/{id}")]
async fn contacts_show(id: web::Path<i32>, pool: web::Data<DatabaseConnection>) -> impl Responder {
   
    let contact: Option<serde_json::Value> = ContactEntity::find_by_id(id.into_inner())
    .into_json()
    .one(&pool)
    .await
    .unwrap();

    match contact {
        Some(contact) => HttpResponse::Ok().json(contact),
        None => HttpResponse::NotFound().body("Contact not found."),
    }
}

/// `contacts_store` is a POST endpoint that creates a new contact.
/// It accepts a JSON object of the contact data and returns a 201 status code upon successful creation.
#[post("/contacts")]
async fn contacts_store(form: web::Json<ContactModel>, pool: web::Data<DatabaseConnection>) -> impl Responder {
    
    let mut photo = None;
    if form.photo.as_ref().and_then(|s| Url::parse(s).ok()).is_some() {
        photo = form.photo.to_owned();
    }

    ContactActiveModel {
        name: Set(form.name.to_owned()),
        email: Set(form.email.to_owned()),
        phone: Set(form.phone.to_owned()),
        address: Set(form.address.to_owned()),
        city: Set(form.city.to_owned()),
        photo: Set(photo),
        ..Default::default()
    }
    .save(&pool)
    .await
    .unwrap();

    HttpResponse::Created().body("Contact Created.")
}

/// `contacts_update` is a PUT endpoint that updates a contact by its ID.
/// It accepts a JSON object of the new contact data and returns a 200 status code upon successful update.
#[put("/contacts/{id}")]
async fn contacts_update(id: web::Path<i32>, form: web::Json<ContactModel>, pool: web::Data<DatabaseConnection>) -> impl Responder {
    
    let contact: Option<ContactModel> = ContactEntity::find_by_id(id.into_inner()).one(&pool).await.unwrap();
    let mut contact: ContactActiveModel = contact.unwrap().into();
        contact.name = Set(form.name.to_owned());
        contact.email = Set(form.email.to_owned());
        contact.phone = Set(form.phone.to_owned());
        contact.address = Set(form.address.to_owned());
        contact.city = Set(form.city.to_owned());

        if form.photo.as_ref().and_then(|s| Url::parse(s).ok()).is_some() {
            contact.photo = Set(form.photo.to_owned());
        }
    
        contact.update(&pool).await.unwrap();

    HttpResponse::Ok().body("Contact Updated.")
}


/// `contacts_destroy` is a DELETE endpoint that deletes a contact by its ID.
/// It returns a 200 status code upon successful deletion.
#[delete("/contacts/{id}")]
async fn contacts_destroy(id: web::Path<i32>, pool: web::Data<DatabaseConnection>) -> impl Responder {
    let contact: Option<ContactModel> = ContactEntity::find_by_id(id.into_inner()).one(&pool).await.unwrap();
    let contact: ContactModel = contact.unwrap();
    ContactEntity::delete(contact.into_active_model())
    .exec(&pool)
    .await
    .unwrap();

    HttpResponse::Ok().body("Contact Deleted.")
}






