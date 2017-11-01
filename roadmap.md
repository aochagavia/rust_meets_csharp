Next:
* Write test programs and query information about them.

Things to measure:
* Nodes queried

Test cases:
* Giant function, get type of expression
* Giant function, go to var definition
* Hundreds of classes, get available methods for type

Maybe too complicated?
* List all variables visible to a given node (interesting to provide auto complete for variables): class variables + parameters + vars declared before in the method

BUG: it is possible to mess the stack by declaring a variable inside one of the blocks of an if. This happens because the stack is not cleaned up after the block.

Things we may want to support:
* Field assignment

Things we may want to remove:
* Array assignment
* Array creation
