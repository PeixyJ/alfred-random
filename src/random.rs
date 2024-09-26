use std::string::ToString;
use uuid::Uuid;
use chrono::{DateTime, NaiveTime, NaiveDate, Local, Duration};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use rand::{distributions::Alphanumeric};

#[derive(Debug, Clone, Copy)]
pub enum RandomKind {
    Name,
    Email,
    Phone,
    UUID,
    Date,
    Time,
    DateTime,
    Number,
}

impl RandomKind {
    pub fn get_random_by_name(name: String) -> RandomKind {
        let name_lower = name.to_lowercase();
        match name_lower.as_str() {
            n if "name".starts_with(n) => RandomKind::Name,
            e if "email".starts_with(e) => RandomKind::Email,
            p if "phone".starts_with(p) => RandomKind::Phone,
            u if "uuid".starts_with(u) => RandomKind::UUID,
            d if "date".starts_with(d) => RandomKind::Date,
            t if "time".starts_with(t) => RandomKind::Time,
            d if "datetime".starts_with(d) => RandomKind::DateTime,
            n if "number".starts_with(n) => RandomKind::Number,
            _ => RandomKind::Name, // 默认返回 Name
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            RandomKind::Name => "name".to_string(),
            RandomKind::Email => "email".to_string(),
            RandomKind::Phone => "phone".to_string(),
            RandomKind::UUID => "uuid".to_string(),
            RandomKind::Date => "date".to_string(),
            RandomKind::Time => "time".to_string(),
            RandomKind::DateTime => "datetime".to_string(),
            RandomKind::Number => "number".to_string(),
        }
    }
}

pub fn random_value(kind: RandomKind, length: i8) -> String {
    match kind {
        RandomKind::Name => random_name(),
        RandomKind::Email => random_email(),
        RandomKind::Phone => random_phone(),
        RandomKind::UUID => random_uuid(),
        RandomKind::Date => random_date(),
        RandomKind::Time => random_time(),
        RandomKind::DateTime => random_datetime(),
        RandomKind::Number => random_number(length),
    }
}

fn random_name() -> String {
    let name_prefix_list = init_name_prefix();
    let name_suffix_list = init_name_suffix();
    let name_prefix = name_prefix_list[thread_rng().gen_range(0..name_prefix_list.len())].clone();
    let name_suffix = name_suffix_list[thread_rng().gen_range(0..name_suffix_list.len())].clone();
    let name_suffix = name_suffix + &*name_suffix_list[thread_rng().gen_range(0..name_suffix_list.len())].clone();
    format!("{}{}", name_prefix, name_suffix)
}

fn random_email() -> String {
    let mut rng = rand::thread_rng();
    let domain_list = init_email_server();
    let domain = domain_list[rand::thread_rng().gen_range(0..domain_list.len())].clone();
    let email_name: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(|c| c as char)
        .take(10) // 随机电子邮件名称的长度
        .collect();
    format!("{}@{}", email_name, domain)
}

fn random_phone() -> String {
    let mut rng = thread_rng();
    let phone_fix_list = init_phone_prefix();
    let phone_fix = phone_fix_list[rand::thread_rng().gen_range(0..phone_fix_list.len())].clone();
    let suffix: String = (0..8)
        .map(|_| rng.gen_range(0..10))
        .map(|i| i.to_string())
        .collect();
    format!("{}{}", phone_fix, suffix)
}

fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn random_date() -> String {
    let start_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let end_date = Local::now().date_naive();
    let random_date = start_date + Duration::days(rand::thread_rng().gen_range(0..(end_date - start_date).num_days()));
    random_date.format("%Y-%m-%d").to_string()
}

fn random_time() -> String {
    // 生成一个随机的小时数（0-23）
    let hour = rand::thread_rng().gen_range(0..24);
    // 生成一个随机的分钟数（0-59）
    let minute = rand::thread_rng().gen_range(0..60);
    // 生成一个随机的秒数（0-59）
    let second = rand::thread_rng().gen_range(0..60);

    // 创建一个随机的 NaiveTime 对象
    let random_time = NaiveTime::from_hms_opt(hour, minute, second);
    // 将时间格式化为字符串
    random_time.unwrap().format("%H:%M:%S").to_string()
}


