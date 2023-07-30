use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Hourse {
    id: u64, // 主键id
    name: String, // 马的名称
    images: String, // 马的图片，多张用英文逗号‘,’隔开
    breed: String, // 马的品种
    birth: u64, // 马的生日
    info: String, // 马的说明信息
    status_health: u8, // 马的健康状态:{0:健康, 1:生病, 2: 死亡}
    status_activity: u8, // 马的活动状态:{0:空闲, 1:使用中}
    features: String, // 马的特点
    tags: String, // 马的标签，多个标签用英文逗号‘,’隔开
    special: String, // 马的特别说明
    manager_user_id: u64, // 管理用户id
    look_number: u64, // 浏览人数
    use_number: u64, // 使用人数
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool, // 是否被删除
}

#[derive(Debug, Serialize, Deserialize)]
struct HorseApply {
    id: u64, // 主键id
    hourse_id: u64, // 马的id
    user_id: u64, // 申请的用户id
    approval_user_id: u64, // 审批用户id
    message: String, // 留言
    start_date: u64, // 开始时间
    end_date: u64, // 结束时间
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool, // 是否被删除
}

#[derive(Debug, Serialize, Deserialize)]
struct HorseAppointment {
    id: u64, // 主键id
    hourse_id: u64, // 预约马的id
    user_id: u64, // 预约的用户id
    approval_user_id: u64, // 审批用户id
    start_date: u64, // 开始时间
    end_date: u64, // 结束时间
    message: String, // 留言
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool, // 是否被删除
}

#[derive(Debug, Serialize, Deserialize)]
struct HorseReport {
    id: u64, // 主键id
    hourse_id: u64, // 马的id
    user_id: u64, // 用户id
    status_health: u8, // 马的健康状态:{0:健康, 1:生病, 2: 死亡}
    status_activity: u8, // 马的活动状态:{0:空闲, 1:使用中}
    heat: u8, // 马的体温 (单位摄氏度)
    location: String, // 当前位置 (经度, 纬度)
    address: String, // 当前地址
    kg: u32, // 重量 (单位千克)
    message: String,
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool, // 是否被删除
}

#[derive(Debug, Serialize, Deserialize)]
struct HorseSignIn {
    id: u64, // 主键id
    hourse_id: u64, // 马的id
    user_id: u64, // 用户id
    ip: String, // IP地址
    location: String, // 当前位置 (经度, 纬度)
    address: String, // 当前地址
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool, // 是否被删除
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    id: u64, // 主键id
    user_id: u64, // 用户id
    message: String, // 消息
    log_type: u8, // 日志类型
    loc: String, // 代码位置
    create_at: u64, // 创建时间
    update_at: u64, // 更新时间
    is_del: bool, // 是否被删除
}

