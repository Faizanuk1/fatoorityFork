#include "diplomat_nanobind_common.hpp"


#include "ProductionRequest.hpp"

namespace _native {
void add_ProductionRequest_binding(nb::module_ mod) {
    PyType_Slot _native_ProductionRequest_slots[] = {
        {Py_tp_free, (void *)_native::ProductionRequest::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ProductionRequest> opaque(mod, "ProductionRequest", nb::type_slots(_native_ProductionRequest_slots), "Owned request and decoding context; contains credentials.");
    opaque
        .def("body", std::move(maybe_op_unwrap(&_native::ProductionRequest::body)))
        .def("decode", std::move(maybe_op_unwrap(&_native::ProductionRequest::decode)), "status"_a, "body"_a)
        .def("headers_json", std::move(maybe_op_unwrap(&_native::ProductionRequest::headers_json)))
        .def("method", std::move(maybe_op_unwrap(&_native::ProductionRequest::method)))
        .def("url", std::move(maybe_op_unwrap(&_native::ProductionRequest::url)));
}

} 