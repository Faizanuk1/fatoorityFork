#ifndef _NATIVE_InvoiceRequest_HPP
#define _NATIVE_InvoiceRequest_HPP

#include "InvoiceRequest.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "Bytes.hpp"
#include "ValidationResponse.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_InvoiceRequest_method_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_method_result;
    fatoora_InvoiceRequest_method_result fatoora_InvoiceRequest_method(const _native::capi::InvoiceRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceRequest_url_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_url_result;
    fatoora_InvoiceRequest_url_result fatoora_InvoiceRequest_url(const _native::capi::InvoiceRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceRequest_headers_json_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_headers_json_result;
    fatoora_InvoiceRequest_headers_json_result fatoora_InvoiceRequest_headers_json(const _native::capi::InvoiceRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceRequest_body_result {union {_native::capi::Bytes* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_body_result;
    fatoora_InvoiceRequest_body_result fatoora_InvoiceRequest_body(const _native::capi::InvoiceRequest* self);

    typedef struct fatoora_InvoiceRequest_decode_result {union {_native::capi::ValidationResponse* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_decode_result;
    fatoora_InvoiceRequest_decode_result fatoora_InvoiceRequest_decode(const _native::capi::InvoiceRequest* self, uint16_t status, _native::diplomat::capi::DiplomatU8View body);

    void fatoora_InvoiceRequest_destroy(InvoiceRequest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::method() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::method_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::url() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::url_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::headers_json() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::headers_json_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::body() const {
    auto result = _native::capi::fatoora_InvoiceRequest_body(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Bytes>>(std::unique_ptr<_native::Bytes>(_native::Bytes::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>> _native::InvoiceRequest::decode(uint16_t status, _native::diplomat::span<const uint8_t> body) const {
    auto result = _native::capi::fatoora_InvoiceRequest_decode(this->AsFFI(),
        status,
        {body.data(), body.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ValidationResponse>>(std::unique_ptr<_native::ValidationResponse>(_native::ValidationResponse::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ValidationResponse>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::InvoiceRequest* _native::InvoiceRequest::AsFFI() const {
    return reinterpret_cast<const _native::capi::InvoiceRequest*>(this);
}

inline _native::capi::InvoiceRequest* _native::InvoiceRequest::AsFFI() {
    return reinterpret_cast<_native::capi::InvoiceRequest*>(this);
}

inline const _native::InvoiceRequest* _native::InvoiceRequest::FromFFI(const _native::capi::InvoiceRequest* ptr) {
    return reinterpret_cast<const _native::InvoiceRequest*>(ptr);
}

inline _native::InvoiceRequest* _native::InvoiceRequest::FromFFI(_native::capi::InvoiceRequest* ptr) {
    return reinterpret_cast<_native::InvoiceRequest*>(ptr);
}

inline void _native::InvoiceRequest::operator delete(void* ptr) {
    _native::capi::fatoora_InvoiceRequest_destroy(reinterpret_cast<_native::capi::InvoiceRequest*>(ptr));
}


#endif // _NATIVE_InvoiceRequest_HPP
