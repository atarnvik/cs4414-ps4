File System Overview	{#Overview}
=====================


Our file system is based on an extended linked list/tree design. We couldn't get the file system to work because Rust programming language "features" prevented the use of null pointers (among other reasons).

----------
Structure Overview
---------

The TreeNode structure has 7 fields:
 - **FileName** - cstr (either a directory name or a file name)
 - **Value** - cstr (the text of a file)
 - **isFile** - Boolean (True if a file, false if a directory)
 - **Parent** - *TreeNode (Pointer to the parent node)
 - **Prev** - *TreeNode (Pointer to previous node at this level)
 - **Next** - *TreeNode (Pointer to next node at this level)
 - **ChildrenHead** - *TreeNode (Pointer to first child node)
 - **ChildrenTail** - *TreeNode (Pointer to last child node)

----------
Diagram
---------
 
                              root
                         _____|  |_____
                        |              |
                      dir1 ---------> dir2
            ___________|___            |
           |               |          file1
          dir3-->file2-->file3              
 
----------
Methods
--------- 

- **read_file(file)**
        This method would be passed a cstr containing either the bare filename or a path to a file and would return the cstr in that file or an error if the file didn't exist.
- **write_file(file, string)**
        This method writes "string" to the specified file's value field.
- **create_file(directory, name)**
         This method would create a file in the directory "directory" with the name "name." The parent directory's ChildrenTail field would point to this file and the former ChildrenTail's next field would point to this file. This file's previous field would point to the former ChildrenTail and would have null next and ChildrenNodes.
- **delete_file(directory, name)**
        This method deletes the file in the given directory with the given name. If it is the ChildrenTail of the parent node then the ChildrenTail is redirected to the previous node. Also, the previous and next nodes are redirected appropriately.
- **get_file(directory, name)**
        Get the TreeNode pointer to the file in the given directory with the given name, or returns null.
- **list_directory(directory)**
        Iterates through the children of the specified directory, printing out the file/directory names.
- **create_directory(parent, name)**
        Creates a directory in the given parent directory with the given name. Moves the parent's ChildrenTail pointer to point to the new directory, and sets previous and next nodes accordingly. 
- **delete_directory(directory)**
        Deletes a directory if and only if it is empty. Moves pointers around appropriately.
- **get_directory(parent, name)**
        Returns a TreeNode pointer in the parent with the given name. Returns null or equivalent if the directory does not exist.

----------
Vector Addendum
--------- 
We also considered containing the child directories and child files with two vectors instead of a single linked list. This would probably been a little easier to work with.