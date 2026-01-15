#[cfg(test)]
mod test {
    use bubble_macro::{controller, delete, get, head, options, patch, post, put, route};

    #[controller("/api/users")]
    struct UserController {
        version: String,
    }

    impl UserController {
        pub fn new(version: String) -> Self {
            Self { version }
        }

        #[get("/")]
        fn list(&self) -> String {
            format!("[GET {}] Get user list", self.version)
        }

        #[get("/:id")]
        fn get(&self, id: i64) -> String {
            format!("[GET {}] Get user ID: {}", self.version, id)
        }

        #[post("/")]
        fn create(&self, name: String) -> String {
            format!("[POST {}] Create user: {}", self.version, name)
        }

        #[put("/:id")]
        fn update(&self, id: i64, name: String) -> String {
            format!("[PUT {}] Update user ID: {} -> {}", self.version, id, name)
        }

        #[delete("/:id")]
        fn delete(&self, id: i64) -> String {
            format!("[DELETE {}] Delete user ID: {}", self.version, id)
        }

        #[patch("/:id")]
        fn patch(&self, id: i64) -> String {
            format!("[PATCH {}] Partial update user ID: {}", self.version, id)
        }

        #[head("/:id")]
        fn check(&self, id: i64) -> String {
            format!("[HEAD {}] Check user ID: {}", self.version, id)
        }

        #[options("/:id")]
        fn options(&self, id: i64) -> String {
            format!(
                "[OPTIONS {}] User ID: {} supported methods",
                self.version, id
            )
        }

        #[route(method = "CUSTOM", path = "/custom")]
        fn custom_method(&self) -> String {
            format!("[CUSTOM {}] Custom method", self.version)
        }

        #[route("TRACE", "/trace")]
        fn trace(&self) -> String {
            format!("[TRACE {}] Trace request", self.version)
        }
    }

    #[test]
    fn mvc_test() {
        let controller = UserController::new("v1.0".to_string());
        println!("=== Test all HTTP methods ===");

        println!("1. {}", controller.list());
        println!("2. {}", controller.get(123));
        println!("3. {}", controller.create("Alice".to_string()));
        println!("4. {}", controller.update(123, "Bob".to_string()));
        println!("5. {}", controller.delete(123));
        println!("6. {}", controller.patch(123));
        println!("7. {}", controller.check(123));
        println!("8. {}", controller.options(123));
        println!("9. {}", controller.custom_method());
        println!("10. {}", controller.trace());
        println!("=== Test completed ===");
    }
}
