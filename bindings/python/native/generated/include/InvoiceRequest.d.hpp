#ifndef _NATIVE_InvoiceRequest_D_HPP
#define _NATIVE_InvoiceRequest_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace _native {
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct Bytes; }
class Bytes;
namespace capi { struct ValidationResponse; }
class ValidationResponse;
} // namespace _native



namespace _native {
namespace capi {
    struct InvoiceRequest;
} // namespace capi
} // namespace

namespace _native {
/**
 * Owned request and decoding context; contains credentials.
 */
class InvoiceRequest {
public:

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> method() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> method_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> url() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> url_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> headers_json() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> headers_json_write(W& writeable_output) const;

  inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> body() const;

  inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> decode(uint16_t status, _native::diplomat::span<const uint8_t> body) const;

    inline const _native::capi::InvoiceRequest* AsFFI() const;
    inline _native::capi::InvoiceRequest* AsFFI();
    inline static const _native::InvoiceRequest* FromFFI(const _native::capi::InvoiceRequest* ptr);
    inline static _native::InvoiceRequest* FromFFI(_native::capi::InvoiceRequest* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceRequest() = delete;
    InvoiceRequest(const _native::InvoiceRequest&) = delete;
    InvoiceRequest(_native::InvoiceRequest&&) noexcept = delete;
    InvoiceRequest operator=(const _native::InvoiceRequest&) = delete;
    InvoiceRequest operator=(_native::InvoiceRequest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_InvoiceRequest_D_HPP
