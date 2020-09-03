import sys
import os


f_debug = None

def debug_log_init(f_name, append=False):
    global f_debug
    dir_name = os.path.dirname(f_name)
    os.makedirs(dir_name, exist_ok=True)
    if append is True:
        f_debug = open(f_name, "a")
    else:
        f_debug = open(f_name, "w")

def debug_log_deinit():
    global f_debug
    f_debug.close()

def debug_log(msg, print_to_console=False):
    global f_debug
    msg_str = str(msg)
    f_debug.write(msg_str)
    if print_to_console is True:
        print(msg_str)
    return