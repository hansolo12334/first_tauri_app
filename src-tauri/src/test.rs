// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;
// use pyo3::ffi::c_str;

// #[test]
// fn test(){
//     Python::with_gil(|py|{
//       let sys=py.import("sys");
//       let version: String=sys.getattr("version")?.extract()?;
      
//       let locals=[("os",py.import("os")?)].into_py_dict(py)?;
//       let code=c_str!("os.getenv('USER') or os.getebv('USERNAME') or 'Unknown'");
//       let user: String=py.eval(code,None,Some(&locals))?.extract()?;

//       println(" python 版本{}",version);
//       Ok(());
//     })
// }