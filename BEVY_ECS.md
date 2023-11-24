### The ECS of Bevy is not complicated, just different.

The ECS was quite a challenge to my brain! It's not application code calling system APIs. It's rather the opposite, a bit like a user interface code: There are things having different properties. And on events, parts of the application code are called, at start time, at user interactions or cyclically.

### Object Orientated?

I am a fan of object orientated programming. But comparing the ECS to OO did rise some resistance. Mostly because that inheritance, which I don't like and don't use anyway. 

Rust doesn’t even use the term "class", but next to procedural programming it supports OO, partly because structur-types may have (pointers to) functions. No inheritance but Traids. The Bevy description of ECS doesn’t show any OO code. But in my mind there still is this imagination of OO: An entity, clearly is an object. And a component is an attribute or property. ECS did “move" the class concept from the compiler to the runtime and makes it highly dynamic; anything may change at runtime! A certain combination of components is about like a class. Components also can be flags to get a sub-group/class of entities.

### System?

The term system was a real problem to me. Naming something just “system” is bad. It could mean anything and has to have a pre-/postfix like operating-system. Calling it ECS-system doesn’t help much, it still does not tell what that thing is doing: Usually a system loops over a “class” of entities and does some functionality with each of them. The code inside the loop looks like a method of a class to me. And a component together with a system code looks like a Rust trait to me.

In a class, a method can use all attributes, a system can only use the queried components. A method needs to be called and can be called at any time. What about the ECS systems? Well, an ECS usually is used for cyclical running applications like visualisations, games or simulations, may be even vor industrial process controls. The application code is spread into all the systems. Systems often run cyclically, may be with additional conditions. But a systems also may run once, wich reminds me to a class constructor. So a system is a part of the application code for certain components. A System is called by the ECS; a call into the application. Could we name it "Callback System"? Or "Component Processing System" ?

Systems are not like methods - yet. Because of the loop. But what if an ECS also would support “methods”? Just like systems, but with the loop done by the ECS? The same query syntax but returning a single component, not an array. Or even more simple, the components may be parameters. Would it be the same execution load this way? May be even better parallelisation?
Instead of add_systems it would be add_methods? Not a good naming, is it? It’s not a components-processing-system any more, rather a components-processes. “add_process” sounds nice to me. Bevy even did have it once, but it was to difficult to maintain.

### Programming Language?

Knowing Rust not that well, the interface of a system is a bit mystic. The system function may have any combination of parameters and the ECS still is able to call it. Just for curiosity I wonder, if that ECS concept could be added into a programming language: entity, component and system become reserved and used words instead of traids. But the ECS must be a runtime anyway. The entities have to be managed dynamically. And efficiently! To cyclically find/query "objects we care" seems quite slow. There are managed lists in the ECS.

### IDE?

There is no IDE for the Bevy ECS, yet, they certainly think about it. There would be a table of components with a structure type each. Also a table of "entity types" with their collection of components. And the systems of course, again with their queries for components and other conditions. At last the systems code.

All that could be done with a UI and textual presentations. Rust needs a symbol name anyway. But many Rust words could be presented by symbols to. The basic types in the structures of course. Next to names, the user could select or draw symbols. And the code logic could be done by graphically concept like Lego Mindstorms or IEC 61131-3 block diagrams.

-karlos-
