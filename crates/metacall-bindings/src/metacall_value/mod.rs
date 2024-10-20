pub mod cast;
pub mod create;
#[repr(C)]
pub enum MetacallValueID {
    MetacallBool = 0,
    MetacallChar = 1,
    MetacallShort = 2,
    MetacallInt = 3,
    MetacallLong = 4,
    MetacallFloat = 5,
    MetacallDouble = 6,
    MetacallString = 7,
    MetacallBuffer = 8,
    MetacallArray = 9,
    MetacallMap = 10,
    MetacallPtr = 11,
    MetacallFuture = 12,
    MetacallFunction = 13,
    MetacallNull = 14,
    MetacallClass = 15,
    MetacallObject = 16,
    MetacallException = 17,
    MetacallThrowable = 18,
    MetacallSize = 19,
    MetacallInvalid = 20,
}
