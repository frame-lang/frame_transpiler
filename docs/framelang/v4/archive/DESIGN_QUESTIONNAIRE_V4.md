# Frame v4 Design Questionnaire

## Instructions
Please answer each question with one of the following:
- **YES/NO** for binary decisions
- **Specific answer** for implementation details
- **DEFER** if the decision should be postponed
- **N/A** if not applicable to v4

Add rationale after each answer to explain the reasoning.

---

## SECTION 1: FILE EXTENSIONS AND LANGUAGE TARGETING

### Q1.1: Should Frame v4 use language-specific file extensions exclusively?
**Answer**: YES_______________
**Extensions**: 
- Python: `.fpy`
- TypeScript: `.frts`
- Rust: `.frs`
- C: `.fc`
- C++: `.fcpp`
- Java: `.fjava`
- C#: `.frcs`
- Go: `.fgo`
**Rationale**: Its informative but not authoratative as to how to compile_______________

### Q1.2: Is the @@target pragma still required with language-specific extensions?
**Answer**: [X] YES - Always required
         [ ] NO - Extension implies target
         [ ] OPTIONAL - Extension sets default, pragma overrides
**Rationale**: _______________

### Q1.3: Can one file extension compile to a different target language?
**Example**: Can a `.fpy` file have `@@target typescript`?
**Answer**: _NO - extension is informational only______________
**Rationale**: _______________

---

## SECTION 2: SYSTEM COMPOSITION AND MODULARITY

### Q2.1: Can multiple systems be defined in a single file?
**Answer**: YES_______________
**If YES, syntax**: just declare multiple systmes like you would multiple classes. only restriction is likely java_______________
**Rationale**: why not? _______________

### Q2.2: How are multiple systems in one file accessed/exported?
**Options**:
[ ] All systems automatically exported
[ ] Explicit export annotation required (`@@export`)
[X] Follow native language conventions
[ ] Primary system (first) exported, others private
**Answer**: _______________
**Rationale**: If we think of systems as just another class, then follow the native policies_______________

### Q2.3: Can Frame systems extend/inherit from other systems?
**Answer**: NO_______________
**If YES, syntax**: 
```
system Child extends Parent { }
// OR
system Child : Parent { }
// OR other: _______________
```
**Rationale**: don't know what it means_______________

### Q2.4: Can Frame systems be generic/parameterized?
**Example**: `system Queue<T> { }` or `system Queue[T] { }`
**Answer**: add as a design question for v5_______________
**If YES, syntax**: _______________
**Rationale**: _______________

---

## SECTION 3: STATE MACHINE FEATURES

### Q3.1: Are history states supported?
**Answer**:that is exactly what the state stack does $$[+]_______________
**If YES**:
- Shallow history syntax: _______________
- Deep history syntax: _______________
- Example: _______________
**Rationale**: _______________

### Q3.2: Is the change state operator `:>` supported?
**Answer**: NO_______________
**If YES, difference from `->` transition**: _______________
**Rationale**: _______________

### Q3.3: How is the initial state determined?
**Options**:
[X] First state in machine block (current v3)
[ ] Explicit annotation: `machine start=$Ready { }`
[ ] Special state name: `$Initial` or `$Start`
[ ] Constructor parameter
**Answer**: _______________
**Rationale**: _______________

### Q3.4: Can the initial state have parameters?
**Answer**: YES _______________
**If YES, how are they provided**: System params. review the grammar showing how they are mapped to param list _______________
**Rationale**: _______________

### Q3.5: Are conditional transitions (guards) supported?
**Answer**: There are no frame guards. the native lang uses its own conditionals to dtermine if any statement, including a transition, is executed._______________
**If YES, syntax**:
```
// Option A: Prefix
?condition? -> $State

// Option B: Inline
-> $State if condition

// Option C: Block
if (condition) -> $State

// Other: _______________
```
**Choice**: _______________
**Rationale**: _______________

---

## SECTION 4: FRAME-SPECIFIC LANGUAGE FEATURES

### Q4.1: Are Frame-level constants supported?
**Example**: `const MAX_RETRIES = 3` at system level
**Answer**: NO - unless there is some consideration for why native won't work_______________
**Rationale**: _______________

### Q4.2: Are Frame-level enums supported?
**Example**: `enum Status { Active, Inactive, Pending }` at system level
**Answer**: NO - unless there is some consideration for why native won't work_______________
**Rationale**: _______________

### Q4.3: Which Frame-specific control flow statements are supported?
[ ] `assert` statement
[ ] `loop` construct  
[ ] `continue` statement (continue after return)
[X] All use native equivalents
**Answer**: _______________
**Rationale**: _______________

