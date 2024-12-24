// @generated automatically by Diesel CLI.

diesel::table! {
    sys_menu (id) {
        id -> Bigint,
        #[max_length = 50]
        menu_name -> Varchar,
        menu_type -> Tinyint,
        status -> Tinyint,
        sort -> Integer,
        parent_id -> Bigint,
        #[max_length = 255]
        menu_url -> Varchar,
        #[max_length = 255]
        api_url -> Varchar,
        #[max_length = 255]
        menu_icon -> Varchar,
        #[max_length = 255]
        remark -> Nullable<Varchar>,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    sys_role (id) {
        id -> Bigint,
        #[max_length = 50]
        role_name -> Varchar,
        status_id -> Tinyint,
        sort -> Integer,
        #[max_length = 255]
        remark -> Varchar,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    sys_role_menu (id) {
        id -> Bigint,
        role_id -> Bigint,
        menu_id -> Bigint,
        create_time -> Datetime,
    }
}

diesel::table! {
    sys_user (id) {
        id -> Bigint,
        #[max_length = 11]
        mobile -> Char,
        #[max_length = 50]
        user_name -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        status_id -> Tinyint,
        sort -> Integer,
        #[max_length = 255]
        remark -> Nullable<Varchar>,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    sys_user_role (id) {
        id -> Bigint,
        user_id -> Bigint,
        role_id -> Bigint,
        create_time -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sys_menu,
    sys_role,
    sys_role_menu,
    sys_user,
    sys_user_role,
);
