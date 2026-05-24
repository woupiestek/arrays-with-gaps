# **LLM‑Aware Code Style Guide**

A style guide for writing code that is clear, maintainable, and optimized for
collaboration with Large Language Models (LLMs).

---

## **1. General Principles**

- **Clarity over cleverness**\
  Prefer readable, predictable code over compact or “smart” constructs.

- **Consistency over originality**\
  LLMs perform best when code follows common idioms and patterns found in
  open‑source ecosystems.

- **Intent over implementation**\
  Comments and structure should communicate _why_ code exists, not restate
  _what_ it does.

- **Predictability over abstraction**\
  Avoid unnecessary abstractions that obscure control flow or intent.

---

## **2. Formatting**

- Always use an **automatic formatter** (e.g., Black, Prettier, gofmt, rustfmt).
- Keep formatting **idiomatic for the language**.
- Avoid unusual or exotic formatting styles.
- Use whitespace to separate logical blocks, but avoid excessive blank lines.
- Keep line length reasonable (typically 80–120 characters).

**Rationale:**\
LLMs rely on statistical patterns. Clean, idiomatic formatting improves
reasoning and reduces token noise.

---

## **3. Naming Conventions**

### **3.1 Variable and Function Names**

- Use **medium‑length, descriptive names**:
  - Good: `user_count`, `fetch_data`, `is_valid`
  - Bad: `x`, `tmp`, `numberOfActiveUsersCurrentlyLoggedIntoTheSystem`

- Use **idiomatic casing** for the language (snake_case, camelCase, PascalCase).

- Include **semantic hints**:
  - Booleans: `is_`, `has_`, `should_`
  - Iterables: plural nouns (`users`, `items`)
  - Counters: `count`, `total`, `index`

- Avoid abbreviations unless universally understood (`id`, `url`, `db`).

**Rationale:**\
LLMs infer type and intent from names. Informative names improve reasoning and
reduce hallucinations.

---

## **4. Comments**

### **4.1 Comment Philosophy**

- Comments should explain **intent**, **constraints**, and **edge cases**.
- Do **not** repeat what the code already says.
- Avoid outdated or misleading comments at all costs.

### **4.2 Good Comment Examples**

```python
# Normalize input by trimming whitespace and collapsing multiple spaces.
# Required because downstream NLP models assume single-space separation.
def clean_input(text):
    ...
```

```python
i += 1  # skip header row
```

### **4.3 Bad Comment Examples**

```python
x = x + 1  # increment x by 1
```

```python
# This function processes data
def process(data):
    ...
```

**Rationale:**\
LLMs benefit from concise, semantically rich comments that provide context not
present in the code.

---

## **5. Code Structure**

### **5.1 Functions**

- Prefer **small, focused functions** with a single responsibility.
- Avoid micro‑functions that fragment logic unnecessarily.
- Use clear input/output contracts.

### **5.2 Modules and Files**

- Group related functionality together.
- Keep files reasonably sized; avoid “god files”.
- Use descriptive module names.

### **5.3 Control Flow**

- Prefer straightforward control flow over clever tricks.
- Avoid deeply nested logic; refactor into smaller units when needed.

**Rationale:**\
LLMs reason better with predictable, modular structures.

---

## **6. Documentation**

- Provide **high-level documentation** for modules, classes, and complex
  workflows.
- Include:
  - purpose
  - assumptions
  - invariants
  - known limitations
  - examples

- Avoid overly verbose “literate programming” unless the project explicitly
  requires it.

**Rationale:**\
LLMs use documentation as semantic scaffolding. High-level docs improve
understanding without overwhelming the model.

---

## **7. Testing**

- Maintain a robust test suite:
  - unit tests
  - integration tests
  - property-based tests (where appropriate)

- Write tests that express **expected behavior**, not implementation details.

- Use clear test names and descriptive assertions.

**Rationale:**\
Tests provide ground truth for LLM-assisted refactoring and debugging.

---

## **8. DRY, WET, and Abstractions**

- Avoid extreme DRY (“Don’t Repeat Yourself”) that leads to unnecessary
  abstraction.
- Prefer **clarity-first** duplication when it improves readability.
- Abstract only when:
  - duplication is harmful
  - the abstraction is stable
  - the abstraction improves clarity

**Rationale:**\
LLMs handle duplication well but struggle with over-abstracted code.

---

## **9. Error Handling**

- Use idiomatic error handling for the language.
- Provide clear error messages with actionable information.
- Avoid silent failures or overly generic exceptions.

**Rationale:**\
LLMs rely on error messages to reason about failure modes.

---

## **10. LLM-Specific Guidelines**

- Keep code **predictable** and **idiomatic** to maximize LLM reasoning quality.
- Provide **contextual comments** where intent is not obvious.
- Avoid:
  - obfuscated code
  - unconventional patterns
  - excessive cleverness
  - inconsistent naming
  - outdated comments

- When using LLMs for refactoring or generation:
  - supply relevant context
  - include function docstrings
  - include edge cases
  - include expected behavior

---

# **Summary**

This style guide optimizes for:

- human readability
- maintainability
- LLM-assisted development
- predictable reasoning
- reduced ambiguity

The core principle is simple:\
**Write code that is easy for humans and models to understand, reason about, and
extend.**
