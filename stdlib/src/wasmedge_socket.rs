pub(crate) use wasmedge_socket::make_module;

#[pymodule]
mod wasmedge_socket {
    use crate::vm::{
        builtins::{PyBaseExceptionRef, PyStrRef, PyTypeRef},
        function::{ArgBytesLike, IntoPyException, OptionalOption},
        types::Constructor,
        PyObjectRef, PyResult, PyValue, VirtualMachine,
    };
    use crossbeam_utils::atomic::AtomicCell;
    use std::io::{self, Read, Write};
    use wasmedge_wasi_socket;

    mod nullable {
        use super::*;
        #[derive(Debug, Clone, Copy)]
        #[repr(transparent)]
        pub struct NullableTcpStream(Option<wasmedge_wasi_socket::TcpStream>);
        impl NullableTcpStream {
            pub fn from_stream(stream: wasmedge_wasi_socket::TcpStream) -> Self {
                NullableTcpStream(Some(stream))
            }
            pub fn invalid() -> Self {
                Self(None)
            }
            pub fn get(&self) -> Option<&wasmedge_wasi_socket::TcpStream> {
                self.0.as_ref()
            }
        }

        #[derive(Debug, Clone, Copy)]
        #[repr(transparent)]
        pub struct NullableTcpListener(Option<wasmedge_wasi_socket::TcpListener>);
        impl NullableTcpListener {
            pub fn from_listener(lisener: wasmedge_wasi_socket::TcpListener) -> Self {
                NullableTcpListener(Some(lisener))
            }
            pub fn invalid() -> Self {
                Self(None)
            }
            pub fn get(&self) -> Option<&wasmedge_wasi_socket::TcpListener> {
                self.0.as_ref()
            }
        }
    }

    #[pyattr(name = "tcpstream")]
    #[pyclass(module = "tcpstream", name = "tcpstream")]
    #[derive(Debug, PyValue)]
    pub struct TcpStream {
        inner: AtomicCell<nullable::NullableTcpStream>,
    }

    #[pyattr(name = "tcplistener")]
    #[pyclass(module = "tcplistener", name = "tcplistener")]
    #[derive(Debug, PyValue)]
    pub struct TcpListener {
        inner: AtomicCell<nullable::NullableTcpListener>,
    }

    impl Constructor for TcpStream {
        type Args = OptionalOption<PyObjectRef>;

        fn py_new(cls: PyTypeRef, _: Self::Args, vm: &VirtualMachine) -> PyResult {
            TcpStream {
                inner: AtomicCell::new(nullable::NullableTcpStream::invalid()),
            }
            .into_pyresult_with_type(vm, cls)
        }
    }

    impl Constructor for TcpListener {
        type Args = OptionalOption<PyObjectRef>;

        fn py_new(cls: PyTypeRef, _: Self::Args, vm: &VirtualMachine) -> PyResult {
            TcpListener {
                inner: AtomicCell::new(nullable::NullableTcpListener::invalid()),
            }
            .into_pyresult_with_type(vm, cls)
        }
    }

    impl TcpStream {
        fn connect_inner(&self, address: PyStrRef) -> Result<(), IoOrPyException> {
            let addt = address.to_string();
            let tcpstream = wasmedge_wasi_socket::TcpStream::connect(addt.as_str())?;
            let nullable = nullable::NullableTcpStream::from_stream(tcpstream);
            self.inner.store(nullable);
            Ok(())
        }
    }

    impl TcpListener {
        fn bind_inner(&self, address: PyStrRef) -> Result<(), IoOrPyException> {
            let addt = address.to_string();
            let tcplistener = wasmedge_wasi_socket::TcpListener::bind(addt.as_str())?;
            let nullable = nullable::NullableTcpListener::from_listener(tcplistener);
            self.inner.store(nullable);
            Ok(())
        }
    }

    #[pyimpl(flags(BASETYPE), with(Constructor))]
    impl TcpStream {
        #[pymethod]
        fn connect(&self, address: PyStrRef, vm: &VirtualMachine) -> PyResult<()> {
            self.connect_inner(address)
                .map_err(|e| e.into_pyexception(vm))
        }

