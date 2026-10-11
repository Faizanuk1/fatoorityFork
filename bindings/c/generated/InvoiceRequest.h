#ifndef InvoiceRequest_H
#define InvoiceRequest_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Bytes.d.h"
#include "ValidationResponse.d.h"

#include "InvoiceRequest.d.h"






typedef struct fatoora_InvoiceRequest_method_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_method_result;
fatoora_InvoiceRequest_method_result fatoora_InvoiceRequest_method(const InvoiceRequest* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceRequest_url_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_url_result;
fatoora_InvoiceRequest_url_result fatoora_InvoiceRequest_url(const InvoiceRequest* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceRequest_headers_json_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_headers_json_result;
fatoora_InvoiceRequest_headers_json_result fatoora_InvoiceRequest_headers_json(const InvoiceRequest* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceRequest_body_result {union {Bytes* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_body_result;
fatoora_InvoiceRequest_body_result fatoora_InvoiceRequest_body(const InvoiceRequest* self);

typedef struct fatoora_InvoiceRequest_decode_result {union {ValidationResponse* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceRequest_decode_result;
fatoora_InvoiceRequest_decode_result fatoora_InvoiceRequest_decode(const InvoiceRequest* self, uint16_t status, DiplomatU8View body);

void fatoora_InvoiceRequest_destroy(InvoiceRequest* self);





#endif // InvoiceRequest_H
