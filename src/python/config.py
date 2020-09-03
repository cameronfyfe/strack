import json

class StrackConfig:
    
    def __init__(self, config_json_text):
        config_json = json.loads(config_json_text)

        self.enabled = config_json.get("enabled", True)
        self.frame_cost = config_json.get("frame_cost", 0)
        self.allow_function_ptrs = config_json.get("allow_function_ptrs", True)
        self.allow_recursion = config_json.get("allow_recursion", True)
        self.tracked_functions = config_json.get("tracked_functions", [])