        #[pymethod]
        fn send(&self, bytes: ArgBytesLike, vm: &VirtualMachine) -> PyResult<usize> {
            let buf = bytes.borrow_buf();
            let buf = &*buf;
            let nullable = self.inner.load();
            if let Some(tcpstream) = nullable.get() {
                let mut tcpstream = *tcpstream;
                let res = tcpstream.write(buf);
                match res {
                    Ok(n) => Ok(n),
                    Err(e) => Err(e.into_pyexception(vm)),
                }
            } else {
                Err(IoOrPyException::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "TcpStream is closed or not connected",
                ))
                .into_pyexception(vm))
            }
        }

        #[pymethod]
        fn recv(&self, bufsize: usize, vm: &VirtualMachine) -> PyResult<Vec<u8>> {
            let nullable = self.inner.load();
            if let Some(tcpstream) = nullable.get() {
                let mut tcpstream = *tcpstream;
                let mut buf = vec![0; bufsize];
                let res = tcpstream.read(&mut buf);
                match res {
                    Ok(n) => Ok(buf[..n].to_vec()),
                    Err(e) => Err(e.into_pyexception(vm)),
                }
            } else {
                Err(IoOrPyException::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "TcpStream is closed or not connected",
                ))
                .into_pyexception(vm))
            }
        }

        #[pymethod]
        fn close(&self, vm: &VirtualMachine) -> PyResult<()> {
            let nullable = self.inner.load();
            if let Some(tcpstream) = nullable.get() {
                let tcpstream = *tcpstream;
                tcpstream
                    .shutdown(wasmedge_wasi_socket::Shutdown::Both)
                    .map_err(|e| e.into_pyexception(vm))
            } else {
                Err(IoOrPyException::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "TcpStream is closed or not connected",
                ))
                .into_pyexception(vm))
            }
        }

        #[pymethod]
        fn close_read(&self, vm: &VirtualMachine) -> PyResult<()> {
            let nullable = self.inner.load();
            if let Some(tcpstream) = nullable.get() {
                let tcpstream = *tcpstream;
                tcpstream
                    .shutdown(wasmedge_wasi_socket::Shutdown::Read)
                    .map_err(|e| e.into_pyexception(vm))
            } else {
                Err(IoOrPyException::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "TcpStream is closed or not connected",
                ))
                .into_pyexception(vm))
            }
        }

        #[pymethod]
        fn close_write(&self, vm: &VirtualMachine) -> PyResult<()> {
            let nullable = self.inner.load();
            if let Some(tcpstream) = nullable.get() {
                let tcpstream = *tcpstream;
                tcpstream
                    .shutdown(wasmedge_wasi_socket::Shutdown::Write)
                    .map_err(|e| e.into_pyexception(vm))
            } else {
                Err(IoOrPyException::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "TcpStream is closed or not connected",
                ))
                .into_pyexception(vm))
            }
        }
    }

    #[pyimpl(flags(BASETYPE), with(Constructor))]
    impl TcpListener {
        #[pymethod]
        fn bind(&self, address: PyStrRef, vm: &VirtualMachine) -> PyResult<()> {
            self.bind_inner(address).map_err(|e| e.into_pyexception(vm))
        }

        #[pymethod]
        fn accept(&self, vm: &VirtualMachine) -> PyResult<(TcpStream, String)> {
            let nullable = self.inner.load();
            if let Some(tcplistener) = nullable.get() {
                let tcplistener = *tcplistener;
                let (tcpstream, addr) =
                    match tcplistener.accept().map_err(|e| e.into_pyexception(vm)) {
                        Ok(res) => res,
                        Err(e) => return Err(e),
                    };
                let nullable = nullable::NullableTcpStream::from_stream(tcpstream);
                let addr: String = addr.to_string();
                Ok((
                    TcpStream {
                        inner: AtomicCell::new(nullable),
                    },
                    addr,
                ))
            } else {
                Err(IoOrPyException::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "TcpListener is closed or not bound",
                ))
                .into_pyexception(vm))
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
