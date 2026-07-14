env "local" {
  src = "file://schema"
  url = "sqlite:///../data/db.sqlite3"
  dev = "sqlite://file?mode=memory"

  migration {
    dir = "file://migrations"
  }
}
