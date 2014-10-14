#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {
<% template render() %>

plop plop tatata < dedd fn() < >


<% end template %>
}

fn main () {

}
