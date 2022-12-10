use options::CookieOptions;

pub mod options;

#[derive(Clone, Debug, Default)]
pub struct Cookies(Vec<Cookie>);

impl Cookies {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, cookie: Cookie) {
        self.0.push(cookie);
    }

    pub fn get_all_mut(&mut self) -> &mut Vec<Cookie> {
        &mut self.0
    }

    pub fn get_all(&self) -> &Vec<Cookie> {
        &self.0
    }

    pub fn get_by_value(&self, value: impl Into<String>) -> Vec<&Cookie> {
        let value = value.into();
        let mut cookies = vec![];

        for cookie in self.get_all() {
            if cookie.value == value {
                cookies.push(cookie);
            }
        }

        cookies
    }

    pub fn get_value_by_key(&self, name: impl Into<String>) -> Option<&String> {
        match self.get_by_key(name) {
            Some(r) => Some(&r.name),
            None => None,
        }
    }

    pub fn get_by_key(&self, name: impl Into<String>) -> Option<&Cookie> {
        let name = name.into();

        for cookie in self.get_all() {
            if cookie.name == name {
                return Some(cookie);
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub options: Option<CookieOptions>,
}

impl ToString for Cookie {
    fn to_string(&self) -> String {
        format!(
            "Set-Cookie: {}={};{}",
            self.name,
            self.value,
            match &self.options {
                Some(options) => options.to_string(),
                None => "".to_string(),
            }
        )
    }
}

#[test]
fn cookies_work() {
    let cookie = Cookie {
        name: "user_id".to_string(),
        value: "1".to_string(),
        options: Some(
            CookieOptions::builder()
                .http_only(true)
                .secure(true)
                .build(),
        ),
    };

    assert_eq!(
        cookie.to_string(),
        "Set-Cookie: user_id=1;Secure;HttpOnly".to_string()
    );
}
