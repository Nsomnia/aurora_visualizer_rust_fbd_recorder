import json
import os
import re
import argparse
import stat
from pathlib import Path

def find_assistant_contents(data):
    """
    Finds and extracts all content strings from assistant messages
    in the Qwen JSON export structure.
    """
    contents = []
    if not isinstance(data, list):
        raise TypeError("Expected the root of the JSON to be a list of chat sessions.")

    for session in data:
        try:
            messages_dict = session['chat']['history']['messages']
            if not isinstance(messages_dict, dict):
                continue
            for msg in messages_dict.values():
                if msg.get('role') == 'assistant' and 'content_list' in msg:
                    for content_item in msg['content_list']:
                        if 'content' in content_item:
                            contents.append(content_item['content'])
        except KeyError:
            continue
    return contents

def parse_content_for_files(content):
    """
    Parses a single assistant's content string for file blocks.
    It looks for the pattern '## File: <filepath>' followed by a code block.
    """
    files = {}
    file_chunks = content.split('\n\n## File: ')
    code_block_regex = re.compile(r"```(?:[a-zA-Z0-9_.-]*)?\n(.*?)```", re.DOTALL)

    for chunk in file_chunks[1:]:
        parts = chunk.split('\n', 1)
        if not parts:
            continue
        filepath = parts[0].strip()
        rest_of_chunk = parts[1] if len(parts) > 1 else ""
        match = code_block_regex.search(rest_of_chunk)
        if filepath and match:
            code_content = match.group(1).strip()
            print(f"  - Found file: {filepath}")
            files[filepath] = code_content
    return files

def create_project_structure(output_dir, files_to_create):
    """
    Creates the directories and files for the project.
    """
    if not files_to_create:
        print("\nNo files found to create. Exiting.")
        return

    output_path = Path(output_dir)
    print(f"\nBuilding project in: {output_path.resolve()}")

    for filepath, content in files_to_create.items():
        # Sanitize filepath to prevent directory traversal attacks
        safe_filepath = os.path.normpath(filepath)
        if os.path.isabs(safe_filepath) or ".." in Path(safe_filepath).parts:
            print(f"  - SKIPPING unsafe file path: {filepath}")
            continue

        # This is the core logic: join the output dir with the relative file path
        full_path = output_path / safe_filepath
        
        # Create parent directories if they don't exist
        full_path.parent.mkdir(parents=True, exist_ok=True)
        
        try:
            full_path.write_text(content + '\n', encoding='utf-8')
            print(f"  - Wrote file: {full_path}")

            # If the file is a shell script, make it executable
            if filepath.endswith('.sh'):
                full_path.chmod(stat.S_IRWXU | stat.S_IRGRP | stat.S_IXGRP | stat.S_IROTH | stat.S_IXOTH)
                print(f"  - Made executable: {full_path}")
        except IOError as e:
            print(f"  - ERROR writing file {full_path}: {e}")

def main():
    parser = argparse.ArgumentParser(
        description="Parse a Qwen AI JSON chat export and build the project file structure.",
        formatter_class=argparse.RawTextHelpFormatter
    )
    parser.add_argument("json_file", help="Path to the Qwen AI JSON chat export file.")
    parser.add_argument(
        "-o", "--output-dir", 
        default="project_output",
        help="The directory to create the project in (default: project_output)."
    )
    args = parser.parse_args()

    try:
        with open(args.json_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except Exception as e:
        print(f"Error reading or parsing JSON file '{args.json_file}': {e}")
        return

    assistant_contents = find_assistant_contents(data)
    if not assistant_contents:
        print("Could not find any content from an 'assistant' role in the JSON file.")
        return

    all_files = {}
    print(f"Found {len(assistant_contents)} assistant message(s) to parse.")
    
    for i, content in enumerate(assistant_contents):
        print(f"\nParsing content from assistant message #{i+1}...")
        found_files = parse_content_for_files(content)
        all_files.update(found_files)

    # --- THIS IS THE FIX ---
    # Directly call the creation function with the files we found and the user's output directory.
    # No more complex, faulty logic.
    create_project_structure(args.output_dir, all_files)
    
    print("\nProject generation complete.")

if __name__ == "__main__":
    main()