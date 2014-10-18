#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {

<% template render() %>

<html>
    <head>
    </head>
    <body>
        hello world
    </body>
</html>

<% end template %>

}


fn main () {

    println!("{}", hello_world::render());
}
