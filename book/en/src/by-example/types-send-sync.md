# Types, Send and Sync

The `app` attribute injects a context, a collection of variables, into every
function. All these variables have predictable, non-anonymous types so you can
write plain functions that take them as arguments.

The API reference specifies how these types are generated from the input. You
can also generate documentation for you binary crate (`cargo doc --bin <name>`);
in the documentation you'll find `Context` structs (e.g. `init::Context` and
`idle::Context`) whose fields represent the variables injected into each
function.

The example below shows the different types generates by the `app` attribute.

``` rust
{{#include ../../../../examples/types.rs}}
```

## `Send`

[`Send`] is a marker trait for "types that can be transferred across thread
boundaries", according to its definition in `core`. In the context of RTFM the
`Send` trait is only required where it's possible to transfer a value between
tasks that run at *different* priorities. This occurs in a few places: in message
passing, in shared `static mut` resources and in the initialization of late
resources.

[`Send`]: https://doc.rust-lang.org/core/marker/trait.Send.html

The `app` attribute will enforce that `Send` is implemented where required so
you don't need to worry much about it. It's more important to know where you do
*not* need the `Send` trait: on types that are transferred between tasks that
run at the *same* priority. This occurs in two places: in message passing and in
shared `static mut` resources.

The example below shows where a type that doesn't implement `Send` can be used.

``` rust
{{#include ../../../../examples/not-send.rs}}
```

It's important to note that late initialization of resources is effectively a
send operation where the initial value is sent from `idle`, which has the lowest
priority of `0`, to a task with will run with a priority greater than or equal
to `1`. Thus all late resources need to implement the `Send` trait.

Sharing a resource with `init` can be used to implement late initialization, see
example below. For that reason, resources shared with `init` must also implement
the `Send` trait.

``` rust
{{#include ../../../../examples/shared-with-init.rs}}
```

## `Sync`

Similarly, [`Sync`] is a marker trait for "types for which it is safe to share
references between threads", according to its definition in `core`. In the
context of RTFM the `Sync` trait is only required where it's possible for two,
or more, tasks that run at different priority to hold a shared reference to a
resource. This only occurs with shared `static` resources.

[`Sync`]: https://doc.rust-lang.org/core/marker/trait.Sync.html

The `app` attribute will enforce that `Sync` is implemented where required but
it's important to know where the `Sync` bound is not required: in `static`
resources shared between tasks that run at the *same* priority.

The example below shows where a type that doesn't implement `Sync` can be used.

``` rust
{{#include ../../../../examples/not-sync.rs}}
```
