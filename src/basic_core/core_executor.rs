use tokio::sync::mpsc::UnboundedReceiver;

pub enum CoreCommand {
    SaveConfig {
        username: Option<String>,
        ip: Option<String>,
        port: Option<String>
    },
    StartServer { username: String, ip: String, port: String },
    StartClient { username: String, ip: String, port: String },
}

pub async fn execute(mut receiver: UnboundedReceiver<CoreCommand>) {
    {
        while let Some(cmd) = receiver.recv().await {
            match cmd {
                CoreCommand::SaveConfig { username, ip, port } => {save_config(
                    username,
                    ip,
                    port
                )},
                CoreCommand::StartServer { username, ip, port } => {start_server(&username, &ip, &port)},
                CoreCommand::StartClient { username, ip, port } => {start_client(&username, &ip, &port)},
            }
        }
    }
}

fn save_config(username: Option<String>, ip: Option<String>, port: Option<String>) {
    log::info!("Core: Сохраняю конфиг...");
    // config.save(username, ip, port);
}

fn start_server(user: &str, ip: &str, port: &str) {
    log::info!("Core: Запускаю сервер...");
    // network.start_server().await;}
}

fn start_client(user: &str, ip: &str, port: &str) {
    log::info!("Core: Подключаюсь как клиент...");
}