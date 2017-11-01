BUG: it is possible to mess the stack by declaring a variable inside one of the blocks of an if. This happens because the stack is not cleaned up after the block.

Things we may want to support:
* Field assignment

Things we may want to remove:
* Array assignment
* Array creation
