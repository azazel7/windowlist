#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <libgen.h>
#include <ctype.h>
#include <X11/Xlib.h>
#include "toml-c.h"
#include "windowlist.h"

#define MAX_STR_LEN 200

struct configuration {
    char* sort_by;
    int max_windows;

    char* name;
    char* name_case;
    int name_max_length;
    int name_padding;

    char* empty_desktop_string;
    char* separator_string;

    char* active_window_left_click;
    char* active_window_middle_click;
    char* active_window_right_click;
    char* active_window_scroll_up;
    char* active_window_scroll_down;

    char* inactive_window_left_click;
    char* inactive_window_middle_click;
    char* inactive_window_right_click;
    char* inactive_window_scroll_up;
    char* inactive_window_scroll_down;

    char* active_window_fg_color;
    char* active_window_bg_color;
    char* active_window_ul_color;

    char* inactive_window_fg_color;
    char* inactive_window_bg_color;
    char* inactive_window_ul_color;

    char* separator_fg_color;
    char* separator_bg_color;
    char* separator_ul_color;

    char* empty_desktop_fg_color;
    char* empty_desktop_bg_color;
    char* empty_desktop_ul_color;

    char* overflow_fg_color;
    char* overflow_bg_color;
    char* overflow_ul_color;

    toml_array_t* ignored_classes;
    toml_table_t* window_nicknames;
} config;

toml_table_t* parse_config(char* filename, char* path) {
    char config_path[MAX_STR_LEN];
    snprintf(config_path, MAX_STR_LEN, "%s/%s", path, filename);

    char errbuf[MAX_STR_LEN];

    FILE* fp = fopen(config_path, "r");
    toml_table_t* tbl = toml_parse_file(fp, errbuf, sizeof(errbuf));
    fclose(fp);

    config.sort_by = toml_table_string(tbl, "sort_by").u.s;
    config.max_windows = toml_table_int(tbl, "max_windows").u.i;

    config.name = toml_table_string(tbl, "name").u.s;
    config.name_case = toml_table_string(tbl, "name_case").u.s;
    config.name_max_length = toml_table_int(tbl, "name_max_length").u.i;
    config.name_padding = toml_table_int(tbl, "name_padding").u.i;

    config.empty_desktop_string = toml_table_string(tbl, "empty_desktop_string").u.s;
    config.separator_string = toml_table_string(tbl, "separator_string").u.s;

    config.active_window_left_click = toml_table_string(tbl, "active_window_left_click").u.s;
    config.active_window_middle_click = toml_table_string(tbl, "active_window_middle_click").u.s;
    config.active_window_right_click = toml_table_string(tbl, "active_window_right_click").u.s;
    config.active_window_scroll_up = toml_table_string(tbl, "active_window_scroll_up").u.s;
    config.active_window_scroll_down = toml_table_string(tbl, "active_window_scroll_down").u.s;

    config.inactive_window_left_click = toml_table_string(tbl, "inactive_window_left_click").u.s;
    config.inactive_window_middle_click = toml_table_string(tbl, "inactive_window_middle_click").u.s;
    config.inactive_window_right_click = toml_table_string(tbl, "inactive_window_right_click").u.s;
    config.inactive_window_scroll_up = toml_table_string(tbl, "inactive_window_scroll_up").u.s;
    config.inactive_window_scroll_down = toml_table_string(tbl, "inactive_window_scroll_down").u.s;

    config.active_window_fg_color = toml_table_string(tbl, "active_window_fg_color").u.s;
    config.active_window_bg_color = toml_table_string(tbl, "active_window_bg_color").u.s;
    config.active_window_ul_color = toml_table_string(tbl, "active_window_ul_color").u.s;

    config.inactive_window_fg_color = toml_table_string(tbl, "inactive_window_fg_color").u.s;
    config.inactive_window_bg_color = toml_table_string(tbl, "inactive_window_bg_color").u.s;
    config.inactive_window_ul_color = toml_table_string(tbl, "inactive_window_ul_color").u.s;

    config.separator_fg_color = toml_table_string(tbl, "separator_fg_color").u.s;
    config.separator_bg_color = toml_table_string(tbl, "separator_bg_color").u.s;
    config.separator_ul_color = toml_table_string(tbl, "separator_ul_color").u.s;

    config.empty_desktop_fg_color = toml_table_string(tbl, "empty_desktop_fg_color").u.s;
    config.empty_desktop_bg_color = toml_table_string(tbl, "empty_desktop_bg_color").u.s;
    config.empty_desktop_ul_color = toml_table_string(tbl, "empty_desktop_ul_color").u.s;

    config.overflow_fg_color = toml_table_string(tbl, "overflow_fg_color").u.s;
    config.overflow_bg_color = toml_table_string(tbl, "overflow_bg_color").u.s;
    config.overflow_ul_color = toml_table_string(tbl, "overflow_ul_color").u.s;

    config.ignored_classes = toml_table_array(tbl, "ignored_classes");
    config.window_nicknames = toml_table_table(tbl, "window_nicknames");

    return tbl;
}

void lowercase(char* str) {
    for(int i = 0; str[i]; i++) {
        str[i] = tolower(str[i]);
    }
}

