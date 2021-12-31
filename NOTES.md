# Goals

- Actor-based concurrency via message-passing
- Erlang inspired supervision tree
- E inspired capability-secure distributed computation

# Architecture Notes

A capability in rust maps nicely to a reference as long as the instance
referenced is not directly accessible to the outside.

This can be enforced by requiring instances to be created by an arena,
which returns a reference to the instance.

But having a reference to an instance is as good as having access to the
instance itself, so a reference to self is not a capability.

Calling a trait method is equivalent to sending a message containing the
methods arguments along with an indication of which method was called.

Holding a reference to a trait is a capability (&dyn Trait), as the underlying
instance is not exposed.

A trait can have more than one method - we can say that the capability is
to call any method on the corresponding trait.

Implementing this requires:
- serializing a reference to a trait object, which is the capability
- converting a call to a trait method on a proxy into a serializable object
- converting that serialized object back into the corresponding call on the
  actual object

Serializing a trait object requires a degree of reflection to know which
trait object is being serialized. However, this information is no longer
available at runtime.

The Reflect trait is introduced to provide the runtime information necessary
to handle serialization. Because implementing this trait incorrectly is UB,
it must be an unsafe trait.

The trait provides a reflect(&self) method to reflect the instance into a
Reflection instance which can be serialized.

# Reading Notes

Readings from http://erights.org/elib/capability/overview.html

## The Three Parts of Security [Bill Frantz]

security is: 
 - keeping objects secret: withhold access
 - protecting objects from modification: withold authority
 - preventing the misuse of objects: not possible, needs trust

best technical solution to people problems is tamper resistant audit trails

programs not loyal to their user can abuse the user's authority - principle of least authority

control where programs can send data they can access - confinement

when programs have authority their users do not - confused deputy

sending capabilities over the network requires a way to authenticate their validity
deriving new capabilities from them requires delegation

## Capability-Based Computer Systems [Henry M. Levy]

return restricted capabilities which can later be amplified to the required access

