// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// createTime：2024/12/25 10:01:11

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/*
 *字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictType {
    pub dict_id: Option<i64>,          //字典主键
    pub dict_name: String,             //字典名称
    pub dict_type: String,             //字典类型
    pub status: i8,                    //状态（0：停用，1:正常）
    pub remark: String,                //备注
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *字典类型基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(DictType {}, "sys_dict_type");

/*
 *根据id查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictType{select_by_id(id:&i64) -> Option => "`where dict_id = #{id} limit 1`"}, "sys_dict_type");

/*
 *根据dict_type查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictType{select_by_dict_type(dict_type:&str) -> Option => "`where dict_type = #{dict_type} limit 1`"}, "sys_dict_type");

/*
 *分页查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictType{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_dict_type");

/*
 *根据条件分页查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictType{select_dict_type_list(dict_name:&str, dict_type:&str, status:i8) =>"
    where 1=1
     if dict_name != null && dict_name != '':
      ` and dict_name = #{dict_name} `
     if dict_type != null && dict_type != '':
      ` and dict_type = #{dict_type} `
     if status != 2:
      ` and status = #{status} `
     if !sql.contains('count'):
      ` order by create_time desc"
},"sys_dict_type");
