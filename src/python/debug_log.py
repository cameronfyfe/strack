import sys
import os


f_debug = None

def debug_log_init(f_name):
    global f_debug
    dir_name = os.path.dirname(f_name)
    os.makedirs(dir_name, exist_ok=True)
    f_debug = open(f_name, "w")

def debug_log(msg, print_to_console=False):
    global f_debug
    msg_str = str(msg) + "\r\n"
    f_debug.write(msg_str)
    if print_to_console is True:
        print(msg_str)
    return