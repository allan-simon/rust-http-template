#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {
<% template render() %>


<% end template %>
}

fn main () {

}
