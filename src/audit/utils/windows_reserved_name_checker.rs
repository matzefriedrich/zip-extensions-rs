use std::borrow::Cow;
use std::path::PathBuf;

const DEVICE_AUXILIARY: &str = "AUX";
const DEVICE_COM1: &str = "COM1";
const DEVICE_COM2: &str = "COM2";
const DEVICE_COM3: &str = "COM3";
const DEVICE_COM4: &str = "COM4";
const DEVICE_COM5: &str = "COM5";
const DEVICE_COM6: &str = "COM6";
const DEVICE_COM7: &str = "COM7";
const DEVICE_COM8: &str = "COM8";
const DEVICE_COM9: &str = "COM9";
const DEVICE_COM_SUP_1: &str = "COM¹"; // U+00B9
const DEVICE_COM_SUP_2: &str = "COM²"; // U+00B2
const DEVICE_COM_SUP_3: &str = "COM³"; // U+00B3
const DEVICE_CONSOLE: &str = "CON";
const DEVICE_LPT1: &str = "LPT1";
const DEVICE_LPT2: &str = "LPT2";
const DEVICE_LPT3: &str = "LPT3";
const DEVICE_LPT4: &str = "LPT4";
const DEVICE_LPT5: &str = "LPT5";
const DEVICE_LPT6: &str = "LPT6";
const DEVICE_LPT7: &str = "LPT7";
const DEVICE_LPT8: &str = "LPT8";
const DEVICE_LPT9: &str = "LPT9";
const DEVICE_LPT_SUP_1: &str = "LPT¹"; // U+00B9
const DEVICE_LPT_SUP_2: &str = "LPT²"; // U+00B2
const DEVICE_LPT_SUP_3: &str = "LPT³"; // U+00B3
const DEVICE_NULL: &str = "NUL";
const DEVICE_PRINTER: &str = "PRN";

const RESERVED_DEVICE_NAMES: [&str; 28] = [
    DEVICE_CONSOLE,
    DEVICE_PRINTER,
    DEVICE_AUXILIARY,
    DEVICE_NULL,
    DEVICE_COM1,
    DEVICE_COM2,
    DEVICE_COM3,
    DEVICE_COM4,
    DEVICE_COM5,
    DEVICE_COM6,
    DEVICE_COM7,
    DEVICE_COM8,
    DEVICE_COM9,
    DEVICE_LPT1,
    DEVICE_LPT2,
    DEVICE_LPT3,
    DEVICE_LPT4,
    DEVICE_LPT5,
    DEVICE_LPT6,
    DEVICE_LPT7,
    DEVICE_LPT8,
    DEVICE_LPT9,
    DEVICE_COM_SUP_1,
    DEVICE_COM_SUP_2,
    DEVICE_COM_SUP_3,
    DEVICE_LPT_SUP_1,
    DEVICE_LPT_SUP_2,
    DEVICE_LPT_SUP_3,
];

pub fn is_windows_reserved_name(path: &PathBuf) -> bool {
    WindowsReservedNameChecker { path }.is_reserved_device_name()
}

struct WindowsReservedNameChecker<'a> {
    path: &'a PathBuf,
}

impl<'a> WindowsReservedNameChecker<'a> {
    #[inline]
    fn extract_file_name_utf8(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    #[inline]
    fn trim_trailing_dots<'b>(&self, file_name: &'b str) -> &'b str {
        file_name.trim_matches('.')
    }

    #[inline]
    fn base_name_before_first_dot<'b>(&self, file_name: &'b str) -> &'b str {
        match file_name.find('.') {
            Some(idx) => &file_name[..idx],
            None => file_name,
        }
    }

    #[inline]
    fn normalize_superscript_digits<'b>(&self, name: &'b str) -> Cow<'b, str> {
        if name.contains(['¹', '²', '³']) {
            Cow::Owned(name.replace('¹', "1").replace('²', "2").replace('³', "3"))
        } else {
            Cow::Borrowed(name)
        }
    }

    #[inline]
    fn is_reserved_device_name(&self) -> bool {
        let Some(file_name) = self.extract_file_name_utf8() else {
            return false;
        };
        let name_without_trailing_dots = self.trim_trailing_dots(file_name);
        let base_without_extension = self.base_name_before_first_dot(name_without_trailing_dots);
        let normalized = self.normalize_superscript_digits(base_without_extension);
        let uppercase = normalized.to_ascii_uppercase();
        RESERVED_DEVICE_NAMES.contains(&uppercase.as_str())
    }
}
