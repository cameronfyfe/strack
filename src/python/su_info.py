import sys
import os
import time
import json
from .function_node import *


def get_fn_info_list_from_su_file(obj_filepath):
    fns = []

    su_filepath = os.path.splitext(obj_filepath)[0] + ".su"
    su_exists = os.path.isfile(su_filepath)
    if su_exists is True:
        f_su = open(su_filepath, "r")
        su_lines = f_su.read().split("\n")
        for line in su_lines:
            if "(" in line:
                # Cpp
                columns = line.split("\t")

                func_name = columns[:-2]
                su = int(columns[-2])
                su_local_type = columns[-1]

                # TODO: get return type and args from func name
                # TODO: cut func name down to name only without args

                fn = FunctionStackUsageInfo(obj_filepath, func_name)
                fn.fn_id.lang = "Cpp"
                fn.su_local = su
                fn.su_local_known = True
                fn.su_local_type = su_local_type

                fns.append(fn)
            else:
                # C
                columns = line.split("\t")
                if len(columns) is not 3: continue

                func_name = columns[0].split(":")[-1]
                su = int(columns[1])
                su_local_type = columns[2]

                fn = FunctionStackUsageInfo(obj_filepath, func_name)
                fn.fn_id.lang = "C"
                fn.su_local = su
                fn.su_local_known = True
                fn.su_local_type = su_local_type
                
                fns.append(fn)

        f_su.close()
    
    return fns 

def create_su_info_file_from_obj_files(su_filename, obj_files):
    start_time = time.time()

    # get stack usage info from .su files
    fn_sus = []
    for obj_file in obj_files:
        fn_sus.extend(get_fn_info_list_from_su_file(obj_file))

    # write stack usage info to fn_su.json
    fn_su_json = []
    for fn in fn_sus:
        fn_su_json.append(vars(fn))

    su_dir = os.path.dirname(su_filename)
    os.makedirs(su_dir, exist_ok=True)
    f_fn_nodes = open(su_filename, "w")
    f_fn_nodes.write(json.dumps(fn_su_json, indent=4, cls=FnInfoEncoder))
    f_fn_nodes.close()
