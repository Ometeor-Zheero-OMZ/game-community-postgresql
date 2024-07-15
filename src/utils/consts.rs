use lazy_static::lazy_static;
use std::collections::HashMap;

// サーバーメッセージ
lazy_static! {
    pub static ref SVR_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("SVR_BUILD_SUCCESS_MSG", "サーバー構築に成功しました 🚀");
        map.insert("SVR_BUILD_FAILURE_ERROR_MSG", "🔥 サーバー構築に失敗しました。");

        map
    };
}

// DBメッセージ
lazy_static! {
    pub static ref DB_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("DB_CONNECTION_SUCCESS_MSG", "✅ データベース接続に成功しました。");
        map.insert("DB_CONNECTION_FAILURE_ERROR_MSG", "🔥 データベース接続に失敗しました。");

        map
    };
}

// 環境変数の未設定メッセージ
lazy_static! {
    pub static ref SET_ENV_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "⚠️ 環境変数が設定されていません: FRONTEND_PORT");
        map.insert("DATABASE_URL", "⚠️ 環境変数が設定されていません: DATABASE_URL");
        map
    };
}

// SQLメッセージ
lazy_static! {
    pub static ref SQL_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "⚠️ 環境変数が設定されていません: FRONTEND_PORT");
        map.insert("DATABASE_URL", "⚠️ 環境変数が設定されていません: DATABASE_URL");
        map
    };
}