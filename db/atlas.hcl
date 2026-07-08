env "local" {
  src = "file://sqldb/schema/001_schema.sql"
  url = "sqlite://sqldb/db.sqlite3"
  dev = "sqlite://dev?mode=memory"

  migration {
    dir    = "file://sqldb/migrate"
  }
}
