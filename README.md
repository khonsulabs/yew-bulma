# yew-bulma

[![crate version](https://img.shields.io/crates/v/yew-bulma.svg)](https://crates.io/crates/yew-bulma)

This crate provides simple [Yew](https://yew.rs/) components that render [Bulma](https://bulma.io/)-compatible DOM nodes. For example:

```rust
html! {
    <Button
        label="Save"
        disabled=!can_save
        css_class="is-primary"
        action=edit_form.link.callback(|e: web_sys::MouseEvent| {e.prevent_default(); Message::Save})
        processing=edit_form.is_saving
    />
}
```

The goals of this crate are:

- Provide easy accessors for common Bulma components, including Rust-native implementations of logic.
- Support for [fluent](https://www.projectfluent.org/)-based localization.
- Markdown rendering support.

This project is very early in development and is really only being added as needed for projects for [Khonsu Labs](https://khonsulabs.com/).
