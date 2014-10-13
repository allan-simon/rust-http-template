#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {
<% template render() %>

plop plop tatata

<% end template %>
}

fn main () {

}
