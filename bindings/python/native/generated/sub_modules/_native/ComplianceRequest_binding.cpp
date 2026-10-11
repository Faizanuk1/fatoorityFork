#include "diplomat_nanobind_common.hpp"


#include "ComplianceRequest.hpp"

namespace _native {
void add_ComplianceRequest_binding(nb::module_ mod) {
    PyType_Slot _native_ComplianceRequest_slots[] = {
        {Py_tp_free, (void *)_native::ComplianceRequest::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ComplianceRequest> opaque(mod, "ComplianceRequest", nb::type_slots(_native_ComplianceRequest_slots), "Owned request and decoding context; contains credentials.");
    opaque
        .def("body", std::move(maybe_op_unwrap(&_native::ComplianceRequest::body)))
        .def("decode", std::move(maybe_op_unwrap(&_native::ComplianceRequest::decode)), "status"_a, "body"_a)
        .def("headers_json", std::move(maybe_op_unwrap(&_native::ComplianceRequest::headers_json)))
        .def("method", std::move(maybe_op_unwrap(&_native::ComplianceRequest::method)))
        .def("url", std::move(maybe_op_unwrap(&_native::ComplianceRequest::url)));
}

} 