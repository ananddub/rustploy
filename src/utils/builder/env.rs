use auto_di::resolve;
use minijinja::{Environment, Value, context};
use serde::Serialize;
use sqlx::SqlitePool;
use std::collections::{BTreeMap, HashMap};

pub fn generate_env_app(
    db_env: String,
    db_project: String,
    db_app: String,
) -> BTreeMap<String, String> {
    let map: HashMap<String, String> = convert_env_to_map(&db_env);
    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project,
    );
    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);
    let app_env = fix_env(env_project_map(map, pro_map), &db_app);
    convert_env_to_btree_map(&app_env)
}

pub async fn generate_env_app_with_id(
    env_id: i64,
    pr_id: i64,
    app_id: i64,
) -> Option<HashMap<String, String>> {
    let q = resolve::<SqlitePool>().await.unwrap();
    let db_env = sqlx::query!(r#"select * from environments where id =?"#, env_id)
        .fetch_one(q.as_ref())
        .await
        .ok()
        .unwrap();
    let db_project_env = sqlx::query!("select * from projects where id = ?", pr_id)
        .fetch_one(q.as_ref())
        .await
        .ok()
        .unwrap();
    let db_app_env = sqlx::query!("select * from applications where id = ?", app_id)
        .fetch_one(q.as_ref())
        .await
        .ok()
        .unwrap();

    let map: HashMap<String, String> = convert_env_to_map(&db_env.env_var);

    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project_env.env_var,
    );

    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);

    let app_env = fix_env(
        env_project_map(map, pro_map),
        &db_app_env.env_var.or_else(|| Some("".to_string())).unwrap(),
    );

    Some(convert_env_to_map(&app_env))
}

pub async fn generate_env_compose(
    env_id: i64,
    pr_id: i64,
    compose_id: i64,
) -> Option<HashMap<String, String>> {
    let q = resolve::<SqlitePool>().await.unwrap();
    let db_env = sqlx::query!(r#"select * from environments where id =?"#, env_id)
        .fetch_one(q.as_ref())
        .await
        .ok()
        .unwrap();
    let db_project_env = sqlx::query!("select * from projects where id = ?", pr_id)
        .fetch_one(q.as_ref())
        .await
        .ok()
        .unwrap();
    let db_compose_env = sqlx::query!("select * from compose_projects where id = ?", compose_id)
        .fetch_one(q.as_ref())
        .await
        .ok()
        .unwrap();
    let map: HashMap<String, String> = convert_env_to_map(&db_env.env_var);

    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project_env.env_var,
    );

    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);

    let compose_env = fix_env(
        env_project_map(map, pro_map),
        &db_compose_env
            .env_var
            .or_else(|| Some("".to_string()))
            .unwrap(),
    );

    Some(convert_env_to_map(&compose_env))
}

pub async fn generate_env_db(
    env_id: i64,
    db_env_var: &str,
) -> Option<HashMap<String, String>> {
    let q = resolve::<SqlitePool>().await.unwrap();
    let db_env = sqlx::query!(r#"select * from environments where id =?"#, env_id)
        .fetch_one(q.as_ref())
        .await
        .ok()?;
    let db_project_env = sqlx::query!("select * from projects where id = ?", db_env.project_id)
        .fetch_one(q.as_ref())
        .await
        .ok()?;
    let map: HashMap<String, String> = convert_env_to_map(&db_env.env_var);

    let pro_env = fix_env(
        context! {
            enviroment => map,
            ENVIROMENT => map
        },
        &db_project_env.env_var,
    );

    let pro_map: HashMap<String, String> = convert_env_to_map(&pro_env);

    let resolved_db_env = fix_env(
        env_project_map(map, pro_map),
        db_env_var,
    );

    Some(convert_env_to_map(&resolved_db_env))
}

fn convert_env_to_map(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect()
}

fn convert_env_to_btree_map(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect()
}

fn env_project_map(env: HashMap<String, String>, proj: HashMap<String, String>) -> Value {
    context! {
        enviroment => env,
        ENVIROMENT => env,
        PROJECT => proj,
        project => proj
    }
}

fn fix_env<S: Serialize>(map: S, text: &str) -> String {
    let mut env = Environment::new();
    env.add_template("env", &text).unwrap();
    let template = env.get_template("env").unwrap();
    template.render(map).unwrap()
}
