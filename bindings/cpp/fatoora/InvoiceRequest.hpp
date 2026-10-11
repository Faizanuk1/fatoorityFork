#ifndef fatoora_InvoiceRequest_HPP
#define fatoora_InvoiceRequest_HPP

#include "InvoiceRequest.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "BindingError.hpp"
#include "Bytes.hpp"
#include "ValidationResponse.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_InvoiceRequest_method_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_method_result;
    fatoora_InvoiceRequest_method_result fatoora_InvoiceRequest_method(const fatoora::capi::InvoiceRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceRequest_url_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_url_result;
    fatoora_InvoiceRequest_url_result fatoora_InvoiceRequest_url(const fatoora::capi::InvoiceRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceRequest_headers_json_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_headers_json_result;
    fatoora_InvoiceRequest_headers_json_result fatoora_InvoiceRequest_headers_json(const fatoora::capi::InvoiceRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceRequest_body_result {union {fatoora::capi::Bytes* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_body_result;
    fatoora_InvoiceRequest_body_result fatoora_InvoiceRequest_body(const fatoora::capi::InvoiceRequest* self);

    typedef struct fatoora_InvoiceRequest_decode_result {union {fatoora::capi::ValidationResponse* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_decode_result;
    fatoora_InvoiceRequest_decode_result fatoora_InvoiceRequest_decode(const fatoora::capi::InvoiceRequest* self, uint16_t status, diplomat::capi::DiplomatU8View body);

    void fatoora_InvoiceRequest_destroy(InvoiceRequest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::method() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::method_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::url() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::url_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::headers_json() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::headers_json_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::body() const {
    auto result = fatoora::capi::fatoora_InvoiceRequest_body(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Bytes>>(std::unique_ptr<fatoora::Bytes>(fatoora::Bytes::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceRequest::decode(uint16_t status, diplomat::span<const uint8_t> body) const {
    auto result = fatoora::capi::fatoora_InvoiceRequest_decode(this->AsFFI(),
        status,
        {body.data(), body.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ValidationResponse>>(std::unique_ptr<fatoora::ValidationResponse>(fatoora::ValidationResponse::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ValidationResponse>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::InvoiceRequest* fatoora::InvoiceRequest::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::InvoiceRequest*>(this);
}

inline fatoora::capi::InvoiceRequest* fatoora::InvoiceRequest::AsFFI() {
    return reinterpret_cast<fatoora::capi::InvoiceRequest*>(this);
}

inline const fatoora::InvoiceRequest* fatoora::InvoiceRequest::FromFFI(const fatoora::capi::InvoiceRequest* ptr) {
    return reinterpret_cast<const fatoora::InvoiceRequest*>(ptr);
}

inline fatoora::InvoiceRequest* fatoora::InvoiceRequest::FromFFI(fatoora::capi::InvoiceRequest* ptr) {
    return reinterpret_cast<fatoora::InvoiceRequest*>(ptr);
}

inline void fatoora::InvoiceRequest::operator delete(void* ptr) {
    fatoora::capi::fatoora_InvoiceRequest_destroy(reinterpret_cast<fatoora::capi::InvoiceRequest*>(ptr));
}


#endif // fatoora_InvoiceRequest_HPP
