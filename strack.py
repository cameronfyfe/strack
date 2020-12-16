import sys
import os
import time
import subprocess
import json
from terminaltables import AsciiTable
from src.python.utils import *
from src.python.debug_log import *
from src.python.function_node import *
from src.python.config import *


def strack_analyze(args):
    # get config from config file
    config_filename = strack_path + "/in/strack_config.json"
    f_config_json = open(config_filename, "r")
    config = StrackConfig(f_config_json.read())
    f_config_json.close()

    if config.enabled is False:
        return

    # gcc object files to run anylsis on
    obj_files = args

    debug_log_deinit()

    # Create stack usage file
    start_time = time.time()
    os.system("python3 src/python/su_info.py " + su_info_filename + " " + " ".join(obj_files))
    print("Compiled stack usage in " + str(round(time.time()-start_time, 3)) + " seconds.")

    # Create call graph file
    start_time = time.time()
    os.system("python3 src/python/cg_info.py " + cg_info_filename + " " + " ".join(obj_files))
    print("Created call graph in " + str(round(time.time()-start_time, 3)) + " seconds.")

    # Analyze
    start_time = time.time()
    os.system("python3 src/python/cg_su_info.py " + node_info_filename + " " + report_filename + " " + su_info_filename + " " + cg_info_filename + " " + config_filename)
    print("Analyzed in " + str(round(time.time()-start_time, 3)) + " seconds.")

def strack_report():
    # get config from config file
    f_config_json = open(strack_path + "/in/strack_config.json", "r")
    config = StrackConfig(f_config_json.read())
    f_config_json.close()

    if config.enabled is False:
        return

    report_filename = strack_path + "/out/strack_report.json"

    f_report = open(report_filename, "r")
    report = json.loads(f_report.read())
    f_report.close()

    num_fn_nodes = report["total_function_nodes"] 
    num_fn_with_known_local_stack = report["num_functions_known_local_stack"]
    num_fn_with_known_max_stack = report["num_functions_known_max_stack"]

    pct_fn_with_known_local_stack = get_percent(num_fn_with_known_local_stack, num_fn_nodes)
    pct_fn_with_known_max_stack = get_percent(num_fn_with_known_max_stack, num_fn_nodes)

    print(
        AsciiTable([
            ["Strack Report"],
            ["Total function nodes", num_fn_nodes],
            ["Functions with known local stack usage", num_fn_with_known_local_stack, pct_fn_with_known_local_stack],
            ["Functions with known max stack usage", num_fn_with_known_max_stack, pct_fn_with_known_max_stack]
        ]).table
    )

if __name__ == "__main__":

    strack_path = sys.argv[1]
    debug_log_init(strack_path + "/local/strack_log.txt")
    strack_function = sys.argv[2]
    args = sys.argv[3:]
    
    su_info_filename = strack_path + "/local/strack_su.json"
    cg_info_filename = strack_path + "/local/strack_cg.json"
    node_info_filename = strack_path + "/out/strack_fn_nodes.json"
    report_filename = strack_path + "/out/strack_report.json"

    if strack_function == "analyze":
        strack_analyze(args)

    elif strack_function == "report":
        strack_report()
