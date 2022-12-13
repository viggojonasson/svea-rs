#[derive(Debug, Clone)]
pub enum SameSiteAttribute {
    Strict,
    Lax,
    None,
}

/// Options passed to Set-Cookie header.
/// Refer to this: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#attributes
#[derive(Default, Clone, Debug)]
pub struct CookieOptions {
    pub expires: Option<String>,
    /// The number of seconds until the cookie expires, max_age has priority over expires.
    pub max_age: Option<usize>,

    pub domain: Option<String>,

    pub path: Option<String>,

    pub secure: bool,

    pub http_only: bool,

    pub same_site: Option<SameSiteAttribute>,
}

impl CookieOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> CookieOptionsBuilder {
        CookieOptionsBuilder::new()
    }
}

impl ToString for CookieOptions {
    fn to_string(&self) -> String {
        let mut output = String::new();

        let create_cookie_string = |title: &str, value: &str| format!("{title}={value};");

        if let Some(expires) = &self.expires {
            output.push_str(&create_cookie_string("Expires", expires));
        }

        if let Some(max_age) = &self.max_age {
            output.push_str(&create_cookie_string("Max-Age", &max_age.to_string()));
        }

        if let Some(domain) = &self.domain {
            output.push_str(&create_cookie_string("Domain", &domain));
        }

        if let Some(path) = &self.path {
            output.push_str(&create_cookie_string("Path", &path));
        }

        if self.secure {
            output.push_str("Secure;");
        }

        if self.http_only {
            output.push_str("HttpOnly;");
        }

        if let Some(same_site) = &self.same_site {
            output.push_str(&create_cookie_string(
                "SameSite",
                &format!("{:#?}", same_site),
            ));
        }

        // Now let's remove the last ";" in the output.
        output = output.trim_end_matches(";").to_string();

        output
    }
}

#[derive(Default)]
pub struct CookieOptionsBuilder {
    pub options: CookieOptions,
}

impl CookieOptionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expires(mut self, expires: String) -> Self {
        self.options.expires = Some(expires);
        self
    }

    pub fn same_site(mut self, same_site: SameSiteAttribute) -> Self {
        self.options.same_site = Some(same_site);
        self
    }

    pub fn http_only(mut self, http_only: bool) -> Self {
        self.options.http_only = http_only;
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.options.path = Some(path);
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.options.secure = secure;
        self
    }

    /// Make this cookie a session cookie.
    pub fn session(mut self) -> Self {
        self.options.expires = None;
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.options.domain = Some(domain);
        self
    }

    pub fn max_age(mut self, max_age: usize) -> Self {
        self.options.max_age = Some(max_age);
        self
    }

    pub fn build(self) -> CookieOptions {
        CookieOptions {
            expires: self.options.expires,
            max_age: self.options.max_age,
            domain: self.options.domain,
            path: self.options.path,
            secure: self.options.secure,
            http_only: self.options.http_only,
            same_site: self.options.same_site,
        }
    }
}

#[cfg(test)]
mod test {
    use super::CookieOptions;

    #[test]
    fn single_works() {
        let options = CookieOptions::builder().http_only(true).build();

        assert_eq!(options.to_string(), "HttpOnly".to_string());
    }

    #[test]
    fn multiple_works() {
        let options = CookieOptions::builder()
            .http_only(true)
            .max_age(60 * 60 * 24)
            .domain("svea.rs".to_string())
            .build();

        assert_eq!(
            options.to_string(),
            "Max-Age=86400;Domain=svea.rs;HttpOnly".to_string()
        );
    }

    #[test]
    fn expires_works() {
        let options = CookieOptions::builder()
            .http_only(true)
            .expires("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
            .build();

        assert_eq!(
            options.to_string(),
            "Expires=Wed, 21 Oct 2015 07:28:00 GMT;HttpOnly".to_string()
        );
    }
}
