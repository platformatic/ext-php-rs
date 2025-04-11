// PHP for Windows uses the `vectorcall` calling convention on some functions.
// This is guarded by the `ZEND_FASTCALL` macro, which is set to `__vectorcall`
// on Windows and nothing on other systems.
//
// However, `ZEND_FASTCALL` is only set when compiling with MSVC and the PHP
// source code checks for the __clang__ macro and will not define `__vectorcall`
// if it is set (even on Windows). This is a problem as Bindgen uses libclang to
// generate bindings. To work around this, we include the header file containing
// the `ZEND_FASTCALL` macro but not before undefining `__clang__` to pretend we
// are compiling on MSVC.
#if defined(_MSC_VER) && defined(__clang__)
#undef __clang__
#include "zend_portability.h"
#define __clang__
#endif

#include "php.h"
#include "php_ini_builder.h"

#include "ext/standard/info.h"
#include "ext/standard/php_var.h"
#include "ext/standard/file.h"
#include "zend_exceptions.h"
#include "zend_inheritance.h"
#include "zend_interfaces.h"
#include "php_variables.h"
#include "zend_ini.h"
#include "main/SAPI.h"

#ifdef ZTS
#include "TSRM.h"
#endif

zend_string *ext_php_rs_zend_string_init(const char *str, size_t len, bool persistent);
void ext_php_rs_zend_string_release(zend_string *zs);
bool ext_php_rs_is_known_valid_utf8(const zend_string *zs);
void ext_php_rs_set_known_valid_utf8(zend_string *zs);

const char *ext_php_rs_php_build_id();
void *ext_php_rs_zend_object_alloc(size_t obj_size, zend_class_entry *ce);
void ext_php_rs_zend_object_release(zend_object *obj);
zend_executor_globals *ext_php_rs_executor_globals();
php_core_globals *ext_php_rs_process_globals();
sapi_globals_struct *ext_php_rs_sapi_globals();
php_file_globals *ext_php_rs_file_globals();
sapi_module_struct *ext_php_rs_sapi_module();
bool ext_php_rs_zend_try_catch(void* (*callback)(void *), void *ctx, void **result);
bool ext_php_rs_zend_first_try_catch(void* (*callback)(void *), void *ctx, void **result);
void ext_php_rs_zend_bailout();

#include "zend.h"
#include "sapi/embed/php_embed.h"

#if defined(ZTS) && defined(PHP_WIN32)
ZEND_TSRMLS_CACHE_DEFINE()
#endif

SAPI_API void* ext_php_rs_embed_callback(int argc, char** argv, void* (*callback)(void *), void *ctx);
SAPI_API void ext_php_rs_sapi_startup();
SAPI_API void ext_php_rs_sapi_shutdown();
SAPI_API void ext_php_rs_sapi_per_thread_init();
SAPI_API void ext_php_rs_sapi_check_sg();
