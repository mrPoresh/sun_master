### Sun Master

`Rust multi threards http server`

This repo contains a http server is writing using standart language features (std).
This server can handle two types of reqests:

---> http://127.0.0.1:7878

---> http://127.0.0.1:7878/"anything"

In the first case, upon a succesful reqest, the server will return the html code of the 
"auth" page. On any other request, the server will return an error page.

### Getting Started

The first step will be, if you don't have Rust installed, install it.
Please folow the official guide:

---> https://www.rust-lang.org/tools/install

Then, write in console

---> git clone https://github.com/mrPoresh/sun_master.git

And the last step to run, write in console

---> cargo run