### Q4.4: Is the `system.return` statement still supported?
**Answer**: YES_______________
**Implementation**: 
[ ] Frame keyword
[ ] Native rewrite by visitor
[ ] Not needed in v4
[X] it is syntacic sugar for the top element in the system return stack
**Rationale**: _______________

---

## SECTION 5: EVENT HANDLING AND ASYNC

### Q5.1: Can event handlers be async?
**Answer**: yes, though lets discuss if that is a formal frame keyword or a passthrough_______________
**If YES, syntax**: 
```python
# Python example
async handleData(data) {
    result = await process(data)
    -> $Done(result)
}
```
**Rationale**: _______________

### Q5.2: Can enter/exit handlers ($>, $<) be async?
**Answer**: Yes - I think they have to be right?_______________
**Rationale**: _______________

### Q5.3: How do async handlers interact with transitions?
**Options**:

I don't know exactly. whatever is simplest. lets explore options
[ ] Transition waits for async completion
[ ] Transition happens immediately
[ ] Configurable behavior
[ ] Error if transition in async handler
**Answer**: _______________
**Rationale**: _______________

### Q5.4: Is there an event queue?
**Answer**: NO _______________
**If YES**:
- Queue type: [ ] FIFO [ ] Priority [ ] Configurable
- Processing: [ ] Sync [ ] Async [ ] Configurable
**Rationale**: _______________

---

## SECTION 6: ACCESS CONTROL AND VISIBILITY

### Q6.1: Can individual methods have access modifiers?
**Example**: `private helper()` in interface block
**Answer**: NO _______________
**Rationale**: _______________

### Q6.2: Can domain variables have access modifiers?
**Example**: `public var count = 0` or `private var state`
**Answer**: NO_______________
**Rationale**: _______________

### Q6.3: Is there a 'protected' concept for inheritance?
**Answer**: there is no system inheritance_______________
**Rationale**: _______________

### Q6.4: Can operations be selectively exposed?
**Current**: All operations are public (both static and instance)
**Question**: Operations visibility model?
**Answer**: Operations ARE PUBLIC - both static (for factories/utilities) and instance (for direct system access)_______________
**Rationale**: Operations provide a way to bypass the state machine for direct system access. Static operations are useful for factories and complex functionality. Instance operations allow peek/poke into the system internals._______________

---

## SECTION 7: STATE PARAMETERS AND COMPARTMENTS

### Q7.1: How long do state parameters persist?
**Options**:
[ ] Until next transition
[ ] Until state exit
[ ] Forever (in compartment)
[ ] Configurable
**Answer**: same scope as the function they are passed to_______________
**Rationale**: _______________

### Q7.2: Are state parameters part of the compartment?
**Answer**: the state args are_______________
**Rationale**: _______________

### Q7.3: Do compartments persist across transitions to the same state?
**Example**: `$A -> $B -> $A` - is $A's compartment the same?
**Answer**: No not like that. however if the $$ is used then yes. _______________
**Rationale**: _______________

### Q7.4: What happens to compartments on state stack operations?
**Push ($$[+])**: they are pushed on a stack_______________
**Pop ($$[-])**:popped off a standard stack and returned_______________
**Rationale**: _______________

---

## SECTION 8: ERROR HANDLING AND SEMANTICS

### Q8.1: Can event handlers throw/raise exceptions?
**Answer**: No _______________
**Rationale**: don't need it yet. all execptions will be handled inside the frame functions_______________

### Q8.2: What happens if an exit handler ($<) fails?
**Options**:
[ ] Transition aborts, stay in current state
[ ] Transition continues, error logged
[ ] System enters error state
[ ] Configurable behavior
**Answer**: how would it fail? if there is a condition that would block a transition that should have been dealt with prior to the transition_______________
**Rationale**: _______________

### Q8.3: What happens if an enter handler ($>) fails?
**Options**:
[ ] Rollback to previous state
[ ] Stay in new state, error logged
[ ] System enters error state
[ ] Configurable behavior
**Answer**: no special handling. _______________
**Rationale**: _______________

### Q8.4: Are state transitions atomic?
**Answer**: currently no concept of atomicity for frame_______________
**Meaning**: _______________
**Rationale**: _______________

---

## SECTION 9: CODE GENERATION AND RUNTIME

### Q9.1: How much code does Frame generate vs developer implements?
**Options**:
[ ] Frame generates everything (current)
[ ] Frame generates structure, developer implements dispatch
[ ] Frame generates interface, developer provides implementation
[ ] Configurable generation levels
**Answer**: it will generate systems + any other specialized syntax we come up with. it is now a preprocessor. _______________
**Rationale**: _______________

