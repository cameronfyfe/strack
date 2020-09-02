import sys
import os
import subprocess
import time
import json
from src.python.utils import *
from src.python.debug_log import *
from src.python.function_node import *


def callpath_has_recursion(callpath):
    callpath_no_duplicates = list(dict.fromkeys(callpath))
    return True if len(callpath_no_duplicates) != len(callpath) else False

def get_callpath_text(callpath):
    text = callpath[0]
    for call in callpath[1:]:
        text += " -> " + call
    return text

def get_index_for_matching_fn(fns, fn):
    for i in range(0, len(fns)):
        if fn.fn_id.matches(fns[i].fn_id) is True:
            return i

def get_index_by_obj_and_symbol_name(fns, fn, symbol):
    # use symbol local to compile unit first
    for i in range(0, len(fns)):
        if fns[i].fn_id.symbol == symbol and fns[i].fn_id.obj_filepath == fn.fn_id.obj_filepath:
            return i
    # use any symbol match if it doesn't exist locally
    for i in range(0, len(fns)):
        if fns[i].fn_id.symbol == symbol:
            return i

def get_index_by_symbol_name(fns, symbol):
    for i in range(0, len(fns)):
        if fns[i].fn_id.symbol == symbol:
            return i

def compute_max_su_for_fn(fn, funcs_list, callpath, config, cms_ind):
        debug_log(cms_ind + "Computing max SU for " + fn.fn_id.name)

        # already computed max su for this node
        if fn.su_max is not -1:
            debug_log(cms_ind + "(" + str(fn.su_max) + ")")
            return

        # this node is a sink node on call graph (no further function calls), local su is max su
        if len(fn.children) is 0:
            fn.su_max = fn.su_local
            fn.su_max_known = True
            debug_log(cms_ind + "(" + str(fn.su_max) + ")")
            return

        # use max of child nodes
        max_child_su_func = None
        for child_name in fn.children:
            
            child_callpath = callpath[:]
            child_callpath.append(fn.fn_id.name)
            # recursion in callpath
            if callpath_has_recursion(child_callpath) is True:
                print_error = True if config.allow_recursion is False else False
                debug_log(cms_ind + "RECURSION PATH DETECTED!!!", print_error)
                debug_log(cms_ind + get_callpath_text(child_callpath), print_error)
                if config.allow_recursion is False:
                    strack_die()
                else:
                    fn.su_max = 99000000
                    fn.su_max_known = False
                    debug_log(cms_ind + "(" + str(fn.su_max) + ")")
                    return

            ci = get_index_by_obj_and_symbol_name(funcs_list, fn, child_name)
            if ci is None:
                fn.children_missing.append(child_name)
                continue

            compute_max_su_for_fn(funcs_list[ci], funcs_list, child_callpath, config, cms_ind+"  ")
            debug_log(cms_ind + "-" + child_name + " (" + str(funcs_list[ci].su_max) + ")")
            if max_child_su_func is None:
                max_child_su_func = funcs_list[ci]
            elif funcs_list[ci].su_max > max_child_su_func.su_max:
                max_child_su_func = funcs_list[ci]

        # check if we found a child node for max su
        if max_child_su_func is None:
            fn.su_max_known = False
            debug_log("Error: function node is not sink but no children found.")
            return

        # record callpath for max su from this node
        fn.su_max_callpath.append(max_child_su_func.fn_id.name)
        fn.su_max_callpath.extend(max_child_su_func.su_max_callpath)
        # compute max su
        fn.su_max = fn.su_local + config.frame_cost + max_child_su_func.su_max
        # check if max su is known or unknown based on callpath
        if max_child_su_func.su_max_known is True and fn.su_local_known is True:
            fn.su_max_known = True
        else:
            fn.su_max_known = False
        debug_log(cms_ind + "(" + str(fn.su_max) + ")")

def analyze_cg_and_su(nodes_fname, report_fname, su_fname, cg_fname, config):
    start_time = time.time()
    funcs = []

    # create list of function nodes for all functions with edge info
    fr_fn_edges = open(cg_fname, "r")
    fn_edges = json.loads(fr_fn_edges.read(), object_hook=FunctionEdgeInfo.json_decode)
    for fn in fn_edges:
        node = FunctionNode("", "")
        fn.copy_info_to(node)
        funcs.append(node)

    # get stack usage for nodes from fn_su.json
    fr_fn_nodes = open(su_fname, "r")
    fn_sus = json.loads(fr_fn_nodes.read(), object_hook=FunctionStackUsageInfo.json_decode)
    for fn in fn_sus:
        # get matching function
        fi = get_index_for_matching_fn(funcs, fn)
        if fi is None: 
            continue
        # copy stack usage info to node
        if (fn.su_local is False):
            print(fn.fn_id.name)
        fn.copy_info_to(funcs[fi])

    fr_fn_nodes.close()

    # compute max stack usage for each function node
    for fn in funcs:
        # compute_max_su is recursive
        compute_max_su_for_fn(fn, funcs, [], config, "")

    # create function node info json file
    fn_json = []
    for fn in funcs:
        fn_json.append(vars(fn))

    nodes_dir = os.path.dirname(nodes_fname)
    os.makedirs(nodes_dir, exist_ok=True)
    f_fn_nodes = open(nodes_fname, "w")
    f_fn_nodes.write(json.dumps(fn_json, indent=4, cls=FnInfoEncoder))
    f_fn_nodes.close()

    # TODO: clean up the following garbage, atleast get rid of duplicated dictionary indices
    # create strack report json
    report_dir = os.path.dirname(report_fname)
    os.makedirs(report_dir, exist_ok=True)
    f_report = open(report_fname, "w")
    report = {}
    # report summary
    report["total_function_nodes"] = len(funcs)
    report["num_functions_known_local_stack"] = sum(fn.su_local_known is True for fn in funcs)
    report["num_functions_known_max_stack"] = sum(fn.su_max_known is True for fn in funcs)
    # info for tracked functions specified in config
    report["tracked_functions"] = []
    for func in config.tracked_functions:
        data = {}
        data["name"] = func
        fi = get_index_by_symbol_name(funcs, func)
        if fi is None:
            data["su_max"] = -1
        else:
            data["su_max"] = funcs[fi].su_max
        report["tracked_functions"].append(data)
    # functions with unknown local stack usage
    report["unknown_local_su"] = []
    for fn in funcs:
        if fn.su_local_known is False: report["unknown_local_su"].append(fn.fn_id.name)
    # functions with unknown max stack usage
    report["unknown_max_su"] = []
    for fn in funcs:
        if fn.su_max_known is False: report["unknown_max_su"].append(fn.fn_id.name)
    # missing children from all nodes
    report["missing_children"] = []
    for fn in funcs:
        for missing_child in fn.children_missing:
            if missing_child not in report["missing_children"]:
                report["missing_children"].append(missing_child)

    f_report.write(json.dumps(report, indent=4))
    f_report.close()

