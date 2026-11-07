---
name: allstack-investigate
description: Diagnose a reported defect or regression from repository evidence, name the responsible layer, and produce a repair plan with regression tests, without editing code.
---

# Investigate the defect

Diagnose the supplied defect from evidence. Read applicable repository instructions, the request, the relevant code, and any retained failure output. Open recalled sources before treating them as requirements.

## Establish the facts

Read `workflows/methods/diagnose.md` first and follow its steps. Identify observed behavior, expected behavior, and the reproduction inputs. Name the exact failed condition. A model's explanation is not reproduction evidence. If required evidence is absent, name it and leave the diagnosis unproven.

Trace the value or state from input through each layer to the output. Find the producer that first creates the wrong value. For data-driven behavior, read the published data contract before proposing an engine change, and distinguish a missing contract from a runtime that violates an existing one.

Confirm which condition actually failed. A guard with several conjuncts needs the runtime value of each conjunct, not the code path alone.

## Produce the repair plan

The final response contains:

- The complete affected user flow, from trigger to observed symptom.
- Concrete source locations (`path:line`) for the producer and each consumer.
- The evidence supporting the cause, with the command or retained output that showed it.
- The smallest responsible repair, and why a larger one is not needed.
- Regression tests that fail for the observed defect and pass after the repair.
- The final checks to run before shipping.

Label each check as executed or proposed. Do not claim a check passed when it did not run.

## Boundaries

Do not edit files, create branches, commit, push, publish, send messages, or spawn agents. The durable path is the `bugfix` catalog workflow (`allstack start --workflow bugfix`), which indexes this plan and runs verification. Write the plan as prose that an incoming host can act on without this conversation.
