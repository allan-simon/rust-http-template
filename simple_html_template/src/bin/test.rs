#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {

<% template plop() %>

<html>
    <head>
    </head>
    <body>
        <% rust for _ in range(0,10u) { %>
            hello world
        <% rust } %>
    </body>
</html>

<% end template %>

}


fn main () {

    println!("{}", hello_world::plop());
}
