use actix_multipart::Multipart;
use actix_web::{post, HttpResponse, Error};
use futures::StreamExt;
use sea_orm::prelude::Uuid;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;
//use std::io::{self};


/// Handles file upload via multipart form data.
///
/// This endpoint accepts multipart form data containing files and saves them to the server.
/// The uploaded files must have allowed MIME types specified in the `allowed_mime_types` list.
/// Each uploaded file is saved with a unique filename in the `uploads` directory.
///
/// # Parameters
///
/// - `payload`: Multipart form data containing files to upload.
///
/// # Returns
///
/// Returns an `HttpResponse` indicating the success or failure of the upload operation.
///
/// - If the upload is successful, returns an `HttpResponse` with status code 200 OK.
/// - If the upload fails due to invalid content type or other errors, returns an `HttpResponse`
///   with an appropriate error message and status code 400 Bad Request.
///
/// # Example
///
/// ```
/// use actix_web::{test, App};
///
/// #[actix_rt::test]
/// async fn test_upload_file() {
///     let mut app = test::init_service(App::new().route("/upload_file", upload_file)).await;
///     
///     // Perform test request with multipart form data containing a file
///     // and assert the response.
/// }
/// ```
#[post("/upload_file")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {

            let upload_dir = "uploads";
            let allowed_mime_types: Vec<(&str, &str)> = vec![
                ("image/jpeg", "jpg"),
                ("image/png", "png"),
            ];

            if !Path::new(upload_dir).exists() {
                fs::create_dir(upload_dir).expect("Unable to create directory");
            }

        // Iterate over multipart stream
        while let Some(item) = payload.next().await {
                
            let mut field = match item {
                    Ok(field) => field,
                    Err(e) => return Err(Error::from(e)),
            };


            let  file_type = field.content_type();
            if file_type.is_none() {
                return Ok(HttpResponse::BadRequest().body("Invalid content type"));
            }

            // print!("File type: {:?}", file_type.unwrap().to_string());
            // io::stdout().flush().unwrap();


            // Find corresponding file extension for the content type
            let ext = match allowed_mime_types.iter().find(|(ct, _)| *ct == &file_type.unwrap().to_string()) {
                Some((_, ext)) => ext,
                None => {
                    return Ok(HttpResponse::BadRequest().body("File type not allowed"));
                },
            };

            let filename = format!("img_{}.{}", Uuid::new_v4(), ext);
            let filepath = format!("{}/{}", upload_dir, filename);

            // Create a file to save the uploaded content
            let mut file = match File::create(&filepath) {
                    Ok(file) => file,
                    Err(e) => return Err(Error::from(e)),
            };

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(data) => data,
                        Err(e) => return Err(Error::from(e)),
                    };
                    file.write_all(&data)?;
            }
        }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}
