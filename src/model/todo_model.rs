use actix_web::web::Data;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value, thing, Array};
use crate::prelude::*;
use crate::utils::{macros::map};
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};


#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: Option<String>,
    pub name: String,
    pub author: String,
    pub created: String,
    pub size: i64,
    pub downloads: i32,
    pub rating: f32
}

impl From<File> for Value {
    fn from(val: File) -> Self {
        match val.id {
            Some(v) => {
                map![
                    "id".into() => v.into(),
                    "name".into() => val.name.into(),
                    "author".into() => val.author.into(),
                    "created".into() => val.created.into(),
                    "size".into() => val.size.into(),
                    "downloads".into() => val.downloads.into(),
                    "rating".into() => val.rating.into()
            ].into()
            },
            None => {
                map![
                    "name".into() => val.name.into(),
                    "author".into() => val.author.into(),
                    "created".into() => val.created.into(),
                    "size".into() => val.size.into(),
                    "downloads".into() => val.downloads.into(),
                    "rating".into() => val.rating.into()
                ].into()
            }
        }
    }
}

impl Creatable for File{}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePatch {
    pub name: Option<String>,
    pub author: Option<String>,
    pub created: Option<String>
}

impl From<FilePatch> for Value {
    fn from(val: FilePatch) -> Self {
        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(n) = val.name { value.insert("name".into(), n.into()); }
        if let Some(a) = val.author { value.insert("author".into(), a.into()); }
        if let Some(c) = val.created { value.insert("created".into(), c.into()); }
        Value::from(value)
    }
}

impl Patchable for FilePatch {}
pub struct FileBMC;

impl FileBMC {

    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM file;";
        let res = db.ds.execute(ast, &db.ses, None, true).await?;
        let first_res = res.into_iter().next().expect("Did not get a response");
        let array: Array = W(first_res.result?).try_into()?;
        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn create<T: Creatable>(db: Data<SurrealDBRepo>, tb: &str, data: T) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";
        let data: Object = W(data.into()).try_into()?;
		let vars: BTreeMap<String, Value> = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];
		let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
        let first_val = ress.into_iter().next().map(|r| r.result).expect("id not returned")?;
        W(first_val.first()).try_into()
    }

    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("file:{}", tid);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
        let first_res = ress.into_iter().next().expect("Did not get a response");
        W(first_res.result?.first()).try_into()
    }
    
    pub async fn update<T: Patchable>(db: Data<SurrealDBRepo>, tid: &str, data: T) -> Result<Object, Error> {
		let sql = "UPDATE $th MERGE $data RETURN *";
        let tid = format!("file:{}", tid);
		let vars = map![
			"th".into() => thing(&tid)?.into(),
			"data".into() => data.into()];
		let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
		let first_res = ress.into_iter().next().expect("id not returned");
		let result = first_res.result?;
        W(result.first()).try_into()
	}

    pub async fn delete(db: Data<SurrealDBRepo>, tid: &str) -> Result<String, Error> {
		let sql = "DELETE $th RETURN *";
        let tid = format!("file:{}", tid);
		let vars = map!["th".into() => thing(&tid)?.into()];
		let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
		let first_res = ress.into_iter().next().expect("id not returned");
		first_res.result?;
        Ok(tid)
	}
}