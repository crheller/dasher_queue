use std::future::Future;
use mysql::*;
use mysql::prelude::*;
use crate::dbsettings::*;

// threading tools
pub fn thread_context() -> glib::MainContext {
    glib::MainContext::thread_default()
        .unwrap_or_else(|| {
            let ctx = glib::MainContext::new();
            ctx.push_thread_default();
            ctx
        })
}

pub fn spawn<F>(future: F) where F: Future<Output = ()> + 'static {
    glib::MainContext::default().spawn_local(future);
}


// db query tools
pub fn get_conn() -> PooledConn {
    // open db connection
    let mut builder = OptsBuilder::new();
    let dboptions = get_db_settings();
    let five_seconds = std::time::Duration::new(5, 0);
    builder = builder.ip_or_hostname(Some(dboptions.host))
            .db_name(Some(dboptions.dbname))
            .user(Some(dboptions.user))
            .pass(Some(dboptions.pass))
            .read_timeout(Some(five_seconds))
            .write_timeout(Some(five_seconds));
    let pool = Pool::new(builder).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}

#[derive(Clone)]
pub struct Data {
    pub experiment_class: String,
    pub experiment_rig: String,
    pub fish_id: String,
    pub chamber_id: String,
    pub imaging: bool,
    pub hardware_test: bool,
    pub addedby: String
}

pub fn get_last_data_entry(userid: std::ffi::OsString, rig: String) -> Option<Data> {

    let mut conn = get_conn();
    // use the connection to query the last entry in the db for this
    // map the results into struct to make things more readable
    let sql_query = format!("SELECT experiment_class, experiment_rig, fish_id, chamber_id, imaging, hardware_test, addedby
                                    FROM data 
                                    WHERE addedby={:?} and experiment_rig='{}'",
                                    userid, rig);
    println!("{:?}", sql_query);
    let res = conn.query_map(
        sql_query,
        |(experiment_class, experiment_rig, fish_id, chamber_id, imaging, hardware_test, addedby)|
        Data {
            experiment_class,
            experiment_rig,
            fish_id,
            chamber_id,
            imaging,
            hardware_test,
            addedby                
        }
    ).expect("Query failed.");    

    if res.len() > 0 {
        let r = Some(res[res.len()-1].clone());
        return r;
    } else {
        let r = None;
        return r;
    }
}

pub fn get_field_options(field: String, db: String) -> Vec<String> {
    let mut conn = get_conn();
    let sql_query = format!("SELECT DISTINCT {} FROM {}", field, db);
    let res:Vec<String> = conn.query(sql_query).unwrap();
    return res
}