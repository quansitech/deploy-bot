//! Database migrations module
//!
//! This module embeds all SQL migration files using the refinery library.
//! Migration files should be named using the format: V{N}__description.sql

use refinery::embed_migrations;

embed_migrations!("src/database/migrations");
