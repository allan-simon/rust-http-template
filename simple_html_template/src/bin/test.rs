#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {

<% template plop() %>

<html>
    <head>
    </head>
    <body>
        <% rust out.push_str(body().as_slice()); %>
    </body>
</html>

<% end template %>

/// generate the inside of <body>
///
///
<% template body() %>
    <% rust for _ in range(0,10u) { %>
        hello world
    <% rust } %>
<%end template %>
}


fn main () {

    println!("{}", hello_world::plop());
}
