#[cfg(feature = "trace")]
#[macro_export]
macro_rules! log_trace {
  ($($input:tt)*) => {
    print!("TRACE File {},", file!());
    print!(" Line {}: ", line!());
    println!($($input)*);
  };
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! log_trace {
  ($($input:tt)*) => {};
}

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! log_debug {
  ($($input:tt)*) => {
    print!("DEBUG File {},", file!());
    print!(" Line {}: ", line!());
    println!($($input)*);
  };
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! log_debug {
  ($($input:tt)*) => {};
}

#[cfg(feature = "info")]
#[macro_export]
macro_rules! log_info {
  ($($input:tt)*) => {
    print!("INFO File {},", file!());
    print!(" Line {}: ", line!());
    println!($($input)*);
  };
}

#[cfg(not(feature = "info"))]
#[macro_export]
macro_rules! log_info {
  ($($input:tt)*) => {};
}

#[cfg(feature = "warning")]
#[macro_export]
macro_rules! log_warn {
  ($($input:tt)*) => {
    print!("WARN File {},", file!());
    print!(" Line {}: ", line!());
    println!($($input)*);
  };
}

#[cfg(not(feature = "warning"))]
#[macro_export]
macro_rules! log_warn {
  ($($input:tt)*) => {};
}

#[cfg(feature = "error")]
#[macro_export]
macro_rules! log_error {
  ($($input:tt)*) => {
    print!("ERROR File {}", file!());
    print!(" Line {}: ", line!());
    println!($($input)*);
  };
}