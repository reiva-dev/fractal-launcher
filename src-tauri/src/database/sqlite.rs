use sqlx::{Sqlite, Connection, SqliteConnection, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::migrate::{Migrate, MigrateDatabase, Migrator};

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    tracing::info!("Init database connection.");
    let uri = "sqlite:../config.db";
    let connection_pool = SqlitePoolOptions::new()
        .min_connections(
            dotenvy::var("DATABASE_MIN_CONNECTIONS")
            .ok()
            .and_then(|f| f.parse().ok())
            .unwrap_or(0)
        )
        .max_connections(
            dotenvy::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|f| f.parse().ok())
            .unwrap_or(4)
        )
        .connect(uri)
        .await?;
    tracing::info!("Build successful database connection.");
    Ok(connection_pool)
}

pub async fn migration() -> Result<(), sqlx::Error> {
    tracing::info!("Check for resource database migration.");
    let uri = "sqlite:../config.db";
    if !Sqlite::database_exists(uri).await? {
        tracing::info!("Create database because not found target.");
        Sqlite::create_database(uri).await?;
    }
    tracing::info!("Start apply migrations.");
    let migrator = Migrator::new(std::path::Path::new("../migration")).await?;
    let mut migrate_connection = SqliteConnection::connect(uri).await?;
    migrate_connection.ensure_migrations_table().await?;
    let ver = migrate_connection.dirty_version().await?;

    match ver {
        None => (),
        _ => panic!("The database is dirty...XP Please check your database status.")
    }

    // There is a way to get the dirty version in sqlx,
    // but no way to get the current version, so I decided to use the deprecated method.
    #[allow(deprecated)]
    let (current_ver, _) = migrate_connection.version().await?.unwrap_or((1, false));
    let _ver = ver.unwrap_or(1);

    for migration in migrator.iter() {
        if migration.version > current_ver {
            //println!("migrate {} to {}", ver, migration.version);
            let applied = migrate_connection.apply(migration).await?;
            tracing::debug!("Applied migration {} to {}: {}ms", current_ver, migration.version, applied.as_millis());
        } else {
            tracing::debug!("Skipped migration {}", current_ver);
        }
    }
    tracing::info!("Migration successful.");
    Ok(())
}