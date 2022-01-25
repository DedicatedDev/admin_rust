
## Installation in Local

* `$ git clone https://github.com/DedicatedDev/admin_rust.git`
* `$ cd admin_rust` 
* `$ cargo run`

## Distritution to Heroku

* `$ heroku create --buildpack https://github.com/emk/heroku-buildpack-rust.git`
* `$ git remote add heroku https://git.heroku.com/<heroku-project-name>.git`
* `$ echo "web: ROCKET_PORT=$PORT ROCKET_ENV=prod ./target/release/sbadmin_rust" > Procfile`
* `$ echo "VERSION=nightly" > RustConfig`
* `$ git add . && git commit -m "Add Heroku deploy configuration"`
* `$ git push heroku master`

