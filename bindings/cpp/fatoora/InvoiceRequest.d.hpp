#ifndef fatoora_InvoiceRequest_D_HPP
#define fatoora_InvoiceRequest_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace fatoora {
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct Bytes; }
class Bytes;
namespace capi { struct ValidationResponse; }
class ValidationResponse;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct InvoiceRequest;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Owned request and decoding context; contains credentials.
 */
class InvoiceRequest {
public:

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> method() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> method_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> url() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> url_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> headers_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> headers_json_write(W& writeable_output) const;

  inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> body() const;

  inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> decode(uint16_t status, diplomat::span<const uint8_t> body) const;

    inline const fatoora::capi::InvoiceRequest* AsFFI() const;
    inline fatoora::capi::InvoiceRequest* AsFFI();
    inline static const fatoora::InvoiceRequest* FromFFI(const fatoora::capi::InvoiceRequest* ptr);
    inline static fatoora::InvoiceRequest* FromFFI(fatoora::capi::InvoiceRequest* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceRequest() = delete;
    InvoiceRequest(const fatoora::InvoiceRequest&) = delete;
    InvoiceRequest(fatoora::InvoiceRequest&&) noexcept = delete;
    InvoiceRequest operator=(const fatoora::InvoiceRequest&) = delete;
    InvoiceRequest operator=(fatoora::InvoiceRequest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_InvoiceRequest_D_HPP
