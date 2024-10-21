use crate::metacall_value::MetacallValueID;

extern "C" {
    /// Calls an object method anonymously using an array of arguments.
    ///
    /// This function assumes no method overloading and performs type conversion on the values.
    ///
    /// # Parameters
    ///
    /// - `obj`: Pointer to the object.
    /// - `name`: Name of the method to call.
    /// - `args`: Array of pointers to the method's arguments.
    /// - `size`: Number of elements in the `args` array.
    ///
    /// # Returns
    ///
    /// Pointer to the result of the method call.
    pub fn metacallv_object(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        args: *mut *mut ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;

    /// Calls an object method anonymously with an expected return type, aiding in method overloading resolution.
    ///
    /// # Parameters
    ///
    /// - `obj`: Pointer to the object.
    /// - `name`: Name of the method to call.
    /// - `ret`: Expected return value type.
    /// - `args`: Array of pointers to the method's arguments.
    /// - `size`: Number of elements in the `args` array.
    ///
    /// # Returns
    ///
    /// Pointer to the result of the method call.
    pub fn metacallt_object(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        ret: MetacallValueID,
        args: *mut *mut ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;

    /// Retrieves an attribute from an object by its key name.
    ///
    /// # Parameters
    ///
    /// - `obj`: Pointer to the object.
    /// - `key`: Name of the attribute to retrieve.
    ///
    /// # Returns
    ///
    /// Pointer to the object attribute value, or `NULL` if an error occurred.
    pub fn metacall_object_get(
        obj: *mut ::std::os::raw::c_void,
        key: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;

    /// Sets an attribute on an object by its key name.
    ///
    /// # Parameters
    ///
    /// - `obj`: Pointer to the object.
    /// - `key`: Name of the attribute to set.
    /// - `value`: Value to set.
    ///
    /// # Returns
    ///
    /// A non-zero integer if an error occurred.
    pub fn metacall_object_set(
        obj: *mut ::std::os::raw::c_void,
        key: *const ::std::os::raw::c_char,
        v: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    /// Retrieves the value contained within a throwable object.
    ///
    /// # Parameters
    ///
    /// - `th`: Pointer to the throwable object.
    ///
    /// # Returns
    ///
    /// Pointer to the value inside the throwable, or `NULL` in case of an error.
    pub fn metacall_throwable_value(th: *mut ::std::os::raw::c_void)
        -> *mut ::std::os::raw::c_void;

    /// Provides information about all loaded objects.
    ///
    /// # Parameters
    ///
    /// - `size`: Size in bytes of the return buffer (output parameter).
    /// - `allocator`: Pointer to the allocator that will allocate the string.
    ///
    /// # Returns
    ///
    /// A string containing introspection information about the loaded objects.
    pub fn metacall_inspect(
        size: *mut usize,
        allocator: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_char;

    /// Provides information about all loaded objects as a value.
    ///
    /// # Returns
    ///
    /// Pointer to a value containing introspection information.
    pub fn metacall_inspect_value() -> *mut ::std::os::raw::c_void;
}
