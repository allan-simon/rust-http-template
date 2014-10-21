#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {

<% template plop() %>

<html>
    <head>
    </head>
    <body>
        This :
        <% rust out.push_str(body(5).as_slice()); %>
        Is equivalent to that
        <% include body(5) %>
    </body>
</html>

<% end template %>

/// generate the inside of <body>
///
///
<% template body(number: uint) %>
    <% rust for _ in range(0, number) { %>
        hello world
    <% rust } %>
<%end template %>
}


fn main () {

    println!("{}", hello_world::plop());
}
