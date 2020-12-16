import sys
import os
import subprocess
import time
import json
from function_node import *


def get_node_edge_info_list_from_object_file(obj_filepath):

    # funcs with info to return
    funcs = []

    # object name and filenames
    filepath_no_ext = os.path.splitext(obj_filepath)[0]
    cdasm_filename = filepath_no_ext + ".cdasm"

    # generate disassembly from object file
    fw_cdasm = open(cdasm_filename, "w")
    subprocess.call(["arm-none-eabi-objdump", "-drw",  obj_filepath], stdout=fw_cdasm )
    fw_cdasm.close()

    # parse .cdasm disassembly file for function call graph info
    fr_cdasm = open(cdasm_filename, "r")
    lines = fr_cdasm.read().split("\n")
    for line in lines:
        # start of new function in disassembly?
        if "00000000 <" in line:
            func_name = line[line.find("<")+1:line.find(">")]
            if func_name[0] == ".": continue # skip section symbols
            if "constprop" in func_name: func_name = ".".join(func_name.split(".")[:-1]) # handle constprop clone
            # TODO: we probably need to be able to handle more things here that gcc does
            funcs.append(FunctionEdgeInfo(obj_filepath, func_name))
            debug_log("*** New Function: " + func_name)
            continue
        # instruction for jumping to another function?
        if "f7ff fffe" in line: # TODO: update criteria to catch all branch and link events
                callee_name = line.split()[-1]
                if "constprop" in callee_name: callee_name = ".".join(callee_name.split(".")[:-1]) # handle constprop clone
                funcs[-1].children.append(callee_name)
                debug_log("*** New Callee: " + callee_name)
    fr_cdasm.close()

    # remove duplicates for functions call multiple times in same function
    for fn in funcs:
        fn.remove_duplicate_callees()
    
    return funcs

def create_cg_info_file_from_obj_files(cg_filename, obj_files):
    start_time = time.time()

    # get function node edge info from .o files
    fn_edges = []
    for obj_file in obj_files:
        fn_edges.extend(get_node_edge_info_list_from_object_file(obj_file))

    # write function node edge info to fn_node_edges.json
    fn_edges_json = []
    for fn in fn_edges:
        fn_edges_json.append(vars(fn))
    
    cg_dir = os.path.dirname(cg_filename)
    os.makedirs(cg_dir, exist_ok=True)
    f_edges_json = open(cg_filename, "w")
    f_edges_json.write(json.dumps(fn_edges_json, indent=4, cls=FnInfoEncoder))
    f_edges_json.close()


if __name__ == "__main__":
    debug_log_init("local/strack_log.txt", append=True)
    create_cg_info_file_from_obj_files(sys.argv[1], sys.argv[2:])
