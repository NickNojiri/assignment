import sys

def filter_lines(lines):
    filtered = []
    for line in lines:
        # Remove null bytes and bell char
        line = line.replace('\0', '').replace('\x07', '')
        stripped = line.strip()
        if not stripped: continue
        
        # Handle merged line case: "Enter the new date: ... n1(int)..."
        if "Enter the new date:" in stripped:
            parts = stripped.split("Enter the new date: (mm-dd-yy)")
            if len(parts) > 1:
                # The part after the prompt might be valid content
                content = parts[1].strip()
                if content:
                    filtered.append(content)
            continue
            
        if stripped.startswith("User Name:"): continue
        if stripped.startswith("The current date is:"): continue
        if stripped.startswith("Tue Nov 11"): continue
        filtered.append(stripped)
    return filtered

def compare_logs(file1, file2):
    with open(file1, 'r', encoding='utf-8', errors='ignore') as f1, open(file2, 'r', encoding='utf-8', errors='ignore') as f2:
        lines1 = filter_lines(f1.readlines())
        lines2 = filter_lines(f2.readlines())
    
    if len(lines1) != len(lines2):
        print(f"Line count mismatch: {len(lines1)} vs {len(lines2)}")
        
    for i in range(min(len(lines1), len(lines2))):
        if lines1[i] != lines2[i]:
            print(f"Difference at line {i+1}:")
            print(f"File 1: {repr(lines1[i])}")
            print(f"File 2: {repr(lines2[i])}")
            return
            
    if len(lines1) != len(lines2):
        print("Files differ in length.")
    else:
        print("Files match!")

if __name__ == "__main__":
    compare_logs("bigint.log", "expected.log")
