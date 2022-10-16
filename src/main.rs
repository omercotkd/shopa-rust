use rocket;
use shopa_rust;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = shopa_rust::rocket().await.launch().await?;

    Ok(())
}
