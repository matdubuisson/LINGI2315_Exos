def all_substrings(string: str):
    substrings = []

    for size in range(1, len(string) + 1):
        for step in range(len(string) - size + 1):
            substrings.append(string[step:step + size])
    
    return substrings

print(all_substrings("AAB"))