
## Rust HTTP template

The goal is to create a HTTP template engine that is converted into pure rust code
at compilation time, so to provide

  * high performance (no parsing during runtime, enable compiler optimization)
  * Compilation time, if you can run it, you're sure to not have "syntax" error like
messages during runtime
  * all the reason why we love rust (strong typing etc.)
  * permit the inclusion of raw rust code for corner cases or not-yet implemented tags


The main goal would be to integrate nicely with the Iron framework, and mid-term goal
would be to make it framework agnostic with maybe if necessary some "adaptor" code

## Compile it

you need to have cargo and the last version of rust

    cargo build


## Use it

you can find an example in `src/bin/test.rs`

```rust

#![feature(phase)]
#[phase(plugin, link)]
extern crate html_template;

html_template! hello_world {

<% template my_main_page() %>

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

    println!("{}", hello_world::my_main_page());
}

```

## Special Thanks

I would like to thanks Artyom, the guy behind the cppcms c++ web framework
for his templating engine that is converted into C++, he's the one who inspired
me to do that project for rust