void uppercase(char* str) {
    for(int i = 0; str[i]; i++) {
        str[i] = toupper(str[i]);
    }
}

int compare_window_class(const void* v1, const void* v2) {
    const struct window_props* p1 = v1;
    const struct window_props* p2 = v2;
    lowercase(p1->class);
    lowercase(p2->class);
    return strcmp(p1->class, p2->class);
}

int compare_position(const void* v1, const void* v2) {
    // Sort wlist by horizontal position on screen
    // If tied, vertical position decides (higher first)
    const struct window_props* p1 = v1;
    const struct window_props* p2 = v2;
    if (p1->x < p2->x) return -1;
    if (p1->x > p2->x) return 1;
    if (p1->y < p2->y) return -1;
    if (p1->y > p2->y) return 1;
    return 0;
}

void pad_spaces(char* window_name) {
    int n = config.name_padding;
    size_t original_length = strlen(window_name);
    memmove(window_name + n, window_name, original_length + 1);
    memset(window_name, ' ', n);
    memset(window_name + n + original_length, ' ', n);
}

bool is_unused(char* option) {
    if (option[0] == '\0' || !strcmp(option, "none")) {
        return true;
    }
    return false;
}

bool is_ignored(char* class) {
    for (int i = 0; i < toml_array_len(config.ignored_classes); i++) {
        char* ignored_class = toml_array_string(config.ignored_classes, i).u.s;
        if (!strcasecmp(class, ignored_class)) {
            return true;
        }
    }
    return false;
}

char* get_window_nickname(char* class, char* title) {
    for (int i = 0; i < toml_table_len(config.window_nicknames); i++) {
        int keylen;

        const char* key = toml_table_key(config.window_nicknames, i, &keylen);
        char* val;

        if (!strcmp(config.name, "title")) {
            if (!strcasecmp(key, title)) {
                val = toml_table_string(config.window_nicknames, key).u.s;
                return val;
            }
        } else {
            if (!strcasecmp(key, class)) {
                val = toml_table_string(config.window_nicknames, key).u.s;
                return val;
            }
        }
    }
    return NULL;
}

void print_polybar_str(char* label, char* fg_color, char* bg_color, char* ul_color,
                       char* l_click, char* m_click, char* r_click, char* scroll_up, char* scroll_down) {

    int actions_count = 0;

    if (!is_unused(l_click)) {
        printf("%%{A1:%s:}", l_click);
        actions_count++;
    }

    if (!is_unused(m_click)) {
        printf("%%{A2:%s:}", m_click);
        actions_count++;
    }

    if (!is_unused(r_click)) {
        printf("%%{A3:%s:}", r_click);
        actions_count++;
    }

    if (!is_unused(scroll_up)) {
        printf("%%{A4:%s:}", scroll_up);
        actions_count++;
    }

    if (!is_unused(scroll_down)) {
        printf("%%{A5:%s:}", scroll_down);
        actions_count++;
    }

    if (!is_unused(bg_color)) {
        printf("%%{B%s}", bg_color);
    }

    if (!is_unused(ul_color)) {
        printf("%%{u%s}%%{+u}", ul_color);
    }

    printf("%%{F%s}", fg_color);
    printf(label);
    printf("%%{F-}");

    if (!is_unused(ul_color)) {
        printf("%%{-u}");
    }

    if (!is_unused(bg_color)) {
        printf("%%{B-}");
    }

    for (int i = 0; i < actions_count; i++) {
        printf("%%{A}");
    }
}

