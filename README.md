# What is Rust Pages?
Rust Pages is a way to generate a static website with Rust. It allows you to structure your data using Rust’s type system. Then write a view function that converts into static HTML. The goal is to create a system that will allow you to write static websites using Rust and leveraging Rust’s type system and compile checker to to build static sites that follow rules that you define by deciding how your data is structured.

It is inspired by the [Elm Pages Project](https://elm-pages.com/) by [Dillon Kearns](https://dillonkearns.com/). It is also inspired by the [Iced.rs project](), which was in turn inspired by Elm. As much as possible I will try to keep the syntax for building pages the same as with Iced.rs so that there is a single consistent, elegant way to build both applications and static websites in Rust (which means less learning).

The following is an example that might look a little like [the books pages on my website](https://sashinexists.com/recommended)

You could start with the custom type Book:

```rust
struct Book {
	title: String,
	author: String,
	isbn: i32,
	description: String,
	cover_image_url: String
}
```

You want to build a web page that lists all your favourite books. Your data model is a list of books.

```rust
struct Model {
	books: Vec<Book>
}
```

You could grab data from Goodreads, Google Books or another API or database and parse them into this model.

You would then be able to write a view that takes in you model and generates the HTML.

```rust
fn view(model: Model) -> Element {
    column()
        .push(row().push(heading("My Favourite Books")))
        .push(row().push(view_books(&model.books)))
}

fn view_books(books: &Vec<Book>) -> Element {
    books
        .iter()
        .fold(column(), |output, book| output.push(view_book(book)))
}

fn view_book(book: &Book) -> Element {
    column()
        .push(row().push(heading(&book.title)))
        .push(row().push(subheading(&book.author)))
        .push(
            row()
                .push(column().push(image(&book.cover_image_url)))
                .push(column().push(text(&book.description))),
        )
}
```

The example here would only generate a single page.

The full scope of the idea would be to generate a full fledged static site with shared elements between pages (header, footer, title, navigation etc), styling (as an external stylesheet), routes (different html pages generated according to a defined routes type).

For example, you could flesh out an advanced personal website. The model you use could include a list of your favourite music, quotes, movies, books and games. You would create structs that represent this data and you could parse the data from various APIs into these structs. A view each page would have a view method that would take in a model and render the relevant content. There could be a home page, a page for lists of movies, books, etc and a route for an individual movie, book etc.

The plan is to build this as a cli app using [clap](https://crates.io/crates/clap) and have it available with to install with cargo.

```zsh
cargo install pages
```

When installed you would run:

```zsh
pages new personal_site
```

This will create a new rust project with all the pages boilerplate. It may ask the user questions about setting up the site and generate the boilerplate accordingly.

When you want to create a new page you would use a command like:

```zsh
pages add music
```

This would add all the boilerplate needed to add a static route for new page. This page could be used for a list of your favourite music.

You could then do:

```zsh
pages add music/song_
```

The underscore here means it is a *dynamic route*. This could be used to create a page for a specific song.

I would probably borrow the syntax for these routes from Elm pages: https://elm-pages.com/docs/file-based-routing

The idea here is that it would allow you to create an entire static site using Rust. This would leverage Rust’s smart type system and compile checker. It aims to be a delightful way to quickly build static websites with all the power and flexibility of Rust.

The workflow would look like this:

Parse data (from whatever source) into Rusts’ Datatypes → Write functions that transform the data to the UI

As developers we tend to need to have build websites to showcase our work and make our projects accessible. We also like having full control over how those websites look at feel. Static sites rather than web apps are faster, lighter and better with SEO.

Rust pages would mean that we can do these things while never having to write HTML, CSS, Javascript (or one of its ever multiplying frameworks ever again). We have more power and flexibility than any off the shelf static site generator and don’t have to relearn the specifics of how to play with it whenever we update our site. We are also not bound to the specific rules and feature sets of existing static site generators, as we are generating the site programmatically we are free to write whatever code we want, we can design our data model however we want, transform our data however we want and translate them to UI elements however we please. We have the freedom, compile checked guarantees and access to libraries that we are used to with Rust.