### Q9.2: What is included in Frame runtime libraries?
**Check all that apply**:
[ ] FrameEvent class
[ ] FrameCompartment class  
[ ] State stack implementation
[ ] Event queue
[ ] Persistence helpers
[ ] Nothing - pure codegen
**Answer**: nothing_______________
**Rationale**: _______________

### Q9.3: Can developers override Frame runtime behavior?
**Answer**: no_______________
**If YES, mechanism**: _______________
**Rationale**: _______________

### Q9.4: Will Frame generate source maps?
**Answer**: yes _______________
**Format**: _______________
**Rationale**: _______________

---

## SECTION 10: IMPORTS AND MODULE SYSTEM

### Q10.1: How are Frame system imports distinguished from native imports?
**Current proposal**: `.fpy`, `.frts` extensions indicate Frame files
**Confirm approach**: I don't know. we now only need to locate systems in the same or other files right? how can our ast or other pipeline mechanisms do that? _______________
**Rationale**: _______________

### Q10.2: Can Frame systems be published as native packages?
**Answer**: _______________
**Process**: _______________
**Rationale**: _______________

### Q10.3: How are circular dependencies between Frame systems handled?
**Options**:
[ ] Prohibited with compile error
[ ] Allowed with lazy loading
[ ] Follow native language rules
[ ] Warning but allowed
**Answer**: _______________
**Rationale**: _______________

### Q10.4: Can Frame systems import from relative vs absolute paths?
**Answer**: we would only be importing systes, I think. broadly, shouldn't native imports/includes resolve system names?_______________
**Examples**: _______________
**Rationale**: _______________

---

## SECTION 11: PERSISTENCE AND SERIALIZATION

### Q11.1: What triggers generation of save/restore methods?
**Options**:
[X] @@persist annotation only
[ ] Automatic for all systems
[ ] @@persist + implements Persistable
[ ] Configuration option
**Answer**: _______________
**Rationale**: _______________

### Q11.2: What state is included in persistence?
**Check all that apply**:
[ ] Current state name
[ ] State parameters
[ ] Domain variables
[ ] State stack
[ ] Compartment data
[ ] Event queue
[ ] Transition history
**Answer**: all in memory compartments should be persisted. this includes current state compartment, ancestors, state stack_______________
**Rationale**: _______________

### Q11.3: What serialization format is used?
**Options**:
[ ] JSON (universal)
[ ] Language-specific (Python pickle, Java serialization)
[ ] Configurable
[ ] Developer-provided
**Answer**: you are aleady working on this. JSON is preferred I assume_______________
**Rationale**: _______________

### Q11.4: Can persistence be customized per domain variable?
**Example**: `@transient var tempData` or `@persist var important`
**Answer**: I thought you already had a strategy for this with native approaches_______________
**Rationale**: _______________

---

## SECTION 12: MIGRATION AND COMPATIBILITY

### Q12.1: How long will v3 syntax be supported?
**Options**:
[ ] 6 months deprecation
[ ] 1 year deprecation  
[ ] Indefinite compatibility mode
[X] Hard cutoff with v4 release
**Answer**: _______________
**Rationale**: _______________

### Q12.2: Can v3 and v4 code interoperate?
**Example**: v3 system importing v4 system
**Answer**: NO_______________
**Rationale**: _______________

### Q12.3: Will there be automated migration tools?
**Answer**: NO _______________
**If YES, scope**: _______________
**Rationale**: _______________

### Q12.4: How is version detected?
**Options**:
[ ] Automatic from syntax
[ ] Explicit pragma: `@@version 4`
[ ] Command-line flag
[ ] File extension
**Answer**: how are different versions of C++ or java detected?_______________
**Rationale**: _______________

---

## SECTION 13: LANGUAGE-SPECIFIC FEATURES

### Q13.1: Will all target languages have feature parity?
**Answer**: _______________
**If NO, core feature set**: _______________
**Rationale**: _______________

### Q13.2: Can language-specific features be used?
**Example**: Python decorators, TypeScript generics, Rust macros
**Answer**: _______________
**Restrictions**: _______________
**Rationale**: _______________

### Q13.3: Priority order for language support?
**Rank 1-8**:
[ X] Python
[ X] TypeScript  
[ X] Rust
[ ] C#
[ ] Java
[ ] C++
[ ] C
[ ] Go
**Rationale**: _______________

### Q13.4: Different feature sets per language?
**Example**: async only in languages that support it
**Answer**: yes we should query for _______________
**Rationale**: _______________

---

## SECTION 14: FRAME ANNOTATIONS

### Q14.1: Complete list of Frame annotations?
**Confirm these**:
- `@@target <language>` - Specifies target language
- `@@persist` - Enables persistence  
- `@@system var = System()` - System instantiation
**Add others**: lets discus using @@system also in the declaration of a system_______________
**Rationale**: _______________

