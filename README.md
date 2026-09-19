# Crusty

An Os made by a little student.
Designed to target x86-64 processor.
Uses an 'everything is an ecs entity' paradigm.

Based upon "Kernel - RustOs" tutorial, and many other things.

# Roadmap
- [X] Boot
- [X] Debug in terminal
- [ ] Handle memory and interrupts
- [ ] Make all the component, entity, and systems lib
- [ ] Make a CLI
- [ ] Make an ELF interpreter/compiler
- [ ] Make a GUI
- [ ] Run Doom
- [ ] Run Bad Apple
- [ ] Get RickRolled

# TODO
- Fix the readme to be cleaner

# Design
well, let me tell you a bit more about the design I want to follow to build this Os.

Instead of, like much of the other kernels,
consider everything as a file and make the  kernel responsible for
the bridge between processes and ressources at any time one requires it (syscall),
I would like to have all the processes declare which ressources they are
going to use, and make them handle it as raw data.


This seems like a really bad idea, if a process had to reserve every ressources
it might use beforehand, this would lock them to other processes,
thus preventing concurential execution, and even context switching.

Well, I build upon this problem in two ways:
- Accept that a process won't be paused, and instead would be executed in one pass
- Make processes much shorter, and requesting way less ressources

I call those processes "systems" (and process would soon reference something else).

What exactly is a system, well let's make a clear definition:
A system is a pure function, which take as argument an Event and some ressources.
It's called when its event are emitted,
perform an operation upon its ressources,
can call into other event,
and then is terminated.

This might make everything clearer
(with some questions too, what are those events?),
and you should know understand how systems are the way to go
if we want to follow what we previously defined as our conduct line.
Those small functions, called only when needed, and asking for few ressources
are perfect to ensure not overloading our kernel. 
This allows us to allocate each systems it's ressources on the fly.
Also, each system having to ask beforehand for its ressources, and not being
executed as long as it does not have all of them ready prevent deadlocks.
Also, if we make the kernel intelligent enough, we can avoid data races by
preventing systems requiring the same ressource.
The pure function part in the definition refer to the fact that this function
shouldn't be able to access any ressources from anywhere else than its
parameter. This make it higly testable, no bug can come from somewhere else,
and you can then narrow down the source of the problem.


About events:
Events are structure which contains data, along with a tag to identify it.

They are used by systems to communicate one with each other, and by the
kernel to pass some hardware interrupts to the systems.


About processes:
I previously said that a process would soon be a name for something new.
From now on, when I would speak of a process, I would refer to a bunch of
systems designed to run all together. This is mostly how a specific task 
would be handled.

Real world example:
Let's say you are a programmer (which, if you're here, is most probably the case)
and you create your application, a simple calculator.
What you would do is create a systems that would take as parameter an
expression as an event, and capture the terminal writing right into this event.
Then you would write your response in the terminal.
And then return. Easy peasy.
Now, what if you want to make it have a gui.
Then you create a bunch of other systems that would each frame draw on the
screen, and listen for user input, communicating with each others with events.


About ressources:
A ressource is the result of a query to the kernel.
When asking for a ressource, you specify through a request everything
that this ressource should contains (components), and the kernel will
look for all entities that match this request, and pass them as paramters.





































