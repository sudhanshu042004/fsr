# Architecture
For now, this is only designed for the transfer of files between two VMs or entities.

- Calculate number of blocks file use.
- Locate the position of these blocks.
- according to the number of blocks calculate the number of threads needed to create.
- created the array of object which include the info of blocks thier location on the drive.
- before start the executioner, it will create the manifest of the task, which includes the number of blocks will sended thier id, block they are sending, their positioning and all and send it over to the reciever.
- Each thread now, can pickup the blocks from the queue and send the block to reciever.
- Blocks can recieved in the unsorted manner, for that using the manifest reciever will postion the block on the calculated offset, use the similar architecture to send the data.
- need to divide the work on three layer 1. storage layer, 2. network/transfer layer 3. Execution layer
