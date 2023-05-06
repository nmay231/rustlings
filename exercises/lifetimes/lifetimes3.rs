// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

struct Book<'a> {
    // In this case, it would probably be best to have the Book struct take ownership of the two String's, right? I guess it depends. It would be annoying to describe the lifetime on every single function that takes a Book as an argument, like returns_a_book() below.
    author: &'a str,
    title: &'a str,
}

struct BookString {
    author: String,
    title: String,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);

    let my_book = returns_a_book();
    borrows_a_book(&my_book);
    println!("{}, {}", my_book.author, my_book.title);
}

// I guess you only need lifetimes if you are returning a book type
fn returns_a_book<'a>() -> Book<'a> {
    Book {
        author: "Me",
        title: "Your face",
    }
}

// And not if you are just borrowing it
fn borrows_a_book(b: &Book) {
    println!("{}", b.author);
}

// Even when you're just implementing a method. Yeah, I think it's much easier to just implement Book using owned String's.

impl<'a> Book<'a> {
    fn new(author: &'a str, title: &'a str) -> Self {
        Book { author, title }
    }
}

// I *was* going to say it is worth using &str over String for performance if you often have the same strings being used in those slots for many objects (you only have one copy of that string instead of a copy for each object), but at that point, either set the lifetime to 'static, or better yet, use an Enum since that's what the finite set of strings were anyways. You can implement Display on the enum if you still need a string value for each one.
// Yeah... I'm starting to appreciate Rust more and more.
