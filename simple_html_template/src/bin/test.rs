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
    <% if number > 4 %>
        woow

    <% end if %>

    <% rust for _ in range(0, number) { %>
        hello world <%= number %>

    <% rust } %>
<%end template %>
}


fn main () {

    println!("{}", hello_world::plop());
}
