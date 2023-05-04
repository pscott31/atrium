# Atrium
Atrium aims to be a useful tool for managing a village hall. 

The entire frontend is written in Rust and compiled to WebAssembly using the delightful [Leptos](https://github.com/leptos-rs/leptos).

The front end communicates directly with [SurrealDB](https://surrealdb.com) for a backend. 

It's in very early development so don't expect it to do anything useful. The main goal of the project was to give me an excuse to learn Rust and dip my toes into the fantastic ecosystem of projects like Leptos and SurrealDB. If it becomes something that is actually works then that is a bonus.

Features I'd like to implement:
- Managing hall users
- Bookings
- Sync bookings with Google calendar
- Generating invoices from bookings
- Keeping track of issued/paid invoices
- Expenses
- Some simple accounting