#[macro_use] extern crate rocket;

#[get("/")]
fn home() ->  &'static str {
    "Home de la pagina"
}

#[get("/acerca")]
fn acerca() -> &'static str {
    "Acerca de mi"
}


#[get("/perfil")]
fn perfil() -> &'static str {
    "Mi perfil"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![home, acerca, perfil])
}
