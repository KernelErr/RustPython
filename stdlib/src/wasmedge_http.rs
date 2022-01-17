pub(crate) use wasmedge_http::make_module;

#[pymodule]
mod wasmedge_http {
    use crate::vm::{
        builtins::{PyBaseExceptionRef, PyStrRef, PyTypeRef, PyDictRef},
        function::{IntoPyException, OptionalOption},
        types::Constructor,
        PyObjectRef, PyResult, PyValue, VirtualMachine,
    };
    use std::io;
    use wasmedge_http_req::response::Headers;

    #[pyattr(name = "client")]
    #[pyclass(module = "client", name = "client")]
    #[derive(Debug, PyValue)]
    pub struct Client {}

    #[pyattr(name = "response")]
    #[pyclass(module = "response", name = "response")]
    #[derive(Debug, PyValue)]
    pub struct Response {
        status_code: u16,
        header: Headers,
        body: Vec<u8>,
    }

    impl Constructor for Client {
        type Args = OptionalOption<PyObjectRef>;

        fn py_new(cls: PyTypeRef, _: Self::Args, vm: &VirtualMachine) -> PyResult {
            Client {}.into_pyresult_with_type(vm, cls)
        }
    }

    impl Constructor for Response {
        type Args = OptionalOption<PyObjectRef>;

        fn py_new(cls: PyTypeRef, _: Self::Args, vm: &VirtualMachine) -> PyResult {
            Response {
                status_code: 200,
                header: Headers::new(),
                body: vec![],
            }.into_pyresult_with_type(vm, cls)
        }
    }

    #[pyimpl(flags(BASETYPE), with(Constructor))]
    impl Client {
        #[pymethod]
        fn get(&self, address: PyStrRef, vm: &VirtualMachine) -> PyResult<Response> {
            let address = address.to_string();
            let mut writer: Vec<u8> = Vec::new();
            let res = wasmedge_http_req::request::get(&address, &mut writer);
            match res {
                Ok(resp) => {
                    Ok(Response {
                        status_code: resp.status_code().into(),
                        header: resp.headers().clone(),
                        body: writer,
                    })
                },
                Err(e) => Err(
                    IoOrPyException::Io(io::Error::new(
                        io::ErrorKind::Other,
                        e.to_string(),
                    )).into_pyexception(vm)
                ),
            }
        }
    }

    #[pyimpl(flags(BASETYPE), with(Constructor))]
    impl Response {
        #[pymethod]
        fn status_code(&self) -> u16 {
            self.status_code
        }

        #[pymethod]
        fn headers(&self, vm: &VirtualMachine) -> PyDictRef {
            let dict = vm.ctx.new_dict();
            for (key, value) in self.header.iter() {
                dict.set_item(vm.ctx.new_str(key.as_str()), vm.ctx.new_str(value.as_str()).into(), vm)
                    .unwrap();
            }
            dict
        }

        #[pymethod]
        fn body(&self) -> Vec<u8> {
            self.body.clone()
        }
    }

    enum IoOrPyException {
        Py(PyBaseExceptionRef),
        Io(io::Error),
    }

    impl From<PyBaseExceptionRef> for IoOrPyException {
        fn from(exc: PyBaseExceptionRef) -> Self {
            Self::Py(exc)
        }
    }

    impl From<io::Error> for IoOrPyException {
        fn from(err: io::Error) -> Self {
            Self::Io(err)
        }
    }

    impl IntoPyException for IoOrPyException {
        fn into_pyexception(self, vm: &VirtualMachine) -> PyBaseExceptionRef {
            match self {
                Self::Io(err) => err.into_pyexception(vm),
                _ => unimplemented!(),
            }
        }
    }
}
