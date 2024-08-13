# Plectrum

`plectrum`[^1] is a rust crate that provides an easy way to represent
lookup tables in a database (or any data source) as enums in rust.

## Quick example

Suppose we have the following lookup table in an sqlite db:

``` sql
CREATE TABLE item_states (
    id integer PRIMARY KEY,
    label text NOT NULL UNIQUE
);
```

And it contains the following data:

``` sql
sqlite> SELECT * from item_states;
1|todo
2|in_progress
3|completed
4|parked
5|archived
```

Using the `Plectrum` procedural macro provided by the crate, we can
define an enum to represent it as follows:

``` rust
#[derive(Debug, Plectrum)]
#[plectrum(rename_all = "snake_case")]
enum ItemState {
    Todo,
    InProgress,
    Completed,
    Parked,
    Archived,
}
```

This will allow us to use the `plectrum::Mapping` abstraction to
easily convert an instance of the enum to the corresponding `id` or
`value` (label in this case) and vice versa:

``` rust
mapping.by_id(1)                     // Some(ItemState::Todo)
mapping.by_value("in_progress")      // Some(ItemState::InProgress)
mapping.get_id(&ItemState::Parked)   // Some(4)
```

To keep this example concise, I've glossed over one part here, which
is to tell `plectrum` how to fetch the lookup table entries from the
db[^2]. This is done by implementing the `plectrum::DataSource` trait
for a struct and then use that struct instance to initialize
`plectrum::Mapping`. Refer to the complete implementation of this
example [here](examples/sqlite).

## Motivation

While database drivers usually provide ways to map db types to rust
types, e.g. the
[sqlx::FromRow](https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html)
trait in [sqlx](https://github.com/launchbadge/sqlx), this library
caters to a different use case. Entries in lookup tables are typically
_static_. And any insertions/deletions to the tables are performed by
developers and usually accompanied by significant changes in code.

So it makes sense to cache the lookup table entries in memory at the
time of process initialization and map them to an enum type. At this
time, `plectrum` also performs some checks to ensure that there are no
inconsistencies between the entries in the db and the enum
variants. Refer to the [Inconsistencies between enum and lookup
table](#inconsistencies-between-enum-and-lookup-table) section below.

In rest of the code, you can simply use the enum everywhere and
convert between enum <-> id/value fields of the lookup table when
required.

## Installation

``` bash
cargo add plectrum --features derive
```

The `derive` feature provides a proc macro that takes care of
implementing the `plectrum::Enum` trait for the enum.

If you are using the [sqlx]() crate for interacting with your data
source, then it's recommended to specify the `sqlx` feature.

``` bash
cargo add plectrum --features derive,sqlx
```

This feature provides the `plectrum::Error::Sqlx` error variant that
wraps over an `sqlx::Error`. Refer to the [sqlite
example](examples/sqlite) to see the usage.

## Case conversions

The `Plectrum` proc macro supports a `rename_all` attribute for
specifying any case conversions while mapping the enum variant names
in `UpperCamel` case (as per rust conventions) with the values in the
lookup table.

Following case conversions are supported:

- `UPPER CASE`
- `lower case`
- `Title Case`
- `camelCase`
- `UpperCamelCase`
- `snake_case`
- `UPPER_SNAKE_CASE`
- `kebab-case`
- `UPPER-KEBAB-CASE`
- `Train-Case`
- `flatcase`
- `UPPERFLATCASE`

Note that the option names are autological.

If the `rename_all` attribute is not specified, the enum variant names
themselves will be considered as values of the lookup table. In other
words, the default case for values is `UpperCamelCase`, but only if
the rust lang convention for enum variant names is followed.

## Inconsistencies between enum and lookup table

Typically, an instance of the `plectrum::Mapping` struct would be
created at the time of process initialization, which is when all the
entries from the lookup table will be loaded in memory and mapped with
the enum variants. At this time, it's possible to have inconsistencies
between lookup table entries and enum variants. For example:

1. There could be entries in the table for which an enum variants are
   not defined.

2. There could enum variants for which entries don't exist in the
   lookup table

In such cases, the `Mapping::load` function would return
`plectrum::Error::NotDefinedInCode` and
`plectrum::Error::NotFoundInDb` errors respectively.

Because the mapping happens only once during process initialization,
it implies that if an entry is added to or removed from the lookup
table, the enum definition would also have to be modified accordingly
(followed by restarting the process).

The above errors act as a safety net to catch any inconsistencies
eagerly i.e. the process will fail to start, which in most cases is
much better than errors at run time.

# Examples

1. [simple](examples/simple): A simple example to understand the
   motivation behind this crate. It uses hard coded data instead of db
   based lookup table in order to focus on library usage

2. [sqlite](examples/sqlite): This example shows how `plectrum` can be
   used `sqlx`

# License

MIT (See [LICENSE](LICENSE)).

[^1]: Why the name `plectrum`? Earlier this lib was named `lute`,
    short for _LookUp Tables mapped to Enums_. But that name was
    already taken on crates.io. `plectrum` is a closely related and
    available name that came to my mind!

[^2]: Note that this crate *DOES NOT* provide an ORM-like
    functionality, so it's upto the user to write a query to fetch
    lookup table entries from the db.


