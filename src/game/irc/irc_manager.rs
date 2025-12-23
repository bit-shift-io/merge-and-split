use std::sync::mpsc::{self, Receiver};
use std::thread;
use irc::client::prelude::*;
use tokio::runtime::Runtime;
use futures::prelude::*;
use tokio::sync::mpsc::UnboundedSender;
use std::collections::BTreeSet;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum IrcCommand {
    SendMessage { target: String, message: String },
    JoinChannel(String),
}

#[derive(Debug, Clone)]
pub enum IrcEvent {
    Connected,
    MessageReceived { target: String, sender: String, message: String },
    UserJoined(String),
    UserLeft(String),
    UserList(Vec<String>),
    Disconnected,
}

pub struct IrcManager {
    command_sender: UnboundedSender<IrcCommand>,
    event_receiver: Receiver<IrcEvent>,
    users: Arc<RwLock<BTreeSet<String>>>,
}

impl IrcManager {
    pub fn new(server: String, nickname: String, channels: Vec<String>) -> Self {
        let (command_sender, mut command_receiver) = tokio::sync::mpsc::unbounded_channel::<IrcCommand>();
        let (event_sender, event_receiver) = mpsc::channel();
        let users = Arc::new(RwLock::new(BTreeSet::new()));
        let users_clone = users.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let config = Config {
                    nickname: Some(nickname.clone()),
                    server: Some(server.clone()),
                    channels: channels.clone(),
                    ..Config::default()
                };

                let mut client = match Client::from_config(config).await {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to create IRC client: {}", e);
                        return;
                    }
                };

                if let Err(e) = client.identify() {
                    eprintln!("Failed to identify: {}", e);
                    // continue, maybe it works anyway or we retry? For now, just log.
                }

                let mut stream = match client.stream() {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to get stream: {}", e);
                        return; 
                    }
                };
                
                let sender = client.sender();

                // Announce connection
                let _ = event_sender.send(IrcEvent::Connected);

                loop {
                    tokio::select! {
                        Some(message) = stream.next() => {
                            match message {
                                Ok(msg) => {
                                     if let Command::PRIVMSG(target, content) = msg.command {
                                        if let Some(prefix) = msg.prefix {
                                             let sender = match prefix {
                                                 Prefix::ServerName(s) => s,
                                                 Prefix::Nickname(n, _, _) => n,
                                             };
                                             let _ = event_sender.send(IrcEvent::MessageReceived {
                                                 target: target,
                                                 sender: sender,
                                                 message: content,
                                             });
                                        }
                                    } else if let Command::JOIN(_channel, _, _) = msg.command {
                                        if let Some(Prefix::Nickname(n, _, _)) = msg.prefix {
                                            if let Ok(mut u) = users_clone.write() {
                                                u.insert(n.clone());
                                                let _ = event_sender.send(IrcEvent::UserJoined(n));
                                            }
                                        }
                                    } else if let Command::PART(_channel, _) = msg.command {
                                        if let Some(Prefix::Nickname(n, _, _)) = msg.prefix {
                                            if let Ok(mut u) = users_clone.write() {
                                                u.remove(&n);
                                                let _ = event_sender.send(IrcEvent::UserLeft(n));
                                            }
                                        }
                                    } else if let Command::QUIT(_) = msg.command {
                                        if let Some(Prefix::Nickname(n, _, _)) = msg.prefix {
                                            if let Ok(mut u) = users_clone.write() {
                                                u.remove(&n);
                                                let _ = event_sender.send(IrcEvent::UserLeft(n));
                                            }
                                        }
                                    } else if let Command::Response(Response::RPL_NAMREPLY, params) = msg.command {
                                        // Format: <nick> = <channel> :<nick1> <nick2> ...
                                        if params.len() >= 4 {
                                            let names = &params[3];
                                            let mut user_list = Vec::new();
                                            if let Ok(mut u) = users_clone.write() {
                                                for name in names.split_whitespace() {
                                                    let clean_name = name.trim_start_matches(|c| c == '@' || c == '+' || c == '%' || c == '&' || c == '~');
                                                    u.insert(clean_name.to_lowercase()); // Store lowercase for comparison
                                                    user_list.push(clean_name.to_lowercase());
                                                }
                                                let _ = event_sender.send(IrcEvent::UserList(user_list));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("IRC Stream Error: {}", e);
                                    // Depending on error we might want to break or continue
                                    // For now, if stream errors, we probably lost connection.
                                    break;
                                }
                            }
                        }
                        Some(cmd) = command_receiver.recv() => {
                             match cmd {
                                 IrcCommand::SendMessage { target, message } => {
                                     let _ = sender.send_privmsg(target, message);
                                 }
                                 IrcCommand::JoinChannel(channel) => {
                                     let _ = sender.send_join(channel);
                                 }
                             }
                        }
                        else => {
                             // Both channels closed?
                             break;
                        }
                    }
                }
                 let _ = event_sender.send(IrcEvent::Disconnected);
            });
        });

        Self {
            command_sender, 
            event_receiver,
            users,
        }
    }

    pub fn get_users(&self) -> Vec<String> {
        if let Ok(u) = self.users.read() {
            u.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn send_message(&self, target: String, message: String) {
        let _ = self.command_sender.send(IrcCommand::SendMessage { target, message });
    }
    
    pub fn join_channel(&self, channel: String) {
         let _ = self.command_sender.send(IrcCommand::JoinChannel(channel));
    }
    
    pub fn process_events(&self) -> Vec<IrcEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_receiver.try_recv() {
            events.push(event);
        }
        events
    }
}
