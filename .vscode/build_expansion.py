#!\bin\python3

import os
import subprocess
import re
from pprint import pprint

target = "main"

# expansion = subprocess.Popen(
#     ("cargo expand --bin " + target).split(), stdout=subprocess.PIPE)
# output, error = expansion.communicate()
output = subprocess.check_output(
    ("cargo expand --bin " + target).split()).decode("utf-8")

macro_start = None
macro_end = None
main_lines = open("src/"+target+".rs", 'r').readlines()
for i, line in enumerate(main_lines):
    if macro_start == None:
        if re.search('\#\[device_config', line):
            macro_start = i
    else:
        if re.search('struct', line):
            macro_end = i-1
            break

impl_start = None
impl_end = None
macro_lines = str(output).splitlines()
impl = []
for i, line in enumerate(macro_lines):
    if impl_start == None:
        if re.search('impl BluePill', line):
            impl_start = i
            impl.append(line)
    else:
        impl.append(line)
        if line == '}':
            impl_end = i
            break

result = []
for i, line in enumerate(main_lines):
    if i < macro_start or i > macro_end:
        result.append(line)
    if i == macro_end + 1:
        for line in impl:
            result.append(line)

with open("src/"+target+"_expanded.rs", "w")as file:
    file.writelines(result)
