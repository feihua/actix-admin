use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::RB;
use crate::schema::sys_user::dsl::sys_user;
use crate::vo::{BaseResponse, err_result_msg, handle_result};

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserAdd {
    pub mobile: String,
    pub user_name: String,
    pub password: String,
    pub status_id: i32,
    pub sort: i32,
    pub remark: Option<String>,

}


#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserUpdate {
    pub id: i64,
    pub mobile: String,
    pub user_name: String,
    pub password: String,
    pub status_id: i32,
    pub sort: i32,
    pub remark: Option<String>,

}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Serialize, Deserialize, QueryableByName, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUser {
    pub id: i64,
    pub mobile: String,
    pub user_name: String,
    pub password: String,
    pub status_id: i32,
    pub sort: i32,
    pub remark: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,

}

impl SysUser {
    // 添加用户信息
    pub fn add_user(s_user: SysUserAdd) -> BaseResponse<String> {
        match &mut RB.clone().get() {
            Ok(conn) => {
                let query = diesel::insert_into(sys_user::table()).values(s_user);
                debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());
                handle_result(query.execute(conn))
            }
            Err(err) => {
                error!("err:{}", err.to_string());
                err_result_msg(err.to_string())
            }
        }
    }
}
