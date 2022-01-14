pub(crate) use wasmedge_http::make_module;

#[pymodule]
mod wasmedge_http {
    use crate::vm::{
        builtins::{PyBaseExceptionRef, PyStrRef, PyTypeRef},
        function::{IntoPyException, OptionalOption},
        types::Constructor,
        PyObjectRef, PyResult, PyValue, VirtualMachine,
    };
    use std::io;

    #[pyattr(name = "client")]
    #[pyclass(module = "client", name = "client")]
    #[derive(Debug, PyValue)]
    pub struct Client {}

    impl Constructor for Client {
        type Args = OptionalOption<PyObjectRef>;

        fn py_new(cls: PyTypeRef, _: Self::Args, vm: &VirtualMachine) -> PyResult {
            Client {}.into_pyresult_with_type(vm, cls)
        }
    }

    #[pyimpl(flags(BASETYPE), with(Constructor))]
    impl Client {
        #[pymethod]
        fn get(&self, address: PyStrRef, vm: &VirtualMachine) -> PyResult<()> {
            let address = address.to_string();
            let mut writer: Vec<u8> = Vec::new();
            let res = wasmedge_http_req::request::get(&address, &mut writer);
            match res {
                Ok(_resp) => {
                    Ok(())
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
