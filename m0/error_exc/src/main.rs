use std::error::Error;
use std::fmt;

fn main() {
    let cases = [
        ("a@b.com", "123", "321"),
        ("wrong@b.com", "123", "321"),
        ("a@b.com", "123", "000"),
    ];

    for (email, pass, key) in cases {
        match get_data(email, pass, key) {
            Ok(data) => println!("Data: {data}"),
            Err(e) => process_error(e),
        }
    }
}

fn process_error(e: impl Error) {
    println!("Message for user: {}", e);
    eprintln!("Debug: {:?}", e);

    if let Some(source) = e.source() {
        eprintln!("Caused by: {}", source);
    }
    
}

pub fn get_data(email: &str, pass: &str, key: &str) -> Result<String, GetDataError> {
    let user_id = login(email, pass)?;
    let data = download(user_id, key)?;
    Ok(data)
}

pub fn login(email: &str, pass: &str) -> Result<u32, LoginError> {
    if email != "a@b.com" {
        return Err(LoginError::InvalidEmail(email.into()));
    }

    if pass != "123" {
        return Err(LoginError::InvalidPassword);
    }

    Ok(42)
}

pub fn download(user_id: u32, key: &str) -> Result<String, LoadError> {
    if user_id != 42 {
        return Err(LoadError::UserNotFound);
    }

    if key != "321" {
        return Err(LoadError::KeyNotFound);
    }

    Ok(String::from("data"))
}

#[derive(Debug)]
pub enum LoadError {
    UserNotFound,
    KeyNotFound,
}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::UserNotFound => write!(f, "user not found"),
            LoadError::KeyNotFound => write!(f, "key not found"),
        }
    }
}

impl Error for LoadError {}

#[derive(Debug)]
pub enum LoginError {
    InvalidEmail(String),
    InvalidPassword,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginError::InvalidEmail(email) => write!(f, "invalid email: {email}"),
            LoginError::InvalidPassword => write!(f, "invalid password"),
        }
    }
}

impl Error for LoginError {}

#[derive(Debug)]
pub enum GetDataError {
    Login(LoginError),
    Load(LoadError),
}
impl fmt::Display for GetDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetDataError::Login(e) => write!(f, "login failed: {e}"),
            GetDataError::Load(e) => write!(f, "load failed: {e}"),
        }
    }
}

impl Error for GetDataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GetDataError::Login(e) => Some(e),
            GetDataError::Load(e) => Some(e),
        }
    }
}

impl From<LoginError> for GetDataError {
    fn from(error: LoginError) -> Self {
        GetDataError::Login(error)
    }
}

impl From<LoadError> for GetDataError {
    fn from(error: LoadError) -> Self {
        GetDataError::Load(error)
    }
} 