/* automatically generated by rust-bindgen 0.64.0 */

#[doc = " Select standard memory, the same as process use."]
pub const MEMKIND_MEMTYPE_DEFAULT: memkind_memtype_t = 1;
#[doc = " Select high bandwidth memory (HBM).\n There must be at least two memories with different bandwidth to\n determine the HBM."]
pub const MEMKIND_MEMTYPE_HIGH_BANDWIDTH: memkind_memtype_t = 2;
#[doc = " \\brief Memkind memory types\n \\warning EXPERIMENTAL API"]
pub type memkind_memtype_t = ::std::os::raw::c_uint;
#[doc = " Allocate local memory.\n If there is not enough memory to satisfy the request errno is set to\n ENOMEM and the allocated pointer is set to NULL."]
pub const MEMKIND_POLICY_BIND_LOCAL: memkind_policy_t = 0;
#[doc = " Memory locality is ignored.\n If there is not enough memory to satisfy the request errno is set to\n ENOMEM and the allocated pointer is set to NULL."]
pub const MEMKIND_POLICY_BIND_ALL: memkind_policy_t = 1;
#[doc = " Allocate preferred memory that is local.\n If there is not enough preferred memory to satisfy the request or\n preferred memory is not available, the allocation will fall back on any\n other memory."]
pub const MEMKIND_POLICY_PREFERRED_LOCAL: memkind_policy_t = 2;
#[doc = " Interleave allocation across local memory.\n For n memory types the allocation will be interleaved across all of\n them."]
pub const MEMKIND_POLICY_INTERLEAVE_LOCAL: memkind_policy_t = 3;
#[doc = " Interleave allocation. Locality is ignored.\n For n memory types the allocation will be interleaved across all of\n them."]
pub const MEMKIND_POLICY_INTERLEAVE_ALL: memkind_policy_t = 4;
#[doc = " It's a special dummy value, to mark the last enum value.\n It's used to validate if policy used by the user is the proper one."]
pub const MEMKIND_POLICY_MAX_VALUE: memkind_policy_t = 5;
#[doc = " \\brief Memkind policy\n \\warning EXPERIMENTAL API"]
pub type memkind_policy_t = ::std::os::raw::c_uint;
#[doc = " Allocations backed by 2 MB page size (2^21 = 2MB)."]
pub const MEMKIND_MASK_PAGE_SIZE_2MB: memkind_bits_t = 21;
#[doc = " \\brief Memkind bits definition\n \\warning EXPERIMENTAL API\n \\note The bits specify flags and masks. Bits <0,1,2,...,7> are reserved for\n       page size, where page sizes are encoded by base-2 logarithm. If the\n       page size bits are set to zero value, than default page size will be\n       used."]
pub type memkind_bits_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct memkind {
    _unused: [u8; 0],
}
#[doc = " \\brief Memkind type definition\n \\warning EXPERIMENTAL API"]
pub type memkind_t = *mut memkind;
#[doc = " Maximum number of kinds."]
pub const MEMKIND_MAX_KIND: memkind_const = 512;
#[doc = " Error message size."]
pub const MEMKIND_ERROR_MESSAGE_SIZE: memkind_const = 128;
#[doc = " The minimum size which allows to limit the file-backed memory partition."]
pub const MEMKIND_PMEM_MIN_SIZE: memkind_const = 16777216;
#[doc = " \\brief Memkind constant values\n \\warning EXPERIMENTAL API"]
pub type memkind_const = ::std::os::raw::c_uint;
#[doc = " Default memory usage."]
pub const MEMKIND_MEM_USAGE_POLICY_DEFAULT: memkind_mem_usage_policy = 0;
#[doc = " Minimize memory usage at all costs."]
pub const MEMKIND_MEM_USAGE_POLICY_CONSERVATIVE: memkind_mem_usage_policy = 1;
#[doc = " Max memory usage policy value."]
pub const MEMKIND_MEM_USAGE_POLICY_MAX_VALUE: memkind_mem_usage_policy = 2;
#[doc = " \\brief Memkind memory usage policy"]
pub type memkind_mem_usage_policy = ::std::os::raw::c_uint;
#[doc = " Maximum number of bytes in physically resident data pages mapped."]
pub const MEMKIND_STAT_TYPE_RESIDENT: memkind_stat_type = 0;
#[doc = " Total number of bytes in active pages."]
pub const MEMKIND_STAT_TYPE_ACTIVE: memkind_stat_type = 1;
#[doc = " Total number of allocated bytes."]
pub const MEMKIND_STAT_TYPE_ALLOCATED: memkind_stat_type = 2;
#[doc = " Max memory statistics type."]
pub const MEMKIND_STAT_TYPE_MAX_VALUE: memkind_stat_type = 3;
#[doc = " \\brief Memkind memory statistics type"]
pub type memkind_stat_type = ::std::os::raw::c_uint;
#[doc = " Print all stats"]
pub const MEMKIND_STAT_PRINT_ALL: memkind_stat_print_opt = 0;
#[doc = " Print stats in JSON format"]
pub const MEMKIND_STAT_PRINT_JSON_FORMAT: memkind_stat_print_opt = 1;
#[doc = " Omit general information that never changes during execution"]
pub const MEMKIND_STAT_PRINT_OMIT_GENERAL: memkind_stat_print_opt = 2;
#[doc = " Omit merged arena statistics"]
pub const MEMKIND_STAT_PRINT_OMIT_MERGED_ARENA: memkind_stat_print_opt = 4;
#[doc = " Omit destroyed merged arena statistics"]
pub const MEMKIND_STAT_PRINT_OMIT_DESTROYED_MERGED_ARENA: memkind_stat_print_opt = 8;
#[doc = " Omit per arena statistics"]
pub const MEMKIND_STAT_PRINT_OMIT_PER_ARENA: memkind_stat_print_opt = 16;
#[doc = " Omit per size class statistics for bins"]
pub const MEMKIND_STAT_PRINT_OMIT_PER_SIZE_CLASS_BINS: memkind_stat_print_opt = 32;
#[doc = " Omit per size class statistics for large objects"]
pub const MEMKIND_STAT_PRINT_OMIT_PER_SIZE_CLASS_LARGE: memkind_stat_print_opt = 64;
#[doc = " Omit all mutex statistics"]
pub const MEMKIND_STAT_PRINT_OMIT_MUTEX: memkind_stat_print_opt = 128;
#[doc = " Omit extent statistics"]
pub const MEMKIND_STAT_PRINT_OMIT_EXTENT: memkind_stat_print_opt = 256;
#[doc = " \\brief Memory statistics print options"]
pub type memkind_stat_print_opt = ::std::os::raw::c_uint;
#[doc = " \\brief Forward declaration of memkind configuration"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct memkind_config {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = "\n \\brief Create a memkind configuration\n \\note STANDARD API\n \\return Memkind configuration, NULL on failure\n"]
    pub fn memkind_config_new() -> *mut memkind_config;
}
extern "C" {
    #[doc = "\n \\brief Delete memkind configuration\n \\note STANDARD API\n \\param cfg memkind configuration\n"]
    pub fn memkind_config_delete(cfg: *mut memkind_config);
}
extern "C" {
    #[doc = "\n \\brief Update memkind configuration with path to specified directory\n        parameter\n \\note STANDARD API\n \\param cfg memkind configuration\n \\param pmem_dir path to specified directory for PMEM kind\n"]
    pub fn memkind_config_set_path(
        cfg: *mut memkind_config,
        pmem_dir: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = "\n \\brief Update memkind configuration with PMEM kind size\n \\note STANDARD API\n \\param cfg memkind configuration\n \\param pmem_size size limit for PMEM kind\n"]
    pub fn memkind_config_set_size(cfg: *mut memkind_config, pmem_size: usize);
}
extern "C" {
    #[doc = "\n \\brief Update memkind configuration with memory usage policy parameter\n \\note STANDARD API\n \\param cfg memkind configuration\n \\param policy memkind memory usage policy\n"]
    pub fn memkind_config_set_memory_usage_policy(
        cfg: *mut memkind_config,
        policy: memkind_mem_usage_policy,
    );
}
extern "C" {
    #[doc = "\n \\brief Create kind that allocates memory with specific memory type, memory\n        binding policy and flags.\n \\warning EXPERIMENTAL API\n \\note Currently implemented memory type and policy configurations:\n       {MEMKIND_MEMTYPE_DEFAULT, MEMKIND_POLICY_PREFERRED_LOCAL},\n       {MEMKIND_MEMTYPE_HIGH_BANDWIDTH, MEMKIND_POLICY_BIND_LOCAL},\n       {MEMKIND_MEMTYPE_HIGH_BANDWIDTH, MEMKIND_POLICY_PREFERRED_LOCAL},\n       {MEMKIND_MEMTYPE_HIGH_BANDWIDTH, MEMKIND_POLICY_INTERLEAVE_ALL},\n       {MEMKIND_MEMTYPE_DEFAULT | MEMKIND_MEMTYPE_HIGH_BANDWIDTH,\n       MEMKIND_POLICY_INTERLEAVE_ALL}.\n \\param memtype_flags determine the memory types to allocate from by\n        combination of memkind_memtype_t values. This field cannot have zero\n        value.\n \\param policy specify policy for page binding to memory types selected by\n        memtype_flags. This field must be set to memkind_policy_t value. If\n        policy is set to MEMKIND_POLICY_PREFERRED_LOCAL then only one memory\n        type must be selected. Note: the value cannot be set to\n        MEMKIND_POLICY_MAX_VALUE.\n \\param flags the field must be set to a combination of memkind_bits_t\n        values.\n \\param kind pointer to kind which will be created\n \\return Memkind operation status, MEMKIND_SUCCESS on success,\n         MEMKIND_ERROR_MEMTYPE_NOT_AVAILABLE or MEMKIND_ERROR_INVALID on\n         failure\n"]
    pub fn memkind_create_kind(
        memtype_flags: memkind_memtype_t,
        policy: memkind_policy_t,
        flags: memkind_bits_t,
        kind: *mut memkind_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Destroy previously created kind object, which must have been returned\n        by a call to memkind_create_kind() or memkind_create_pmem().\n        The function has undefined behavior when the handle is invalid or\n        memkind_destroy_kind(kind) was already called before\n \\warning EXPERIMENTAL API\n \\note if the kind was returned by memkind_create_kind() all allocated memory\n       must be freed\n       before kind is destroyed, otherwise this will cause memory leak.\n \\param kind specified memory kind\n \\return Memkind operation status, MEMKIND_SUCCESS on success,\n         MEMKIND_ERROR_OPERATION_FAILED on failure\n"]
    pub fn memkind_destroy_kind(kind: memkind_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Get kind associated with allocated memory referenced by ptr\n \\note STANDARD API\n \\note This function has non-trivial performance overhead\n \\param ptr pointer to the allocated memory\n \\return Kind associated with allocated memory, NULL on failure\n"]
    pub fn memkind_detect_kind(ptr: *mut ::std::os::raw::c_void) -> memkind_t;
}
extern "C" {
    #[doc = "\n \\brief Get Memkind API version\n \\note STANDARD API\n \\return Version number represented by a single integer number(major *\n         1000000 + minor * 1000 + patch)\n"]
    pub fn memkind_get_version() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Convert error number into an error message\n \\note STANDARD API\n \\param err error number\n \\param msg error message\n \\param size size of message\n"]
    pub fn memkind_error_message(
        err: ::std::os::raw::c_int,
        msg: *mut ::std::os::raw::c_char,
        size: usize,
    );
}
extern "C" {
    #[doc = "\n \\brief Create a new PMEM (file-backed) kind of given size on top of a\n        temporary file in the given directory dir\n \\note STANDARD API\n \\param dir path to specified directory to temporary file\n \\param max_size size limit for kind\n \\param kind pointer to kind which will be created\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_create_pmem(
        dir: *const ::std::os::raw::c_char,
        max_size: usize,
        kind: *mut memkind_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Create a new PMEM kind with given memkind configuration\n \\note STANDARD API\n \\param cfg memkind configuration for specifying PMEM parameters\n \\param kind pointer to kind which will be created\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_create_pmem_with_config(
        cfg: *mut memkind_config,
        kind: *mut memkind_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Create a new kind on a fixed size map\n \\note STANDARD API\n \\param addr address of the mapping\n \\param size size of the mapping\n \\param kind pointer to kind which will be created\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_create_fixed(
        addr: *mut ::std::os::raw::c_void,
        size: usize,
        kind: *mut memkind_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Check if kind is available\n \\note STANDARD API\n \\param kind specified memory kind\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_check_available(kind: memkind_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Get capacity of memory for a given kind\n \\note STANDARD API\n \\param kind specified memory kind\n \\return Capacity on nodes from which a given kind could allocate\n         (file size or filesystem capacity in case of a file-backed PMEM\n         kind; memory area size for fixed kind) in bytes on success, -1\n         on failure\n"]
    pub fn memkind_get_capacity(kind: memkind_t) -> isize;
}
extern "C" {
    #[doc = "\n \\brief Update memkind cached statistics\n \\note STANDARD API\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_update_cached_stats() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Get memkind statistic\n \\note STANDARD API\n \\param kind specified memory kind\n \\param stat specified type of memory statistic\n \\param value reference to value of memory statistic\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_get_stat(
        kind: memkind_t,
        stat: memkind_stat_type,
        value: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Print human-readable malloc statistics\n \\note STANDARD API\n \\param write_cb pointer to a callback function which prints the statistics,\n        pass NULL to use the default one\n \\param cbopaque data passed to write_cb function\n \\param opts additional options altering the contents of statistics output\n \\return Memkind operation status, MEMKIND_SUCCESS on success,\n         MEMKIND_ERROR_INVALID on failure\n"]
    pub fn memkind_stats_print(
        write_cb: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_char,
            ),
        >,
        cbopaque: *mut ::std::os::raw::c_void,
        opts: memkind_stat_print_opt,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Allocates size bytes of uninitialized storage of the specified kind\n \\note STANDARD API\n \\param kind specified memory kind\n \\param size number of bytes to allocate\n \\return Pointer to the allocated memory\n"]
    pub fn memkind_malloc(kind: memkind_t, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = "\n \\brief Obtain size of block of memory allocated with the memkind API\n \\note STANDARD API\n \\param kind specified memory kind\n \\param ptr pointer to the allocated memory\n \\return Number of usable bytes\n"]
    pub fn memkind_malloc_usable_size(kind: memkind_t, ptr: *mut ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "\n \\brief Allocates memory of the specified kind for an array of num elements\n        of size bytes each and initializes all bytes in the allocated storage\n        to zero\n \\note STANDARD API\n \\param kind specified memory kind\n \\param num number of objects\n \\param size specified size of each element\n \\return Pointer to the allocated memory\n"]
    pub fn memkind_calloc(kind: memkind_t, num: usize, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = "\n \\brief Allocates size bytes of the specified kind and places the address of\n        the allocated memory in *memptr. The address of the allocated memory\n        will be a multiple of alignment, which must be a power of two and a\n        multiple of sizeof(void*)\n \\note EXPERIMENTAL API\n \\param kind specified memory kind\n \\param memptr address of the allocated memory\n \\param alignment specified alignment of bytes\n \\param size specified size of bytes\n \\return Memkind operation status, MEMKIND_SUCCESS on success, EINVAL or\n         ENOMEM on failure\n"]
    pub fn memkind_posix_memalign(
        kind: memkind_t,
        memptr: *mut *mut ::std::os::raw::c_void,
        alignment: usize,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Reallocates memory of the specified kind\n \\note STANDARD API\n \\param kind specified memory kind\n \\param ptr pointer to the memory block to be reallocated\n \\param size new size for the memory block in bytes\n \\return Pointer to the allocated memory\n"]
    pub fn memkind_realloc(
        kind: memkind_t,
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = "\n \\brief Free the memory space of the specified kind pointed by ptr\n \\note STANDARD API\n \\param kind specified memory kind\n \\param ptr pointer to the allocated memory\n"]
    pub fn memkind_free(kind: memkind_t, ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = "\n \\brief Try to reallocate allocation to reduce fragmentation\n \\note STANDARD API\n \\param kind specified memory kind\n \\param ptr pointer to the allocated memory\n \\return Pointer to newly transferred allocated memory\n"]
    pub fn memkind_defrag_reallocate(
        kind: memkind_t,
        ptr: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = "\n \\brief Verifies if file-backed memory kind in the specified directory can be\n        created with the DAX attribute\n \\note STANDARD API\n \\param pmem_dir path to specified directory for PMEM kind\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_check_dax_path(pmem_dir: *const ::std::os::raw::c_char)
        -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "\n \\brief Enables/disables background threads\n \\note STANDARD API\n \\param state expected state of background threads - true if enabled, false\n        if disabled\n \\return Memkind operation status, MEMKIND_SUCCESS on success, other values\n         on failure\n"]
    pub fn memkind_set_bg_threads(state: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " @brief Sets the behavior for allocations with size zero\n @param kind specified memory kind\n @param allow_zero_allocs determines returned ptr for malloc-like functions\n for allocations with size zero, return a valid ptr when set to true, NULL\n when set to false"]
    pub fn memkind_set_allow_zero_allocs(kind: memkind_t, allow_zero_allocs: bool);
}
pub const HBW_POLICY_BIND: hbw_policy_t = 1;
pub const HBW_POLICY_PREFERRED: hbw_policy_t = 2;
pub const HBW_POLICY_INTERLEAVE: hbw_policy_t = 3;
pub const HBW_POLICY_BIND_ALL: hbw_policy_t = 4;
pub type hbw_policy_t = ::std::os::raw::c_uint;
pub const HBW_PAGESIZE_4KB: hbw_pagesize_t = 1;
pub const HBW_PAGESIZE_2MB: hbw_pagesize_t = 2;
pub const HBW_PAGESIZE_MAX_VALUE: hbw_pagesize_t = 3;
pub type hbw_pagesize_t = ::std::os::raw::c_uint;
extern "C" {
    pub fn hbw_get_policy() -> hbw_policy_t;
}
extern "C" {
    pub fn hbw_set_policy(mode: hbw_policy_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hbw_check_available() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hbw_verify_memory_region(
        addr: *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hbw_malloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn hbw_calloc(num: usize, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn hbw_posix_memalign(
        memptr: *mut *mut ::std::os::raw::c_void,
        alignment: usize,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hbw_posix_memalign_psize(
        memptr: *mut *mut ::std::os::raw::c_void,
        alignment: usize,
        size: usize,
        pagesize: hbw_pagesize_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hbw_realloc(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn hbw_free(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn hbw_malloc_usable_size(ptr: *mut ::std::os::raw::c_void) -> usize;
}
