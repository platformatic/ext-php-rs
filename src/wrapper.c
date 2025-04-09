#include "wrapper.h"

zend_string *ext_php_rs_zend_string_init(const char *str, size_t len, bool persistent) {
  return zend_string_init(str, len, persistent);
}

void ext_php_rs_zend_string_release(zend_string *zs) {
  zend_string_release(zs);
}

bool ext_php_rs_is_known_valid_utf8(const zend_string *zs) {
  return GC_FLAGS(zs) & IS_STR_VALID_UTF8;
}

void ext_php_rs_set_known_valid_utf8(zend_string *zs) {
  if (!ZSTR_IS_INTERNED(zs)) {
    GC_ADD_FLAGS(zs, IS_STR_VALID_UTF8);
  }
}

const char *ext_php_rs_php_build_id() { return ZEND_MODULE_BUILD_ID; }

void *ext_php_rs_zend_object_alloc(size_t obj_size, zend_class_entry *ce) {
  return zend_object_alloc(obj_size, ce);
}

void ext_php_rs_zend_object_release(zend_object *obj) {
  zend_object_release(obj);
}

zend_executor_globals *ext_php_rs_executor_globals() {
#ifdef ZTS
#ifdef ZEND_ENABLE_STATIC_TSRMLS_CACHE
  return TSRMG_FAST_BULK_STATIC(executor_globals_offset, zend_executor_globals);
#else
  return TSRMG_FAST_BULK(executor_globals_offset, zend_executor_globals *);
#endif
#else
  return &executor_globals;
#endif
}

php_core_globals *ext_php_rs_process_globals() {
#ifdef ZTS
#ifdef ZEND_ENABLE_STATIC_TSRMLS_CACHE
  return TSRMG_FAST_BULK_STATIC(core_globals_offset, php_core_globals);
#else
  return TSRMG_FAST_BULK(core_globals_offset, php_core_globals *);
#endif
#else
  return &core_globals;
#endif
}

sapi_globals_struct *ext_php_rs_sapi_globals() {
#ifdef ZTS
#ifdef ZEND_ENABLE_STATIC_TSRMLS_CACHE
  return TSRMG_FAST_BULK_STATIC(sapi_globals_offset, sapi_globals_struct);
#else
  return TSRMG_FAST_BULK(sapi_globals_offset, sapi_globals_struct *);
#endif
#else
  return &sapi_globals;
#endif
}

php_file_globals *ext_php_rs_file_globals() {
#ifdef ZTS
  return TSRMG_FAST_BULK(file_globals_id, php_file_globals *);
#else
  return &file_globals;
#endif
}

sapi_module_struct *ext_php_rs_sapi_module() {
  return &sapi_module;
}

bool ext_php_rs_zend_try_catch(void* (*callback)(void *), void *ctx, void **result) {
  zend_try {
    *result = callback(ctx);
  } zend_catch {
    return true;
  } zend_end_try();

  return false;
}

bool ext_php_rs_zend_first_try_catch(void* (*callback)(void *), void *ctx, void **result) {
  zend_first_try {
    *result = callback(ctx);
  } zend_catch {
    return true;
  } zend_end_try();

  return false;
}

void ext_php_rs_zend_bailout() {
  zend_bailout();
}

#include <sapi/embed/php_embed.h>

// We actually use the PHP embed API to run PHP code in test
// At some point we might want to use our own SAPI to do that
SAPI_API void* ext_php_rs_embed_callback(int argc, char** argv, void* (*callback)(void *), void *ctx) {
  void *result = NULL;

  PHP_EMBED_START_BLOCK(argc, argv)

  result = callback(ctx);

  PHP_EMBED_END_BLOCK()

  return result;
}

SAPI_API void ext_php_rs_sapi_startup() {
  #if defined(SIGPIPE) && defined(SIG_IGN)
    signal(SIGPIPE, SIG_IGN);
  #endif

  #ifdef ZTS
    php_tsrm_startup();
    // php_tsrm_startup_ex(4);
    #ifdef PHP_WIN32
      ZEND_TSRMLS_CACHE_UPDATE();
    #endif
  #endif

  zend_signal_startup();
}

SAPI_API void ext_php_rs_sapi_shutdown() {
  #ifdef ZTS
   	tsrm_shutdown();
  #endif
}

SAPI_API void ext_php_rs_sapi_per_thread_init() {
  #ifdef ZTS
    (void)ts_resource(0);
    #ifdef PHP_WIN32
      ZEND_TSRMLS_CACHE_UPDATE();
    #endif
  #endif
}

SAPI_API void ext_php_rs_sapi_check_sg() {
  const char* request_method = SG(request_info).request_method;
  if (request_method == NULL) {
    printf("SG(request_info).request_method: NULL\n");
  } else {
    printf("SG(request_info).request_method: \"%s\"\n", request_method);
  }
}
