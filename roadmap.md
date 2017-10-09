Type checking should be working (modulo name resolution):
* Get return type of method
* Get param types of method
* Get type of var declaration
* Get type of expression

Things we may want to support:
* Field assignment
* If-then-else and booleans

Next: name resolution
* Linking from VarAssign and VarRead to VarDecl (note that a VarDecl can include an assignment... does this need special casing in the implementation?)
* Lining from `this` to ClassDecl
