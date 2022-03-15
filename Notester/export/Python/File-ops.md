# Python.File-ops --

    close()  
    Close the file. A closed file cannot be read or written anymore. Any operation which requires that the file be open will raise a ValueError after the file has been closed. Calling close() more than once is allowed.  
     
    flush()  
    Flush the internal buffer, like stdio's fflush(). This may be a no-op on some file-like objects.  
     
    isatty()  
    Return true if the file is connected to a tty(-like) device, else false. Note: If a file-like object is not associated with a real file, this method should not be implemented.  
     
    fileno()  
      Return the integer ``file descriptor'' that is used by the underlying implementation to request I/O operations from the operating system. This can be useful for other, lower level interfaces that use file descriptors, such as the fcntl module or os.read() and friends. Note: File-like objects which do not have a real file descriptor should not provide this method!  
     
    read([size])  
    Read at most size bytes from the file (less if the read hits EOF before obtaining size bytes). If the size argument is negative or omitted, read all data until EOF is reached. The bytes are returned as a string object. An empty string is returned when EOF is encountered immediately. (For certain files, like ttys, it makes sense to continue reading after an EOF is hit.) Note that this method may call the underlying C function fread() more than once in an effort to acquire as close to size bytes as possible.  
     
    readline([size])  
    Read one entire line from the file. A trailing newline character is kept in the string2.11 (but may be absent when a file ends with an incomplete line). If the size argument is present and non-negative, it is a maximum byte count (including the trailing newline) and an incomplete line may be returned. An empty string is returned when EOF is hit immediately. Note: Unlike stdio's fgets(), the returned string contains null characters ('\0') if they occurred in the input.  
     
    readlines([sizehint])  
    Read until EOF using readline() and return a list containing the lines thus read. If the optional sizehint argument is present, instead of reading up to EOF, whole lines totalling approximately sizehint bytes (possibly after rounding up to an internal buffer size) are read. Objects implementing a file-like interface may choose to ignore sizehint if it cannot be implemented, or cannot be implemented efficiently.  
     
    xreadlines()  
    Equivalent to xreadlines.xreadlines(file).  (See the xreadlines module for more information.) New in version 2.1.  
     
    seek(offset[, whence])  
    Set the file's current position, like stdio's fseek(). The whence argument is optional and defaults to 0 (absolute file positioning); other values are 1 (seek relative to the current position) and 2 (seek relative to the file's end). There is no return value. Note that if the file is opened for appending (mode 'a' or 'a+'), any seek() operations will be undone at the next write. If the file is only opened for writing in append mode (mode 'a'), this method is essentially a no-op, but it remains useful for files opened in append mode with reading enabled (mode 'a+').  
     
    tell()  
    Return the file's current position, like stdio's ftell().  
     
    truncate([size])  
    Truncate the file's size. If the optional size argument present, the file is truncated to (at most) that size. The size defaults to the current position. Availability of this function depends on the operating system version (for example, not all Unix versions support this operation).  
     
    write(str)  
    Write a string to the file. There is no return value. Due to buffering, the string may not actually show up in the file until the flush() or close() method is called.  
     
    writelines(sequence)  
    Write a sequence of strings to the file. The sequence can be any iterable object producing strings, typically a list of strings. There is no return value. (The name is intended to match readlines(); writelines() does not add line separators.)  
    Files support the iterator protocol. Each iteration returns the same result as file.readline(), and iteration ends when the readline() method returns an empty string.  
     
    File objects also offer a number of other interesting attributes. These are not required for file-like objects, but should be implemented if they make sense for the particular object.  
     
     
    closed  
    Boolean indicating the current state of the file object. This is a read-only attribute; the close() method changes the value. It may not be available on all file-like objects.  
     
    mode  
    The I/O mode for the file. If the file was created using the open() built-in function, this will be the value of the mode parameter. This is a read-only attribute and may not be present on all file-like objects.  
     
    name  
    If the file object was created using open(), the name of the file. Otherwise, some string that indicates the source of the file object, of the form "<...>". This is a read-only attribute and may not be present on all file-like objects.  
     
    softspace  
    Boolean that indicates whether a space character needs to be printed before another value when using the print statement. Classes that are trying to simulate a file object should also have a writable softspace attribute, which should be initialized to zero. This will be automatic for most classes implemented in Python (care may be needed for objects that override attribute access); types implemented in C will have to provide a writable softspace attribute. Note: This attribute is not used to control the print statement, but to allow the implementation of print to keep track of its internal state.  
     
     
     
     
     