void output(struct window_props* wlist, int n, Window active_window, char* path) {

    if (!strcmp(config.sort_by, "application")) {
        qsort(wlist, n, sizeof(struct window_props), compare_window_class);
    }
    if (!strcmp(config.sort_by, "position")) {
        qsort(wlist, n, sizeof(struct window_props), compare_position);
    }

    int window_count = 0;

    for (int i = 0; i < n; i++) {
        if (window_count > config.max_windows) {
            window_count++;
            continue;
        }

        char* class = wlist[i].class;
        char* title = wlist[i].title;
        Window wid = wlist[i].id;

        if (is_ignored(class)) {
            continue;
        }

        if (window_count > 0) {
            print_polybar_str(config.separator_string, config.separator_fg_color, config.separator_bg_color, config.separator_ul_color,
                              "none", "none", "none", "none", "none");
        }

        char window_left_click  [MAX_STR_LEN];
        char window_middle_click[MAX_STR_LEN];
        char window_right_click [MAX_STR_LEN];
        char window_scroll_up   [MAX_STR_LEN];
        char window_scroll_down [MAX_STR_LEN];

        char* window_fg_color;
        char* window_bg_color;
        char* window_ul_color;

        if (wid != active_window) {
            snprintf(window_left_click,   MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.inactive_window_left_click,   wid);
            snprintf(window_middle_click, MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.inactive_window_middle_click, wid);
            snprintf(window_right_click,  MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.inactive_window_right_click,  wid);
            snprintf(window_scroll_up,    MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.inactive_window_scroll_up,    wid);
            snprintf(window_scroll_down,  MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.inactive_window_scroll_down,  wid);
            window_fg_color = config.inactive_window_fg_color;
            window_bg_color = config.inactive_window_bg_color;
            window_ul_color = config.inactive_window_ul_color;
        } else {
            snprintf(window_left_click,   MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.active_window_left_click,   wid);
            snprintf(window_middle_click, MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.active_window_middle_click, wid);
            snprintf(window_right_click,  MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.active_window_right_click,  wid);
            snprintf(window_scroll_up,    MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.active_window_scroll_up,    wid);
            snprintf(window_scroll_down,  MAX_STR_LEN, "sh %s/click-actions/%s 0x%lx", path, config.active_window_scroll_down,  wid);
            window_fg_color = config.active_window_fg_color;
            window_bg_color = config.active_window_bg_color;
            window_ul_color = config.active_window_ul_color;
        }

        char* window_name = get_window_nickname(class, title);
        
        if (!window_name) {
            if (!strcmp(config.name, "title")) {
                window_name = malloc(strlen(title)+1 + (config.name_padding * 2) * sizeof(char));
                strcpy(window_name, title);
            } else {
                window_name = malloc(strlen(class)+1 + (config.name_padding * 2) * sizeof(char));
                strcpy(window_name, class);
            }
        }

        if (strlen(window_name) > config.name_max_length) {
            // Name is truncated
            strcpy(window_name + config.name_max_length, "‥");
        }

        if (!strcmp(config.name_case, "lowercase")) {
            lowercase(window_name);
        }
        if (!strcmp(config.name_case, "uppercase")) {
            uppercase(window_name);
        }

        pad_spaces(window_name);

        print_polybar_str(window_name, window_fg_color, window_bg_color, window_ul_color,
                          window_left_click, window_middle_click, window_right_click,
                          window_scroll_up, window_scroll_down);

        window_count++;
        free(window_name);
        free(wlist[i].class);
        free(wlist[i].title);
    }

    if (window_count == 0) {
        print_polybar_str(config.empty_desktop_string, config.empty_desktop_fg_color, config.empty_desktop_bg_color, config.empty_desktop_ul_color,
                          "none", "none", "none", "none", "none");
    }

    if (window_count > config.max_windows) {
        char overflow_string[20];
        snprintf(overflow_string, 20, "(+%d)", window_count - config.max_windows);
        print_polybar_str(overflow_string, config.overflow_fg_color, config.overflow_bg_color, config.overflow_ul_color,
                          "none", "none", "none", "none", "none");
    }

    printf("\n");
}

void configure_windows_notify(Display* d, struct window_props* prev_wlist, int prev_wlist_len, struct window_props* wlist, int n) {
    for (int i = 0; i < n; i++) {
        bool found = false;
        for (int j = 0; j < prev_wlist_len; j++) {
            if (wlist[i].id == prev_wlist[j].id) {
                found = true;
                break;
            }
        }
        if (!found) {
            XSelectInput(d, wlist[i].id, PropertyChangeMask);
        }
    }
}

void spy_root_window(Display* d, char* path) {
    XEvent e;
    Window root = DefaultRootWindow(d);

    // Asks X server to send ConfigureNotify and PropertyNotify events
    // ConfigureNotify is sent when a window's size or position changes
    // PropertyNotify for changes in client list and active window
    XSelectInput(d, root, SubstructureNotifyMask | PropertyChangeMask);

    struct window_props* prev_wlist = NULL;
    int prev_wlist_len = 0;

    for (;;) {
        fflush(stdout);
        XNextEvent(d, &e);

        long current_desktop_id = get_desktop_id(d, root, "_NET_CURRENT_DESKTOP");
        Window active_window = get_active_window(d);

        if (e.type == ConfigureNotify || e.type == PropertyNotify) {
            int n;
            struct window_props* wlist = generate_window_list(d, current_desktop_id, &n);
            configure_windows_notify(d, prev_wlist, prev_wlist_len, wlist, n);
            output(wlist, n, active_window, path);

            free(prev_wlist);
            prev_wlist = wlist;
            prev_wlist_len = n;
        }
    }
    free(prev_wlist);
}

Window str_to_wid(char* str) {
    unsigned long wid;
    if (sscanf(str, "0x%lx", &wid) != 1) {
        fputs("Cannot convert argument to number.\n", stderr);
        return EXIT_FAILURE;
    }
    return (Window) wid;
}

int main(int argc, char* argv[]) {
    char* path = dirname(argv[0]);

    Display* d = XOpenDisplay(NULL);
    toml_table_t* tbl = parse_config("config.toml", path);

    if (argc < 2) {
        // No arguments: listen to XEvents forever
        // and print the window list (output to stdout)
        spy_root_window(d, path);
    } else {
        // Arguments exist: handle on-click action
        char* action = argv[1];
        Window wid = str_to_wid(argv[2]);

        if (!strcmp(action, "--close")) {
            close_window(d, wid);
        }
        if (!strcmp(action, "--raise")) {
            raise_window(d, wid);
        }
        if (!strcmp(action, "--minimize")) {
            minimize_window(d, wid);
        }
    }

    XCloseDisplay(d);
    toml_free(tbl);
}
