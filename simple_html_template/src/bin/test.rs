#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {

<% template plop() %>

<html>
    <head>
    </head>
    <body>
        <% rust prout %>
        hello world
    </body>
</html>

<% end template %>

}


fn main () {

    println!("{}", hello_world::plop());
}
