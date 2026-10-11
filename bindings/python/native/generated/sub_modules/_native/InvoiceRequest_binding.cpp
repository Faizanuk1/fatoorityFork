#include "diplomat_nanobind_common.hpp"


#include "InvoiceRequest.hpp"

namespace _native {
void add_InvoiceRequest_binding(nb::module_ mod) {
    PyType_Slot _native_InvoiceRequest_slots[] = {
        {Py_tp_free, (void *)_native::InvoiceRequest::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::InvoiceRequest> opaque(mod, "InvoiceRequest", nb::type_slots(_native_InvoiceRequest_slots), "Owned request and decoding context; contains credentials.");
    opaque
        .def("body", std::move(maybe_op_unwrap(&_native::InvoiceRequest::body)))
        .def("decode", std::move(maybe_op_unwrap(&_native::InvoiceRequest::decode)), "status"_a, "body"_a)
        .def("headers_json", std::move(maybe_op_unwrap(&_native::InvoiceRequest::headers_json)))
        .def("method", std::move(maybe_op_unwrap(&_native::InvoiceRequest::method)))
        .def("url", std::move(maybe_op_unwrap(&_native::InvoiceRequest::url)));
}

} 