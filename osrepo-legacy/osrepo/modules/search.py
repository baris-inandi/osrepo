from time import time
from difflib import SequenceMatcher as matcher
from collections import OrderedDict
from modules.repo import repo


def search(keyword: str, success_treshold=0.67, limit=50):
    time_start = time()
    # parse keywords to a list
    result_dict, result_list = {}, []
    for entry_id in repo:
        match = matcher(None, entry_id.lower(), keyword.lower()).ratio()
        if match >= success_treshold:
            result_dict[match] = entry_id

    # order matching entries
    ordered_index = OrderedDict(sorted(result_dict.items()))
    for i in ordered_index:
        result_list.append(result_dict[i])

    # end time
    time_end = time()
    elapsed = round(time_end - time_start, 2)
    if elapsed == 0:
        elapsed = 0.01

    result_list.reverse()
    result_list = result_list[:limit]

    return result_list, elapsed
