from json import JSONEncoder

def cpp_demangle(symbol):
    # TODO
    return symbol

# Basic function info
class FunctionInfo:

    def __init__(self, obj_filepath, symbol):
        self.name = cpp_demangle(symbol)                # function name (demangled)
        self.symbol = symbol                            # function symbol
        self.obj_filepath = obj_filepath                # gcc object name
        self.obj_filename = obj_filepath.split("/")[-1] # path to gcc object file
        self.lang = ""                                  # language ("C" or "Cpp")
        self.return_type = ""                           # return type of function
        self.arg_types = []                             # arg types of function

    def copy_info_to(self, fn):
        fn.__dict__.update(self.__dict__)

    def matches(self, fn):
        if self.lang == "Cpp":
            # TODO
            return False
        else:
            if self.name == fn.name and self.obj_filepath == fn.obj_filepath:
                return True
            else:
                return False

    @staticmethod
    def can_decode_dict(obj):
        if obj.get("name") is None: return False
        if obj.get("obj_filepath") is None: return False
        if obj.get("obj_filename") is None: return False
        return True

    @staticmethod
    def json_decode(obj):
        fn = FunctionInfo("", "")
        fn.__dict__.update(obj)
        return fn

# Local stack usage info for function nodes
class FunctionStackUsageInfo:

    def __init__(self, obj_filepath, name):
        self.fn_id = FunctionInfo(obj_filepath, name)

        self.su_local = 0           # stack frame size for function
        self.su_local_known = False # local stack usage is known
        self.su_local_type = ""     # type from gcc .su file for this function (should be 'static')

    def copy_info_to(self, fn):
        fn.fn_id.copy_info_to(self.fn_id)
        fn.su_local = self.su_local
        fn.su_local_known = self.su_local_known
        fn.su_local_type = self.su_local_type

    @staticmethod
    def json_decode(obj):
        if FunctionInfo.can_decode_dict(obj) is True:
            return FunctionInfo.json_decode(obj)
        else:
            fn = FunctionStackUsageInfo("", "")
            fn.__dict__.update(obj)
            return fn

# Local edge info for function nodes (which other functions it calls)
class FunctionEdgeInfo:

    def __init__(self, obj_filepath, name):
        self.fn_id = FunctionInfo(obj_filepath, name)

        self.children = [] # functions called by this function

    def copy_info_to(self, fn):
        self.fn_id.copy_info_to(fn.fn_id)
        fn.children = self.children

    def remove_duplicate_callees(self):
        self.children = list(dict.fromkeys(self.children))

    @staticmethod
    def json_decode(obj):
        if FunctionInfo.can_decode_dict(obj) is True:
            return FunctionInfo.json_decode(obj)
        else:
            fn = FunctionEdgeInfo("", "")
            fn.__dict__.update(obj)
            return fn

# Function class holds all info we care about for functions in context of stack usage
# This holds info for each node of the call graph and how much stack each node uses
class FunctionNode(FunctionStackUsageInfo, FunctionEdgeInfo):

    def __init__(self, obj_filepath, name):
        FunctionStackUsageInfo.__init__(self, obj_filepath, name)
        FunctionEdgeInfo.__init__(self, obj_filepath, name)

        self.children_missing = []  # function name with no corresponding node in callgraph (memcpy, etc)
        self.su_max = -1            # max stack used starting from this function
        self.su_max_known = False   # recursion or function pointers can make stack usage unknown
        self.su_max_callpath = []   # call path for max stack usage from this function

    def get_info_text(self):
        info = self.fn_id.name + " (su: " + str(self.su_local) + ")" + "\r\n"
        for callee in self.children:
            info += "|-- " + callee + "\r\n"
        return info


# Json encoder for these class
class FnInfoEncoder(JSONEncoder):
    def default(self, o):
        return o.__dict__