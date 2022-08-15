#![macro_use]
#![allow(unused)]

#[macro_export]
macro_rules! s {
    ($e:expr) => {
        format!("{}", $e)
    };
    (f $t:tt) => {
        format!($t)
    };
    (f $t:tt, $($e:expr), *) => {
        format!($t, $($e), *)
    };
}

#[macro_export]
macro_rules! web_log {
    ($e:expr) => {
        web_sys::console::log_1(&JsValue::from(s!($e)))
    };
    (f $t:tt) => {
        web_sys::console::log_1(&JsValue::from(s!(f $t)))
    };
    (f $t:tt, $($e:expr), *) => {
        web_sys::console::log_1(&JsValue::from(s!(f $t, $($e), *)))
    };
}

#[cfg(test)]
mod macro_test {
    #[test]
    fn test_s_from_str() {
        // Arrange
        let value = "hello";
        let expected = String::from(value);

        // Act
        let s = s!(value);

        // Assert
        assert_eq!(expected, s);
    }

    #[test]
    fn test_s_from_num() {
        // Arrange
        let value = 2000;
        let expected = value.to_string();

        // Act
        let s = s!(2000);

        // Assert
        assert_eq!(expected, s);
    }

    #[test]
    fn test_s_from_algebric() {
        // Arrange
        let value = 2000;
        let expected = value.to_string();

        // Act
        let s = s!(1500 + 500);

        // Assert
        assert_eq!(expected, s);
    }

    #[test]
    fn test_s_from_f_zero_args() {
        // Arrange
        let expected = format!("test");

        // Act
        let s = s!(f"test");

        // Assert
        assert_eq!(expected, s);
    }

    #[test]
    fn test_s_from_f_one_args() {
        // Arrange
        let expected = format!("test {}", 1);

        // Act
        let s = s!(f"test {}", 1);

        // Assert
        assert_eq!(expected, s);
    }

    #[test]
    fn test_s_from_f_two_args() {
        // Arrange
        let expected = format!("test {} - {}", 1, "one");

        // Act
        let s = s!(f"test {} - {}", 1, "one");

        // Assert
        assert_eq!(expected, s);
    }
}