fn random_datetime() -> String {
    let start = DateTime::from_timestamp(946656000, 0); // 2000年1月1日
    let end = DateTime::from_timestamp(1670000000, 0); // 2023年1月1日
    let duration = end.unwrap() - start.unwrap();
    let seconds = rand::thread_rng().gen_range(0..i64::from(duration.num_seconds()));
    let random_date = start.unwrap() + chrono::Duration::seconds(seconds);
    random_date.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn random_number(length: i8) -> String {
    let length = if length < 0 { 9 } else { length } as usize;
    let mut rng = rand::thread_rng();
    let digit_dist = Uniform::new(0, 10);
    (0..length).map(|_| rng.sample(digit_dist)).map(|i| i.to_string()).collect()
}

fn init_phone_prefix() -> Vec<String> {
    let mut phone_prefix = Vec::new();
    // 循环生成 130-139 的前缀
    for i in 30..=39 {
        let prefix = format!("13{}", i);
        phone_prefix.push(prefix);
    }

    // 单独添加 144 的前缀
    phone_prefix.push("144".to_string());

    // 循环生成 145-149 的前缀
    for i in 5..=9 {
        phone_prefix.push(format!("14{}", i));
    }

    // 循环生成 150-159 的前缀
    for i in 0..=9 {
        let prefix = format!("15{}", i);
        phone_prefix.push(prefix);
    }

    // 添加 165、166、167 的前缀
    phone_prefix.push("165".to_string());
    phone_prefix.push("166".to_string());
    phone_prefix.push("167".to_string());

    // 循环生成 170-179 的前缀
    for i in 0..=9 {
        let prefix = format!("17{}", i);
        phone_prefix.push(prefix);
    }

    // 循环生成 180-189 的前缀
    for i in 0..=9 {
        let prefix = format!("18{}", i);
        phone_prefix.push(prefix);
    }

    // 添加 190、191、193 的前缀
    phone_prefix.push("190".to_string());
    phone_prefix.push("191".to_string());
    phone_prefix.push("193".to_string());

    // 添加 195、196、197、198、199 的前缀
    phone_prefix.push("195".to_string());
    phone_prefix.push("196".to_string());
    phone_prefix.push("197".to_string());
    phone_prefix.push("198".to_string());
    phone_prefix.push("199".to_string());
    phone_prefix
}

fn init_email_server() -> Vec<String> {
    let mut email_server = Vec::new();
    email_server.push("qq.com".to_string());
    email_server.push("163.com".to_string());
    email_server.push("126.com".to_string());
    email_server.push("gmail.com".to_string());
    email_server.push("outlook.com".to_string());
    email_server.push("yahoo.com".to_string());
    email_server.push("sina.com".to_string());
    email_server.push("sohu.com".to_string());
    email_server.push("aliyun.com".to_string());
    email_server.push("hotmail.com".to_string());
    email_server.push("icloud.com".to_string());
    email_server.push("live.com".to_string());
    email_server
}

fn init_name_prefix() -> Vec<String> {
    let name_prefix_list = vec!["赵".to_string(), "钱".to_string(), "孙".to_string(), "李".to_string(), "周".to_string(), "吴".to_string(), "郑".to_string(), "王".to_string(), "冯".to_string(), "陈".to_string(), "褚".to_string(), "卫".to_string(), "蒋".to_string(), "沈".to_string(), "韩".to_string(), "杨".to_string(), "朱".to_string(), "秦".to_string(), "尤".to_string(), "许".to_string(),
                                "何".to_string(), "吕".to_string(), "施".to_string(), "张".to_string(), "孔".to_string(), "曹".to_string(), "严".to_string(), "华".to_string(), "金".to_string(), "魏".to_string(), "陶".to_string(), "姜".to_string(), "戚".to_string(), "谢".to_string(), "邹".to_string(), "喻".to_string(), "柏".to_string(), "水".to_string(), "窦".to_string(), "章".to_string(),
                                "云".to_string(), "苏".to_string(), "潘".to_string(), "葛".to_string(), "奚".to_string(), "范".to_string(), "彭".to_string(), "郎".to_string(), "鲁".to_string(), "韦".to_string(), "昌".to_string(), "马".to_string(), "苗".to_string(), "凤".to_string(), "花".to_string(), "方".to_string(), "俞".to_string(), "任".to_string(), "袁".to_string(), "柳".to_string(),
                                "酆".to_string(), "鲍".to_string(), "史".to_string(), "唐".to_string(), "费".to_string(), "廉".to_string(), "岑".to_string(), "薛".to_string(), "雷".to_string(), "贺".to_string(), "倪".to_string(), "汤".to_string(), "滕".to_string(), "殷".to_string(), "罗".to_string(), "毕".to_string(), "郝".to_string(), "邬".to_string(), "安".to_string(), "常".to_string(),
                                "乐".to_string(), "于".to_string(), "时".to_string(), "傅".to_string(), "皮".to_string(), "卞".to_string(), "齐".to_string(), "康".to_string(), "伍".to_string(), "余".to_string(), "元".to_string(), "卜".to_string(), "顾".to_string(), "孟".to_string(), "平".to_string(), "黄".to_string(), "和".to_string(), "穆".to_string(), "萧".to_string(), "尹".to_string(),
                                "姚".to_string(), "邵".to_string(), "堪".to_string(), "汪".to_string(), "祁".to_string(), "毛".to_string(), "禹".to_string(), "狄".to_string(), "米".to_string(), "贝".to_string(), "明".to_string(), "臧".to_string(), "计".to_string(), "伏".to_string(), "成".to_string(), "戴".to_string(), "谈".to_string(), "宋".to_string(), "茅".to_string(), "庞".to_string(),
                                "熊".to_string(), "纪".to_string(), "舒".to_string(), "屈".to_string(), "项".to_string(), "祝".to_string(), "董".to_string(), "梁".to_string()];
    name_prefix_list
}

fn init_name_suffix() -> Vec<String> {
    let name_suffix_list = vec!["的".to_string(), "一".to_string(), "是".to_string(), "了".to_string(), "我".to_string(), "不".to_string(), "人".to_string(), "在".to_string(), "他".to_string(), "有".to_string(), "这".to_string(), "个".to_string(), "上".to_string(), "们".to_string(), "来".to_string(), "到".to_string(), "时".to_string(), "大".to_string(), "地".to_string(), "为".to_string(),
                                "子".to_string(), "中".to_string(), "你".to_string(), "说".to_string(), "生".to_string(), "国".to_string(), "年".to_string(), "着".to_string(), "就".to_string(), "那".to_string(), "和".to_string(), "要".to_string(), "她".to_string(), "出".to_string(), "也".to_string(), "得".to_string(), "里".to_string(), "后".to_string(), "自".to_string(), "以".to_string(),
                                "会".to_string(), "家".to_string(), "可".to_string(), "下".to_string(), "而".to_string(), "过".to_string(), "天".to_string(), "去".to_string(), "能".to_string(), "对".to_string(), "小".to_string(), "多".to_string(), "然".to_string(), "于".to_string(), "心".to_string(), "学".to_string(), "么".to_string(), "之".to_string(), "都".to_string(), "好".to_string(),
                                "看".to_string(), "起".to_string(), "发".to_string(), "当".to_string(), "没".to_string(), "成".to_string(), "只".to_string(), "如".to_string(), "事".to_string(), "把".to_string(), "还".to_string(), "用".to_string(), "第".to_string(), "样".to_string(), "道".to_string(), "想".to_string(), "作".to_string(), "种".to_string(), "开".to_string(), "美".to_string(),
                                "总".to_string(), "从".to_string(), "无".to_string(), "情".to_string(), "己".to_string(), "面".to_string(), "最".to_string(), "女".to_string(), "但".to_string(), "现".to_string(), "前".to_string(), "些".to_string(), "所".to_string(), "同".to_string(), "日".to_string(), "手".to_string(), "又".to_string(), "行".to_string(), "意".to_string(), "动".to_string(),
                                "方".to_string(), "期".to_string(), "它".to_string(), "头".to_string(), "经".to_string(), "长".to_string(), "儿".to_string(), "回".to_string(), "位".to_string(), "分".to_string(), "爱".to_string(), "老".to_string(), "因".to_string(), "很".to_string(), "给".to_string(), "名".to_string(), "法".to_string(), "间".to_string(), "斯".to_string(), "知".to_string(),
                                "世".to_string(), "什".to_string(), "两".to_string(), "次".to_string(), "使".to_string(), "身".to_string(), "者".to_string(), "被".to_string(), "高".to_string(), "已".to_string(), "亲".to_string(), "其".to_string(), "进".to_string(), "此".to_string(), "话".to_string(), "常".to_string(), "与".to_string(), "活".to_string(), "正".to_string(), "感".to_string(),
                                "见".to_string(), "明".to_string(), "问".to_string(), "力".to_string(), "理".to_string(), "尔".to_string(), "点".to_string(), "文".to_string(), "几".to_string(), "定".to_string(), "本".to_string(), "公".to_string(), "特".to_string(), "做".to_string(), "外".to_string(), "孩".to_string(), "相".to_string(), "西".to_string(), "果".to_string(), "走".to_string(),
                                "将".to_string(), "月".to_string(), "十".to_string(), "实".to_string(), "向".to_string(), "声".to_string(), "车".to_string(), "全".to_string(), "信".to_string(), "重".to_string(), "三".to_string(), "机".to_string(), "工".to_string(), "物".to_string(), "气".to_string(), "每".to_string(), "并".to_string(), "别".to_string(), "真".to_string(), "打".to_string(),
                                "太".to_string(), "新".to_string(), "比".to_string(), "才".to_string(), "便".to_string(), "夫".to_string(), "再".to_string(), "书".to_string(), "部".to_string(), "水".to_string(), "像".to_string(), "眼".to_string(), "等".to_string(), "体".to_string(), "却".to_string(), "加".to_string(), "电".to_string(), "主".to_string(), "界".to_string(), "门".to_string(),
                                "利".to_string(), "海".to_string(), "受".to_string(), "听".to_string(), "表".to_string(), "德".to_string(), "少".to_string(), "克".to_string(), "代".to_string(), "员".to_string(), "许".to_string(), "稜".to_string(), "先".to_string(), "口".to_string(), "由".to_string(), "死".to_string(), "安".to_string(), "写".to_string(), "性".to_string(), "马".to_string(),
                                "光".to_string(), "白".to_string(), "或".to_string(), "住".to_string(), "难".to_string(), "望".to_string(), "教".to_string(), "命".to_string(), "花".to_string(), "结".to_string(), "乐".to_string(), "色".to_string(), "更".to_string(), "拉".to_string(), "东".to_string(), "神".to_string(), "记".to_string(), "处".to_string(), "让".to_string(), "母".to_string(),
                                "父".to_string(), "应".to_string(), "直".to_string(), "字".to_string(), "场".to_string(), "平".to_string(), "报".to_string(), "友".to_string(), "关".to_string(), "放".to_string(), "至".to_string(), "张".to_string(), "认".to_string(), "接".to_string(), "告".to_string(), "入".to_string(), "笑".to_string(), "内".to_string(), "英".to_string(), "军".to_string(),
                                "候".to_string(), "民".to_string(), "岁".to_string(), "往".to_string(), "何".to_string(), "度".to_string(), "山".to_string(), "觉".to_string(), "路".to_string(), "带".to_string(), "万".to_string(), "男".to_string(), "边".to_string(), "风".to_string(), "解".to_string(), "叫".to_string(), "任".to_string(), "金".to_string(), "快".to_string(), "原".to_string(),
                                "吃".to_string(), "妈".to_string(), "变".to_string(), "通".to_string(), "师".to_string(), "立".to_string(), "象".to_string(), "数".to_string(), "四".to_string(), "失".to_string(), "满".to_string(), "战".to_string(), "远".to_string(), "格".to_string(), "士".to_string(), "音".to_string(), "轻".to_string(), "目".to_string(), "条".to_string(), "呢".to_string(),
                                "病".to_string(), "始".to_string(), "达".to_string(), "深".to_string(), "完".to_string(), "今".to_string(), "提".to_string(), "求".to_string(), "清".to_string(), "王".to_string(), "化".to_string(), "空".to_string(), "业".to_string(), "思".to_string(), "切".to_string(), "怎".to_string(), "非".to_string(), "找".to_string(), "片".to_string(), "罗".to_string(),
                                "钱".to_string(), "紶".to_string(), "吗".to_string(), "语".to_string(), "元".to_string(), "喜".to_string(), "曾".to_string(), "离".to_string(), "飞".to_string(), "科".to_string(), "言".to_string(), "干".to_string(), "流".to_string(), "欢".to_string(), "约".to_string(), "各".to_string(), "即".to_string(), "指".to_string(), "合".to_string(), "反".to_string(),
                                "题".to_string(), "必".to_string(), "该".to_string(), "论".to_string(), "交".to_string(), "终".to_string(), "林".to_string(), "请".to_string(), "医".to_string(), "晚".to_string(), "制".to_string(), "球".to_string(), "决".to_string(), "窢".to_string(), "传".to_string(), "画".to_string(), "保".to_string(), "读".to_string(), "运".to_string(), "及".to_string(),
                                "则".to_string(), "房".to_string(), "早".to_string(), "院".to_string(), "量".to_string(), "苦".to_string(), "火".to_string(), "布".to_string(), "品".to_string(), "近".to_string(), "坐".to_string(), "产".to_string(), "答".to_string(), "星".to_string(), "精".to_string(), "视".to_string(), "五".to_string(), "连".to_string(), "司".to_string(), "巴".to_string(),
                                "奇".to_string(), "管".to_string(), "类".to_string(), "未".to_string(), "朋".to_string(), "且".to_string(), "婚".to_string(), "台".to_string(), "夜".to_string(), "青".to_string(), "北".to_string(), "队".to_string(), "久".to_string(), "乎".to_string(), "越".to_string(), "观".to_string(), "落".to_string(), "尽".to_string(), "形".to_string(), "影".to_string(),
                                "红".to_string(), "爸".to_string(), "百".to_string(), "令".to_string(), "周".to_string(), "吧".to_string(), "识".to_string(), "步".to_string(), "希".to_string(), "亚".to_string(), "术".to_string(), "留".to_string(), "市".to_string(), "半".to_string(), "热".to_string(), "送".to_string(), "兴".to_string(), "造".to_string(), "谈".to_string(), "容".to_string(),
                                "极".to_string(), "随".to_string(), "演".to_string(), "收".to_string(), "首".to_string(), "根".to_string(), "讲".to_string(), "整".to_string(), "式".to_string(), "取".to_string(), "照".to_string(), "办".to_string(), "强".to_string(), "石".to_string(), "古".to_string(), "华".to_string(), "諣".to_string(), "拿".to_string(), "计".to_string(), "您".to_string(),
                                "装".to_string(), "似".to_string(), "足".to_string(), "双".to_string(), "妻".to_string(), "尼".to_string(), "转".to_string(), "诉".to_string(), "米".to_string(), "称".to_string(), "丽".to_string(), "客".to_string(), "南".to_string(), "领".to_string(), "节".to_string(), "衣".to_string(), "站".to_string(), "黑".to_string(), "刻".to_string(), "统".to_string(),
                                "断".to_string(), "福".to_string(), "城".to_string(), "故".to_string(), "历".to_string(), "惊".to_string(), "脸".to_string(), "选".to_string(), "包".to_string(), "紧".to_string(), "争".to_string(), "另".to_string(), "建".to_string(), "维".to_string(), "绝".to_string(), "树".to_string(), "系".to_string(), "伤".to_string(), "示".to_string(), "愿".to_string(),
                                "持".to_string(), "千".to_string(), "史".to_string(), "谁".to_string(), "准".to_string(), "联".to_string(), "妇".to_string(), "纪".to_string(), "基".to_string(), "买".to_string(), "志".to_string(), "静".to_string(), "阿".to_string(), "诗".to_string(), "独".to_string(), "复".to_string(), "痛".to_string(), "消".to_string(), "社".to_string(), "算".to_string(),
                                "义".to_string(), "竟".to_string(), "确".to_string(), "酒".to_string(), "需".to_string(), "单".to_string(), "治".to_string(), "卡".to_string(), "幸".to_string(), "兰".to_string(), "念".to_string(), "举".to_string(), "仅".to_string(), "钟".to_string(), "怕".to_string(), "共".to_string(), "毛".to_string(), "句".to_string(), "息".to_string(), "功".to_string(),
                                "官".to_string(), "待".to_string(), "究".to_string(), "跟".to_string(), "穿".to_string(), "室".to_string(), "易".to_string(), "游".to_string(), "程".to_string(), "号".to_string(), "居".to_string(), "考".to_string(), "突".to_string(), "皮".to_string(), "哪".to_string(), "费".to_string(), "倒".to_string(), "价".to_string(), "图".to_string(), "具".to_string(),
                                "刚".to_string(), "脑".to_string(), "永".to_string(), "歌".to_string(), "响".to_string(), "商".to_string(), "礼".to_string(), "细".to_string(), "专".to_string(), "黄".to_string(), "块".to_string(), "脚".to_string(), "味".to_string(), "灵".to_string(), "改".to_string(), "据".to_string(), "般".to_string(), "破".to_string(), "引".to_string(), "食".to_string(),
                                "仍".to_string(), "存".to_string(), "众".to_string(), "注".to_string(), "笔".to_string(), "甚".to_string(), "某".to_string(), "沉".to_string(), "血".to_string(), "备".to_string(), "习".to_string(), "校".to_string(), "默".to_string(), "务".to_string(), "土".to_string(), "微".to_string(), "娘".to_string(), "须".to_string(), "试".to_string(), "怀".to_string(),
                                "料".to_string(), "调".to_string(), "广".to_string(), "蜖".to_string(), "苏".to_string(), "显".to_string(), "赛".to_string(), "查".to_string(), "密".to_string(), "议".to_string(), "底".to_string(), "列".to_string(), "富".to_string(), "梦".to_string(), "错".to_string(), "座".to_string(), "参".to_string(), "八".to_string(), "除".to_string(), "跑".to_string(),
                                "亮".to_string(), "假".to_string(), "印".to_string(), "设".to_string(), "线".to_string(), "温".to_string(), "虽".to_string(), "掉".to_string(), "京".to_string(), "初".to_string(), "养".to_string(), "香".to_string(), "停".to_string(), "际".to_string(), "致".to_string(), "阳".to_string(), "纸".to_string(), "李".to_string(), "纳".to_string(), "验".to_string(),
                                "助".to_string(), "激".to_string(), "够".to_string(), "严".to_string(), "证".to_string(), "帝".to_string(), "饭".to_string(), "忘".to_string(), "趣".to_string(), "支".to_string(), "春".to_string(), "集".to_string(), "丈".to_string(), "木".to_string(), "研".to_string(), "班".to_string(), "普".to_string(), "导".to_string(), "顿".to_string(), "睡".to_string(),
                                "展".to_string(), "跳".to_string(), "获".to_string(), "艺".to_string(), "六".to_string(), "波".to_string(), "察".to_string(), "群".to_string(), "皇".to_string(), "段".to_string(), "急".to_string(), "庭".to_string(), "创".to_string(), "区".to_string(), "奥".to_string(), "器".to_string(), "谢".to_string(), "弟".to_string(), "店".to_string(), "否".to_string(),
                                "害".to_string(), "草".to_string(), "排".to_string(), "背".to_string(), "止".to_string(), "组".to_string(), "州".to_string(), "朝".to_string(), "封".to_string(), "睛".to_string(), "板".to_string(), "角".to_string(), "况".to_string(), "曲".to_string(), "馆".to_string(), "育".to_string(), "忙".to_string(), "质".to_string(), "河".to_string(), "续".to_string(),
                                "哥".to_string(), "呼".to_string(), "若".to_string(), "推".to_string(), "境".to_string(), "遇".to_string(), "雨".to_string(), "标".to_string(), "姐".to_string(), "充".to_string(), "围".to_string(), "案".to_string(), "伦".to_string(), "护".to_string(), "冷".to_string(), "警".to_string(), "贝".to_string(), "著".to_string(), "雪".to_string(), "索".to_string(),
                                "剧".to_string(), "啊".to_string(), "船".to_string(), "险".to_string(), "烟".to_string(), "依".to_string(), "斗".to_string(), "值".to_string(), "帮".to_string(), "汉".to_string(), "慢".to_string(), "佛".to_string(), "肯".to_string(), "闻".to_string(), "唱".to_string(), "沙".to_string(), "局".to_string(), "伯".to_string(), "族".to_string(), "低".to_string(),
                                "玩".to_string(), "资".to_string(), "屋".to_string(), "击".to_string(), "速".to_string(), "顾".to_string(), "泪".to_string(), "洲".to_string(), "团".to_string(), "圣".to_string(), "旁".to_string(), "堂".to_string(), "兵".to_string(), "七".to_string(), "露".to_string(), "园".to_string(), "牛".to_string(), "哭".to_string(), "旅".to_string(), "街".to_string(),
                                "劳".to_string(), "型".to_string(), "烈".to_string(), "姑".to_string(), "陈".to_string(), "莫".to_string(), "鱼".to_string(), "异".to_string(), "抱".to_string(), "宝".to_string(), "权".to_string(), "鲁".to_string(), "简".to_string(), "态".to_string(), "级".to_string(), "票".to_string(), "怪".to_string(), "寻".to_string(), "杀".to_string(), "律".to_string(),
                                "胜".to_string(), "份".to_string(), "汽".to_string(), "右".to_string(), "洋".to_string(), "范".to_string(), "床".to_string(), "舞".to_string(), "秘".to_string(), "午".to_string(), "登".to_string(), "楼".to_string(), "贵".to_string(), "吸".to_string(), "责".to_string(), "例".to_string(), "追".to_string(), "较".to_string(), "职".to_string(), "属".to_string(),
                                "渐".to_string(), "左".to_string(), "录".to_string(), "丝".to_string(), "牙".to_string(), "党".to_string(), "继".to_string(), "托".to_string(), "赶".to_string(), "章".to_string(), "智".to_string(), "冲".to_string(), "叶".to_string(), "胡".to_string(), "吉".to_string(), "卖".to_string(), "坚".to_string(), "喝".to_string(), "肉".to_string(), "遗".to_string(),
                                "救".to_string(), "修".to_string(), "松".to_string(), "临".to_string(), "藏".to_string(), "担".to_string(), "戏".to_string(), "善".to_string(), "卫".to_string(), "药".to_string(), "悲".to_string(), "敢".to_string(), "靠".to_string(), "伊".to_string(), "村".to_string(), "戴".to_string(), "词".to_string(), "森".to_string(), "耳".to_string(), "差".to_string(),
                                "短".to_string(), "祖".to_string(), "云".to_string(), "规".to_string(), "窗".to_string(), "散".to_string(), "迷".to_string(), "油".to_string(), "旧".to_string(), "适".to_string(), "乡".to_string(), "架".to_string(), "恩".to_string(), "投".to_string(), "弹".to_string(), "铁".to_string(), "博".to_string(), "雷".to_string(), "府".to_string(), "压".to_string(),
                                "超".to_string(), "负".to_string(), "勒".to_string(), "杂".to_string(), "醒".to_string(), "洗".to_string(), "采".to_string(), "毫".to_string(), "嘴".to_string(), "毕".to_string(), "九".to_string(), "冰".to_string(), "既".to_string(), "状".to_string(), "乱".to_string(), "景".to_string(), "席".to_string(), "珍".to_string(), "童".to_string(), "顶".to_string(),
                                "派".to_string(), "素".to_string(), "脱".to_string(), "农".to_string(), "疑".to_string(), "练".to_string(), "野".to_string(), "按".to_string(), "犯".to_string(), "拍".to_string(), "征".to_string(), "坏".to_string(), "骨".to_string(), "余".to_string(), "承".to_string(), "置".to_string(), "臓".to_string(), "彩".to_string(), "灯".to_string(), "巨".to_string(),
                                "琴".to_string(), "免".to_string(), "环".to_string(), "姆".to_string(), "暗".to_string(), "换".to_string(), "技".to_string(), "翻".to_string(), "束".to_string(), "增".to_string(), "忍".to_string(), "餐".to_string(), "洛".to_string(), "塞".to_string(), "缺".to_string(), "忆".to_string(), "判".to_string(), "欧".to_string(), "层".to_string(), "付".to_string(),
                                "阵".to_string(), "玛".to_string(), "批".to_string(), "岛".to_string(), "项".to_string(), "狗".to_string(), "休".to_string(), "懂".to_string(), "武".to_string(), "革".to_string(), "良".to_string(), "恶".to_string(), "恋".to_string(), "委".to_string(), "拥".to_string(), "娜".to_string(), "妙".to_string(), "探".to_string(), "呀".to_string(), "营".to_string(),
                                "退".to_string(), "摇".to_string(), "弄".to_string(), "桌".to_string(), "熟".to_string(), "诺".to_string(), "宣".to_string(), "银".to_string(), "势".to_string(), "奖".to_string(), "宫".to_string(), "忽".to_string(), "套".to_string(), "康".to_string(), "供".to_string(), "优".to_string(), "课".to_string(), "鸟".to_string(), "喊".to_string(), "降".to_string(),
                                "夏".to_string(), "困".to_string(), "刘".to_string(), "罪".to_string(), "亡".to_string(), "鞋".to_string(), "健".to_string(), "模".to_string(), "败".to_string(), "伴".to_string(), "守".to_string(), "挥".to_string(), "鲜".to_string(), "财".to_string(), "孤".to_string(), "枪".to_string(), "禁".to_string(), "恐".to_string(), "伙".to_string(), "杰".to_string(),
                                "迹".to_string(), "妹".to_string(), "藸".to_string(), "遍".to_string(), "盖".to_string(), "副".to_string(), "坦".to_string(), "牌".to_string(), "江".to_string(), "顺".to_string(), "秋".to_string(), "萨".to_string(), "菜".to_string(), "划".to_string(), "授".to_string(), "归".to_string(), "浪".to_string(), "听".to_string(), "凡".to_string(), "预".to_string(),
                                "奶".to_string(), "雄".to_string(), "升".to_string(), "碃".to_string(), "编".to_string(), "典".to_string(), "袋".to_string(), "莱".to_string(), "含".to_string(), "盛".to_string(), "济".to_string(), "蒙".to_string(), "棋".to_string(), "端".to_string(), "腿".to_string(), "招".to_string(), "释".to_string(), "介".to_string(), "烧".to_string(), "误".to_string(),
                                "乾".to_string(), "坤".to_string()];
    name_suffix_list
}