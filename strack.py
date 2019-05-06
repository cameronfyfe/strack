import sys
import os
import subprocess
import json


# Function class holds all info we care about for functions in context of stack usage
# This holds info for each node of the call graph and how much stack each node uses
class FunctionNode:

    def __init__(self, obj_file, name):
        # obj file name included to handle duplicate symbol cases
        self.obj_file = obj_file
        self.obj_name = obj_file.split("/")[-1]
        # function symbol name
        self.name = name
        # functions called by this function
        self.children = []
        # stack usage info
        self.su_local = 0           # stack frame size for function
        self.su_local_known = False # local stack usage is known
        self.su_local_type = ""     # type from gcc .su file for this function (should be 'static')
        self.su_max = -1            # max stack used starting from this function
        self.su_max_known = False   # recursion or function pointers can make stack usage unknown
        self.su_max_callpath = []   # call path for max stack usage from this function

    def remove_duplicate_callees(self):
        self.children = list(dict.fromkeys(self.children))

    def compute_max_su(self, funcs_list, callpath, cms_ind):
        debug_print(cms_ind + "Computing max SU for " + self.name)

        # already computed max su for this node
        if self.su_max is not -1:
            debug_print(cms_ind + "(" + str(self.su_max) + ")")
            return

        # this node is a sink node on call graph (no further function calls), local su is max su
        if len(self.children) is 0:
            self.su_max = self.su_local
            self.su_max_known = True
            debug_print(cms_ind + "(" + str(self.su_max) + ")")
            return

        # use max of child nodes
        max_child_su_func = None
        for child_name in self.children:
            
            child_callpath = callpath[:]
            child_callpath.append(self.name)
            if callpath_has_recursion(child_callpath) is True:
                debug_print(cms_ind + "RECURSION PATH DETECTED!!!")
                debug_print(cms_ind + get_callpath_text(child_callpath))
                self.su_max = 99000000
                self.su_max_known = False
                debug_print(cms_ind + "(" + str(self.su_max) + ")")
                return

            ci = get_index_for_func_name(funcs_list, child_name)
            if ci is -1: continue

            funcs_list[ci].compute_max_su(funcs_list, child_callpath, cms_ind+"  ")
            debug_print(cms_ind + "-" + child_name + " (" + str(funcs_list[ci].su_max) + ")")
            if max_child_su_func is None:
                max_child_su_func = funcs_list[ci]
            elif funcs_list[ci].su_max > max_child_su_func.su_max:
                max_child_su_func = funcs_list[ci]

        # check if we found a child node for max su
        if max_child_su_func is None:
            self.su_max_known = False
            debug_print("Error: function node is not sink but no children found.")
            return

        # record callpath for max su from this node
        self.su_max_callpath.append(max_child_su_func.name)
        self.su_max_callpath.extend(max_child_su_func.su_max_callpath)
        # compute max su
        self.su_max = self.su_local + max_child_su_func.su_max
        if max_child_su_func.su_max_known is True and self.su_local_known is True:
            self.su_max_known = True
        else:
            self.su_max_known = False
        debug_print(cms_ind + "(" + str(self.su_max) + ")")

    def get_info_text(self):
        info = self.name + " (su: " + str(self.su_local) + ")" + "\r\n"
        for callee in self.children:
            info += "|-- " + callee + "\r\n"
        return info

    def get_su_max_callpath_text(self):
        return get_callpath_text(self.su_max_callpath)

    def get_json(self):
        return vars(self)


def callpath_has_recursion(callpath):
    callpath_no_duplicates = list(dict.fromkeys(callpath))
    if len(callpath_no_duplicates) is not len(callpath):
        return True
    else:
        return False

def get_callpath_text(callpath):
    text = callpath[0]
    for call in callpath[1:]:
        text += " -> " + call
    return text

def get_index_for_func_name(funcs, name):
    for i in range(0, len(funcs)):
        if funcs[i].name == name:
            return i
    return -1

def debug_print(msg):
    print(msg)
    return


def get_func_nodes_from_obj_file(obj_filename):

    # funcs with info to return
    funcs = []

    # object name and filenames
    obj_base_name = os.path.splitext(obj_filename)[0]
    cdasm_filename = obj_base_name + ".cdasm"
    su_filename = obj_base_name + ".su"

    # generate disassembly from object file
    fw_cdasm = open(cdasm_filename, "w")
    subprocess.call(["arm-none-eabi-objdump", "-drw",  obj_filename], stdout=fw_cdasm )
    fw_cdasm.close()

    # parse .cdasm disassembly file for function call graph info
    fr_cdasm = open(cdasm_filename, "r")
    lines = fr_cdasm.read().split("\n")
    for line in lines:
        # start of new function in disassembly?
        if "Disassembly of section .text." in line:
            func_name = line[line.find(".text.")+6:line.find(":")]
            funcs.append(FunctionNode(obj_base_name, func_name))
            debug_print("*** New Function: " + func_name)
            continue
        # instruction for jumping to another function?
        if "f7ff fffe" in line: # TODO: update criteria to catch all branch and link events
                callee_name = line.split()[-1]
                funcs[-1].children.append(callee_name)
                debug_print("*** New Callee: " + callee_name)
    fr_cdasm.close()

    # get stack usage of functions from .su file
    su_exists = os.path.isfile(su_filename)
    if su_exists is True:
        f_su = open(su_filename, "r")
        su_lines = f_su.read().split("\n")
        debug_print(su_lines)
        for line in su_lines:
            columns = line.split("\t")
            if len(columns) is not 3: continue

            func_name = columns[0]
            func_name = func_name.split(":")[-1]
            su = int(columns[1])
            su_local_type = columns[2]

            fi = get_index_for_func_name(funcs, func_name)
            if fi is -1: continue 
            funcs[fi].su_local = su
            funcs[fi].su_local_known = True
            funcs[fi].su_local_type = su_local_type

        f_su.close()

    # remove duplicates for functions call multiple times in same function
    for func in funcs:
        func.remove_duplicate_callees()

    return funcs


def main(argv):
    # read in config
    f_config_json = open("strack/strack_config.json", "r")
    config_json = json.loads(f_config_json.read())
    f_config_json.close()

    # get function node info from .o/.su files
    funcs = []
    obj_files = argv[1:]
    for obj_file in obj_files:
        funcs.extend(get_func_nodes_from_obj_file(obj_file))

    # compute max stack usage for each function node
    for func in funcs:
        func.compute_max_su(funcs, [], "")

    # create function node info json file
    function_json = []
    for func in funcs:
        function_json.append(func.get_json())
    f_func_json = open("strack/strack_fn_nodes.json", "w")
    f_func_json.write(json.dumps(function_json, indent=4))
    f_func_json.close()

    # create strack report json
    f_report = open("strack/strack_report.json", "w")
    tracked_func_list = []
    for func in config_json["tracked_functions"]:
        data = {}
        data["name"] = func
        fi = get_index_for_func_name(funcs, func)
        if fi is -1:
            data["su_max"] = -1
        else:
            data["su_max"] = funcs[fi].su_max
        tracked_func_list.append(data)
    f_report.write(json.dumps(tracked_func_list, indent=4))
    f_report.close()


if __name__ == "__main__":
    main(sys.argv)
