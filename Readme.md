# swisseph_rs
 
##WIP: This project is under construction and interfaces should not be considered to be final!

Unofficial `swisseph_rs` implements higher level rust wrappers and types around the swisseph c library.
The goal would be to provide multiple different levels of abstraction based on the need of
the caller. 


## TODO:
* Many functions still need to be implemented. 
* Slight discrepancy exists in example USA mundane calculation. Off by some minutes
* Provide builder that does some sanity checks with flags
* Provide a way to support both pointers to arrays for efficient, reuse of data structures 
but also allow for creation of structures inside of function calls that return new structures.

## Abstraction Levels

TODO: Solidify exactly what each layer will do

As a rough sketch of the different layers would be as follows:
* raw -> c types straight from bindgen. See `libswisseph-sys_rs`
* level 1 -> rust types calling functions as you would in c
* level 2 -> rust types calling functions as you would in rust
* level 3 -> rust types with rust ergonomics using Result and things with verbose codes
* level 4 -> rust ergonomics with Result in the cleanest possible rust code
* level 5 -> high level interface
	
	
### Raw (`libswisseph-sys_rs`)
As an explanation, the raw bindgen functions require unsafe blocks, and raw mutable pointers
to be passed in to functions. All parameters that are passed in are expected to be fully
initialized and will modify data outside of the function. This allows for maximum perfomance
possible by allowing the user to reuse initialized arrays in anyway they desire and does
not create new arrays for each call being made. This however requires the user to know the
exact data types (arrays of varying sizes and types) as well as knowing what each index of
the returned data is meant to be. Many arrays have multiple floats returned with nothing
more than an index to identify them as opposed to a struct or a hash with string identifier.

The raw wrapper also may use awkward data types that might not be intuitive to appealing to
the typical rust developer.

### Level 1 (Improved C)
This level's goal would be to just clean up any c-inhereted awkwardness, but still require
end users to know the proper way to initialize data and rely on pointers that functions 
will modify data outside the scope of the function itself.

### Level 5 (Fully featured w/ maximum rust ergonomics)
