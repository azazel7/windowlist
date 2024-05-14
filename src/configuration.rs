use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::c_int;
use std::ffi::CString;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct CConfiguration {
    sort_by: CString,
    max_windows: i32,
    name: CString,
    name_case: CString,
    name_max_length: i32,
    name_padding: i32,
    empty_desktop_string: CString,
    separator_string: CString,
    active_window_left_click: CString,
    active_window_middle_click: CString,
    active_window_right_click: CString,
    active_window_scroll_up: CString,
    active_window_scroll_down: CString,
    inactive_window_left_click: CString,
    inactive_window_middle_click: CString,
    inactive_window_right_click: CString,
    inactive_window_scroll_up: CString,
    inactive_window_scroll_down: CString,
    active_window_fg_color: CString,
    active_window_bg_color: CString,
    active_window_ul_color: CString,
    inactive_window_fg_color: CString,
    inactive_window_bg_color: CString,
    inactive_window_ul_color: CString,
    separator_fg_color: CString,
    separator_bg_color: CString,
    separator_ul_color: CString,
    empty_desktop_fg_color: CString,
    empty_desktop_bg_color: CString,
    empty_desktop_ul_color: CString,
    overflow_fg_color: CString,
    overflow_bg_color: CString,
    overflow_ul_color: CString,
    ignored_classes: Vec<CString>,
    window_nicknames: HashMap<CString, CString>,
}
impl CConfiguration {
    pub fn to_configuration(self) -> Configuration {
        Configuration {
            sort_by: self.sort_by.into_raw(),
            max_windows: self.max_windows,
            name: self.name.into_raw(),
            name_case: self.name_case.into_raw(),
            name_max_length: self.name_max_length,
            name_padding: self.name_padding,
            empty_desktop_string: self.empty_desktop_string.into_raw(),
            separator_string: self.separator_string.into_raw(),
            active_window_left_click: self.active_window_left_click.into_raw(),
            active_window_middle_click: self.active_window_middle_click.into_raw(),
            active_window_right_click: self.active_window_right_click.into_raw(),
            active_window_scroll_up: self.active_window_scroll_up.into_raw(),
            active_window_scroll_down: self.active_window_scroll_down.into_raw(),
            inactive_window_left_click: self.inactive_window_left_click.into_raw(),
            inactive_window_middle_click: self.inactive_window_middle_click.into_raw(),
            inactive_window_right_click: self.inactive_window_right_click.into_raw(),
            inactive_window_scroll_up: self.inactive_window_scroll_up.into_raw(),
            inactive_window_scroll_down: self.inactive_window_scroll_down.into_raw(),
            active_window_fg_color: self.active_window_fg_color.into_raw(),
            active_window_bg_color: self.active_window_bg_color.into_raw(),
            active_window_ul_color: self.active_window_ul_color.into_raw(),
            inactive_window_fg_color: self.inactive_window_fg_color.into_raw(),
            inactive_window_bg_color: self.inactive_window_bg_color.into_raw(),
            inactive_window_ul_color: self.inactive_window_ul_color.into_raw(),
            separator_fg_color: self.separator_fg_color.into_raw(),
            separator_bg_color: self.separator_bg_color.into_raw(),
            separator_ul_color: self.separator_ul_color.into_raw(),
            empty_desktop_fg_color: self.empty_desktop_fg_color.into_raw(),
            empty_desktop_bg_color: self.empty_desktop_bg_color.into_raw(),
            empty_desktop_ul_color: self.empty_desktop_ul_color.into_raw(),
            overflow_fg_color: self.overflow_fg_color.into_raw(),
            overflow_bg_color: self.overflow_bg_color.into_raw(),
            overflow_ul_color: self.overflow_ul_color.into_raw(),
            ignored_classes: TomlArrayT::default(),
            window_nicknames: TomlTableT::default(),
            ignored_classes_2: self.ignored_classes,
            window_nicknames_2: self.window_nicknames,
            // ignored_classes_2: Box::into_raw(Box::new(self.ignored_classes)),
            // window_nicknames_2: Box::into_raw(Box::new(self.window_nicknames)),
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Configuration {
    pub sort_by: *mut i8,
    pub max_windows: c_int,
    pub name: *mut i8,
    pub name_case: *mut i8,
    pub name_max_length: c_int,
    pub name_padding: c_int,
    pub empty_desktop_string: *mut i8,
    pub separator_string: *mut i8,
    pub active_window_left_click: *mut i8,
    pub active_window_middle_click: *mut i8,
    pub active_window_right_click: *mut i8,
    pub active_window_scroll_up: *mut i8,
    pub active_window_scroll_down: *mut i8,
    pub inactive_window_left_click: *mut i8,
    pub inactive_window_middle_click: *mut i8,
    pub inactive_window_right_click: *mut i8,
    pub inactive_window_scroll_up: *mut i8,
    pub inactive_window_scroll_down: *mut i8,
    pub active_window_fg_color: *mut i8,
    pub active_window_bg_color: *mut i8,
    pub active_window_ul_color: *mut i8,
    pub inactive_window_fg_color: *mut i8,
    pub inactive_window_bg_color: *mut i8,
    pub inactive_window_ul_color: *mut i8,
    pub separator_fg_color: *mut i8,
    pub separator_bg_color: *mut i8,
    pub separator_ul_color: *mut i8,
    pub empty_desktop_fg_color: *mut i8,
    pub empty_desktop_bg_color: *mut i8,
    pub empty_desktop_ul_color: *mut i8,
    pub overflow_fg_color: *mut i8,
    pub overflow_bg_color: *mut i8,
    pub overflow_ul_color: *mut i8,
    pub ignored_classes: TomlArrayT,
    pub window_nicknames: TomlTableT,
    // pub ignored_classes_2: *mut Vec<CString>,
    // pub window_nicknames_2: *mut HashMap<CString, CString>,
    pub ignored_classes_2: Vec<CString>,
    pub window_nicknames_2: HashMap<CString, CString>,
}
impl Configuration {
    pub fn new(filename: String) -> Self {
        let config_string = fs::read_to_string(filename)
            .unwrap()
            .parse::<String>()
            .unwrap();
        let cconfig: CConfiguration = toml::from_str(&config_string).unwrap();
        cconfig.to_configuration()
    }
    #[export_name = "is_ignored"]
    pub fn is_class_ignored(&self, class : *mut i8) -> bool {
        false
        // bool is_ignored(char* class) {
        //     for (int i = 0; i < toml_array_len(config.ignored_classes); i++) {
        //         char* ignored_class = toml_array_string(config.ignored_classes, i).u.s;
        //         if (!strcasecmp(class, ignored_class)) {
        //             return true;
        //         }
        //     }
        //     return false;
        // }
    }
    #[export_name = "get_window_nickname"]
    pub fn get_window_nickname(&self, class : *mut i8, title : *mut i8) -> *mut i8 {
        std::ptr::null_mut()
        // char* get_window_nickname(char* class, char* title) {
        //     for (int i = 0; i < toml_table_len(config.window_nicknames); i++) {
        //         int keylen;
        //
        //         const char* key = toml_table_key(config.window_nicknames, i, &keylen);
        //         char* val;
        //
        //         if (!strcmp(config.name, "title")) {
        //             if (!strcasecmp(key, title)) {
        //                 val = toml_table_string(config.window_nicknames, key).u.s;
        //                 return val;
        //             }
        //         } else {
        //             if (!strcasecmp(key, class)) {
        //                 val = toml_table_string(config.window_nicknames, key).u.s;
        //                 return val;
        //             }
        //         }
        //     }
        //     return NULL;
        // }
        
    }
}

// toml_table_t* parse_config(char* filename, char* path) {
//     char config_path[MAX_STR_LEN];
//     snprintf(config_path, MAX_STR_LEN, "%s/%s", path, filename);
//
//     char errbuf[MAX_STR_LEN];
//
//     FILE* fp = fopen(config_path, "r");
//     toml_table_t* tbl = toml_parse_file(fp, errbuf, sizeof(errbuf));
//     fclose(fp);
//
//     config.sort_by = toml_table_string(tbl, "sort_by").u.s;
//     config.max_windows = toml_table_int(tbl, "max_windows").u.i;
//
//     config.name = toml_table_string(tbl, "name").u.s;
//     config.name_case = toml_table_string(tbl, "name_case").u.s;
//     config.name_max_length = toml_table_int(tbl, "name_max_length").u.i;
//     config.name_padding = toml_table_int(tbl, "name_padding").u.i;
//
//     config.empty_desktop_string = toml_table_string(tbl, "empty_desktop_string").u.s;
//     config.separator_string = toml_table_string(tbl, "separator_string").u.s;
//
//     config.active_window_left_click = toml_table_string(tbl, "active_window_left_click").u.s;
//     config.active_window_middle_click = toml_table_string(tbl, "active_window_middle_click").u.s;
//     config.active_window_right_click = toml_table_string(tbl, "active_window_right_click").u.s;
//     config.active_window_scroll_up = toml_table_string(tbl, "active_window_scroll_up").u.s;
//     config.active_window_scroll_down = toml_table_string(tbl, "active_window_scroll_down").u.s;
//
//     config.inactive_window_left_click = toml_table_string(tbl, "inactive_window_left_click").u.s;
//     config.inactive_window_middle_click = toml_table_string(tbl, "inactive_window_middle_click").u.s;
//     config.inactive_window_right_click = toml_table_string(tbl, "inactive_window_right_click").u.s;
//     config.inactive_window_scroll_up = toml_table_string(tbl, "inactive_window_scroll_up").u.s;
//     config.inactive_window_scroll_down = toml_table_string(tbl, "inactive_window_scroll_down").u.s;
//
//     config.active_window_fg_color = toml_table_string(tbl, "active_window_fg_color").u.s;
//     config.active_window_bg_color = toml_table_string(tbl, "active_window_bg_color").u.s;
//     config.active_window_ul_color = toml_table_string(tbl, "active_window_ul_color").u.s;
//
//     config.inactive_window_fg_color = toml_table_string(tbl, "inactive_window_fg_color").u.s;
//     config.inactive_window_bg_color = toml_table_string(tbl, "inactive_window_bg_color").u.s;
//     config.inactive_window_ul_color = toml_table_string(tbl, "inactive_window_ul_color").u.s;
//
//     config.separator_fg_color = toml_table_string(tbl, "separator_fg_color").u.s;
//     config.separator_bg_color = toml_table_string(tbl, "separator_bg_color").u.s;
//     config.separator_ul_color = toml_table_string(tbl, "separator_ul_color").u.s;
//
//     config.empty_desktop_fg_color = toml_table_string(tbl, "empty_desktop_fg_color").u.s;
//     config.empty_desktop_bg_color = toml_table_string(tbl, "empty_desktop_bg_color").u.s;
//     config.empty_desktop_ul_color = toml_table_string(tbl, "empty_desktop_ul_color").u.s;
//
//     config.overflow_fg_color = toml_table_string(tbl, "overflow_fg_color").u.s;
//     config.overflow_bg_color = toml_table_string(tbl, "overflow_bg_color").u.s;
//     config.overflow_ul_color = toml_table_string(tbl, "overflow_ul_color").u.s;
//
//     config.ignored_classes = toml_table_array(tbl, "ignored_classes");
//     config.window_nicknames = toml_table_table(tbl, "window_nicknames");
//
//     return tbl;
// }

#[repr(C)]
#[derive(Debug, Default)]
pub struct TomlArrayT {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct TomlTableT {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