### Q14.2: Can Frame annotations be parameterized?
**Example**: `@@persist(format="json", compress=true)`
**Answer**: yes_______________
**Rationale**: _______________

### Q14.3: Can custom Frame annotations be defined?
**Answer**: no_______________
**If YES, mechanism**: _______________
**Rationale**: _______________

### Q14.4: Order requirements for annotations?
**Example**: Must `@@target` come first?
**Answer**: @@target is the only prescribed one with order _______________
**Rationale**: _______________

---

## SECTION 15: TOOLING AND ECOSYSTEM

### Q15.1: Will Frame provide a Language Server Protocol (LSP) implementation?
**Answer**: _______________
**Features**: _______________
**Rationale**: _______________

### Q15.2: Will Frame have a package registry?
**Answer**: _______________
**If NO, distribution method**: _______________
**Rationale**: _______________

### Q15.3: Will Frame provide project scaffolding tools?
**Example**: `frame init my-project --template typescript`
**Answer**: _______________
**Rationale**: _______________

### Q15.4: Will Frame integrate with existing build tools?
**List confirmed integrations**:
[ ] npm/yarn (TypeScript/JavaScript)
[ ] pip/poetry (Python)
[ ] cargo (Rust)
[ ] maven/gradle (Java)
[ ] dotnet (C#)
[ ] make/cmake (C/C++)
[ ] go modules (Go)
**Rationale**: we can discuss. I don't know what is involved. _______________

---

## SECTION 16: PHILOSOPHY AND PRINCIPLES

### Q16.1: Primary goal of Frame v4?
**Rank in priority order**:
[ ] Simplicity
[ ] Native ecosystem integration
[ ] Performance
[ ] Feature completeness
[ ] Developer experience
[ ] Enterprise adoption
**Rationale**: _______________

### Q16.2: What should Frame NOT try to be?
**Check all that apply**:
[X ] General-purpose programming language
[X ] Complete type system
[? ] Runtime framework
[? ] Application framework
[X ] Universal syntax
**Rationale**: _______________

### Q16.3: Target audience priority?
**Rank 1-5**:
[ ] Individual developers
[ ] Startups
[ ] Enterprise teams
[ ] Open source projects
[ ] Academic/Research
**Rationale**: _______________

### Q16.4: Success metrics for v4?
**Define measurable goals**:
- Adoption: X_______________
- Performance: _______________
- Simplicity: _______________
- Quality: _______________
**Rationale**: _______________

---

## SECTION 17: CRITICAL DECISIONS

### Q17.1: Is Frame v4 a preprocessor or a compiler?
**Answer**: [X] Preprocessor [ ] Compiler [ ] Both
**Rationale**: _______________

### Q17.2: Should Frame have its own module system?
**Answer**: [ ] YES - Frame modules [ ] NO - Native only [ ] HYBRID
**Rationale**: discuss_______________

### Q17.3: Should Frame generate minimal or comprehensive code?
**Answer**: [ ] Minimal [ ] Comprehensive [ ] Configurable
**Rationale**: I don't know what this means_______________

### Q17.4: Should Frame files be debuggable directly?
**Answer**: Yes_______________
**If YES, mechanism**: vscode debugging_______________
**Rationale**: _______________

---

## SECTION 18: IMMEDIATE NEXT STEPS

### Q18.1: What should be implemented first?
**Order these 1-10**:
[4] Native annotation support
[1] Remove .frm extension support
[2] Simplified parser (no MIR)
[3] Persistence generation
[?] System validation
[Already is?] History states
[5] Async handlers
[Native only support?] Generic systems
[?] Migration tools
[ ] Documentation
**Rationale**: _______________

### Q18.2: What can be deferred to v5?
**List features**: _______________
**Rationale**: _______________

### Q18.3: What needs user feedback before deciding?
**List items**: _______________
**Rationale**: _______________

### Q18.4: What are the "must haves" for v4.0 release?
**List requirements**: _______________
**Rationale**: _______________

---

## NOTES SECTION
*Space for additional thoughts, concerns, or clarifications:*

________________________________________________
________________________________________________
________________________________________________
________________________________________________
________________________________________________

---

## SIGN-OFF SECTION

**Completed by**: _________________________
**Date**: _________________________________
**Version**: ______________________________
**Review status**: [ ] Draft [ ] Under Review [ ] Approved

---

## APPENDIX: Quick Decision Summary

*To be filled after questionnaire completion*

### Definitive YES Decisions:
- _______________
- _______________
- _______________

### Definitive NO Decisions:
- _______________
- _______________
- _______________

### Still Under Consideration:
- _______________
- _______________
- _______________

### Deferred to Post-v4:
- _______________
- _______________
- _______________