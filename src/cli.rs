use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct MainCliArgs {
    /// Command to execute
    #[clap(subcommand)]
    pub command: Command,

    /// Api key to access server
    #[clap(long, global = true)]
    pub api: Option<String>,

    /// Api server IP address
    #[clap(long, global = true, default_value = "127.0.0.1")]
    pub ip: String,

    /// Api server's port
    #[clap(long, global = true, default_value = "8000")]
    pub port: u16,

    /// Verbose mode. Print raw server responses
    #[clap(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manipulate users
    User(UserArgs),

    /// Manipulate caches
    Cache(CacheArgs),
}

#[derive(Args, Debug)]
pub struct UserArgs {
    /// Command on users to execute
    #[clap(subcommand)]
    pub command: UserCommand,
}

#[derive(Subcommand, Debug)]
pub enum UserCommand {
    /// Create a new user
    Create(UserCreateArgs),
    /// View a user
    View(UserViewArgs),
    /// Change user
    Change(UserChangeArgs),
    /// User's keys managment
    Keys(UserKeysArgs),
}

#[derive(Args, Debug)]
pub struct UserCreateArgs {
    /// Displayed name of account
    #[clap(short, long)]
    pub name: String,

    /// email of account, must be unique
    #[clap(short, long)]
    pub email: String,

    /// Password
    #[clap(short, long)]
    pub pwd: String,
}

#[derive(Args, Debug)]
pub struct UserViewArgs {
    /// ID of requested user to view
    #[clap(short, long)]
    pub id: i32,
}

#[derive(Args, Debug)]
pub struct UserChangeArgs {
    /// ID of requested user to change
    #[clap(short, long)]
    pub id: i32,

    /// New email for user. Can be skipped
    #[clap(short, long)]
    pub email: Option<String>,

    /// New password for user
    #[clap(short, long)]
    pub pwd: Option<String>,
}

#[derive(Args, Debug)]
pub struct UserKeysArgs {
    /// Operation on keys
    #[clap(subcommand)]
    pub command: UserKeysCommand,
}

#[derive(Subcommand, Debug)]
pub enum UserKeysCommand {
    /// Generates a new key for user
    Generate(UserKeysGenerateArgs),
    /// View key(s)
    View(UserKeysViewArgs),
    /// Revoke and delete key
    Revoke(UserKeysDeleteArgs),
}

#[derive(Args, Debug)]
pub struct UserKeysGenerateArgs {
    /// ID of requested user to add key
    #[clap(short, long)]
    pub id: i32,
}

#[derive(Args, Debug)]
pub struct UserKeysViewArgs {
    /// ID of requested user to view key
    #[clap(short, long)]
    pub id: i32,

    /// Number of key to view. If not present, a program displays all.
    #[clap(short, long)]
    pub nmb: Option<usize>,
}

#[derive(Args, Debug)]
pub struct UserKeysDeleteArgs {
    /// ID of requested user to delete key
    #[clap(short, long)]
    pub id: i32,

    /// Number of key to delete
    #[clap(short, long)]
    pub nmb: usize,
}

#[derive(Args, Debug)]
pub struct CacheArgs {
    /// Command on users to execute
    #[clap(subcommand)]
    pub command: CacheCommand,
}

#[derive(Subcommand, Debug)]
pub enum CacheCommand {
    /// Creates cache
    Create(CacheCreateArgs),

    /// Find caches by owner or/and bounds
    Find(CacheFindArgs),

    /// View specified cache
    View(CacheViewArgs),

    /// Change specified cache values
    Change(CacheChangeArgs),

    /// Delete specified cache
    Delete(CacheDeleteArgs),
}

#[derive(Args, Debug)]
pub struct CacheCreateArgs {
    /// Latitude
    #[clap(long)]
    /// Longitude
    pub lat: f64,
    #[clap(long)]
    pub long: f64,

    #[clap(long)]
    pub descrip: String,
    #[clap(long)]
    pub hint: String,
}

#[derive(Args, Debug)]
pub struct CacheFindArgs {
    /// Filter user id
    #[clap(long)]
    pub user: Option<i32>,

    /// Part of bound condition
    #[clap(long)]
    pub min_lat: Option<f64>,
    /// Part of bound condition
    #[clap(long)]
    pub max_lat: Option<f64>,

    /// Part of bound condition
    #[clap(long)]
    pub min_long: Option<f64>,
    /// Part of bound condition
    #[clap(long)]
    pub max_long: Option<f64>,
}

#[derive(Args, Debug)]
pub struct CacheViewArgs {
    /// ID of cache
    #[clap(short, long)]
    pub id: i32,
}

#[derive(Args, Debug)]
pub struct CacheChangeArgs {
    /// ID of cache
    #[clap(short, long)]
    pub id: i32,

    /// new latitide
    #[clap(long)]
    pub lat: Option<f64>,
    /// new longitude
    #[clap(long)]
    pub long: Option<f64>,

    /// new description
    #[clap(long)]
    pub descrip: Option<String>,

    /// new hint
    #[clap(long)]
    pub hint: Option<String>,
}

#[derive(Args, Debug)]
pub struct CacheDeleteArgs {
    /// ID of cache
    #[clap(short, long)]
    pub id: i32,
}
