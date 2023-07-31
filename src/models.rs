use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Horse {
    pub id: i64,              // 主键ID
    pub name: String,         // 马的名称
    pub images: String,       // 马的图片，多张用英文逗号‘,’隔开
    pub breed: String,        // 马的品种
    pub birth: u64,           // 马的生日
    pub info: String,         // 马的说明信息
    pub status_health: u8,    // 马的健康状态:{0:健康, 1:生病, 2: 死亡}
    pub status_activity: u8,  // 马的活动状态:{0:空闲, 1:使用中}
    pub features: String,     // 马的特点
    pub tags: String,         // 马的标签，多个标签用英文逗号‘,’隔开
    pub special: String,      // 马的特别说明
    pub manager_user_id: i64, // 管理用户ID
    pub look_number: u64,     // 浏览人数
    pub use_number: u64,      // 使用人数
    pub create_at: u64,       // 创建时间
    pub update_at: u64,       // 更新时间
    pub is_del: bool,         // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct HorseApply {
    id: i64,               // 主键ID
    horse_id: i64,         // 马的ID
    user_id: i64,          // 申请的用户ID
    approval_user_id: i64, // 审批用户id
    message: String,       // 留言
    start_date: u64,       // 开始时间
    end_date: u64,         // 结束时间
    create_at: u64,        // 创建时间
    update_at: u64,        // 更新时间
    is_del: bool,          // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct HorseAppointment {
    id: i64,               // 主键ID
    horse_id: i64,         // 预约马的ID
    user_id: i64,          // 预约的用户ID
    approval_user_id: i64, // 审批用户ID
    start_date: u64,       // 开始时间
    end_date: u64,         // 结束时间
    message: String,       // 留言
    create_at: u64,        // 创建时间
    update_at: u64,        // 更新时间
    is_del: bool,          // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct HorseReport {
    id: i64,             // 主键ID
    horse_id: i64,       // 马的ID
    user_id: i64,        // 用户ID
    status_health: u8,   // 马的健康状态:{0:健康, 1:生病, 2: 死亡}
    status_activity: u8, // 马的活动状态:{0:空闲, 1:使用中}
    heat: u8,            // 马的体温 (单位摄氏度)
    location: String,    // 当前位置 (经度, 纬度)
    address: String,     // 当前地址
    kg: u32,             // 重量 (单位千克)
    message: String,
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool,   // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct HorseSignIn {
    id: i64,          // 主键ID
    horse_id: i64,    // 马的ID
    user_id: i64,     // 用户ID
    ip: String,       // IP地址
    location: String, // 当前位置 (经度, 纬度)
    address: String,  // 当前地址
    create_at: u64,   // 创建时间
    update_at: u64,   // 更新时间
    is_del: bool,     // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Log {
    id: i64,         // 主键ID
    user_id: i64,    // 用户ID
    message: String, // 消息
    log_type: u8,    // 日志类型
    loc: String,     // 代码位置
    create_at: u64,  // 创建时间
    update_at: u64,  // 更新时间
    is_del: bool,    // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    id: i64,             // 主键ID
    message_type: u8,    // 0:普通消息, 1:用户反馈
    send_user_id: i64,   // 发送用户ID
    accept_user_id: i64, // 接收用户ID
    title: String,       // 消息标题
    content: String,     // 消息内容
    create_at: u64,      // 创建时间
    update_at: u64,      // 更新时间
    is_del: bool,        // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    id: i64,        // 主键ID
    parent_id: i64, // 父ID
    name: String,   // 角色名称
    info: String,   // 角色介绍
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool,   // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RoleApply {
    id: i64,               // 主键id
    user_id: i64,          // 申请人的ID
    role_id: i64,          // 申请角色的ID
    approval_user_id: i64, // 审批的用户ID，如果为-1 表示还没有审批 如果为0表示拒绝
    create_at: u64,        // 创建时间
    update_at: u64,        // 更新时间
    is_del: bool,          // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RolePowerRouter {
    id: i64,          // 主键id
    router: String,   // 路由
    is_allow: bool,   // 是否允许访问
    is_inherit: bool, // 是否是继承
    create_at: u64,   // 创建时间
    update_at: u64,   // 更新时间
    is_del: bool,     // 是否被删除
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: i64,              // 主键ID
    role_id: i64,         //权限ID
    username: String,     //用户名
    password: String,     //密码
    nick: String,         //昵称
    email: String,        //邮箱
    avatar: String,       //头像
    name: String,         //姓名
    brith: String,        //出生年月日
    gender: u8,           //性别{0:女, 1:男, 2:中性}
    phone_number: String, //手机号
    affiliation: String,  //所属集体
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    errcode: u8,
    message: String,
    data: T,
}

#[allow(unused)]
impl<T> APIResponse<T> {
    fn new(errcode: u8, message: &str, data: T) -> Self {
        APIResponse {
            errcode,
            message: message.to_string(),
            data,
        }
    }
}
