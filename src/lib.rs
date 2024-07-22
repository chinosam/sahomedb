//! ![Sahome](https://i.postimg.cc/X7rGVsBb/banner.png)

mod db;
mod func;

pub use db::database;
pub use func::collection;
pub use func::vector;

#[cfg(test)]
mod tests;
