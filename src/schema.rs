table! {
    #[allow(non_snake_case)]
    BattleHistory (battle_id) {
        battle_id -> Unsigned<Bigint>,
        hero_id -> Unsigned<Bigint>,
        monster_id -> Unsigned<Bigint>,
        result -> Bool,
        time -> Timestamp,
    }
}

table! {
    #[allow(non_snake_case)]
    Class (class_id) {
        class_id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    #[allow(non_snake_case)]
    Hero (hero_id) {
        hero_id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        class_id -> Unsigned<Bigint>,
        weapon_id -> Nullable<Unsigned<Bigint>>,
        name -> Varchar,
        level -> Unsigned<Tinyint>,
        stars -> Unsigned<Tinyint>,
        price -> Unsigned<Integer>,
        experience -> Unsigned<Integer>,
        energy -> Unsigned<Integer>,
        last_battle_time -> Nullable<Timestamp>,
    }
}

table! {
    #[allow(non_snake_case)]
    HeroSkill (hero_id, skill_id) {
        hero_id -> Unsigned<Bigint>,
        skill_id -> Unsigned<Bigint>,
    }
}

table! {
    #[allow(non_snake_case)]
    Item (item_id) {
        item_id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Unsigned<Integer>,
    }
}

table! {
    #[allow(non_snake_case)]
    Market (transaction_id) {
        transaction_id -> Unsigned<Bigint>,
        seller_id -> Unsigned<Bigint>,
        buyer_id -> Unsigned<Bigint>,
        hero_id -> Unsigned<Bigint>,
        post_time -> Timestamp,
        end_time -> Timestamp,
        price -> Unsigned<Integer>,
        status -> Varchar,
    }
}

table! {
    #[allow(non_snake_case)]
    Monster (monster_id) {
        monster_id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Nullable<Text>,
        level -> Tinyint,
        win_rate -> Unsigned<Integer>,
    }
}

table! {
    #[allow(non_snake_case)]
    Skill (skill_id) {
        skill_id -> Unsigned<Bigint>,
        class_id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Nullable<Text>,
        level_min -> Unsigned<Tinyint>,
        star_min -> Unsigned<Tinyint>,
        win_rate_increase -> Unsigned<Integer>,
        price -> Unsigned<Integer>,
    }
}

table! {
    #[allow(non_snake_case)]
    User (user_id) {
        user_id -> Unsigned<Bigint>,
        first_name -> Varchar,
        surname -> Varchar,
        email -> Varchar,
        date_of_birth -> Date,
        gender -> Char,
        password -> Varchar,
        gold -> Unsigned<Integer>,
    }
}

table! {
    #[allow(non_snake_case)]
    UserItem (user_id, item_id) {
        user_id -> Unsigned<Bigint>,
        item_id -> Unsigned<Bigint>,
        quantity -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    #[allow(non_snake_case)]
    Weapon (weapon_id) {
        weapon_id -> Unsigned<Bigint>,
        class_id -> Unsigned<Bigint>,
        name -> Varchar,
        description -> Nullable<Text>,
        rare -> Unsigned<Tinyint>,
        level_min -> Unsigned<Tinyint>,
        level_max -> Unsigned<Tinyint>,
        win_rate_increase -> Unsigned<Integer>,
        price -> Unsigned<Integer>,
    }
}

joinable!(BattleHistory -> Hero (hero_id));
joinable!(BattleHistory -> Monster (monster_id));
joinable!(Hero -> Class (class_id));
joinable!(Hero -> User (user_id));
joinable!(Hero -> Weapon (weapon_id));
joinable!(HeroSkill -> Hero (hero_id));
joinable!(HeroSkill -> Skill (skill_id));
joinable!(Market -> Hero (hero_id));
joinable!(Skill -> Class (class_id));
joinable!(UserItem -> Item (item_id));
joinable!(UserItem -> User (user_id));
joinable!(Weapon -> Class (class_id));

allow_tables_to_appear_in_same_query!(
    BattleHistory,
    Class,
    Hero,
    HeroSkill,
    Item,
    Market,
    Monster,
    Skill,
    User,
    UserItem,
    Weapon,
);
