---

# 📄 **PR Cleanup Prompt (LLM‑Aware Code Style Guide)**

This prompt is intended for use **before submitting a pull request**.  
It enforces a strict cleanup cycle based on the *LLM‑Aware Code Style Guide* and performs a review **against the diff with the project’s trunk branch** (which may be named `main`, `master`, or `develop`).

---

## 🚦 **When to Use This Prompt**
Use this prompt when:

- functionality is complete  
- the code is ready for cleanup  
- you want a consistent, high‑quality PR  
- you want the LLM to review only the **changes**, not the entire codebase  

---

# 🧼 **PR Cleanup Prompt**

**Context:**  
We are in the pre‑PR phase. The functionality is complete.  
Your task is to prepare the code for a clean, high‑quality pull request.

**Important:**  
The project’s trunk branch may be named `main`, `master`, or `develop`.  
Review the diff **against the trunk branch of this repository**, whichever name it uses.

---

## **Your Tasks**

### **1. Diff‑Aware Review**
Perform a strict review of **only the changes** in the diff between the feature branch and the trunk branch.

For each issue you find, categorize it under:

- Naming  
- Structure  
- Comments  
- Intent communication  
- Control flow  
- Formatting  
- Abstractions  
- Error handling  
- Other inconsistencies  

Base your review strictly on the *LLM‑Aware Code Style Guide*.

---

### **2. Rewrite**
Rewrite the modified code so that **all violations are resolved**.

Requirements:

- Apply the style guide rigorously  
- Improve naming, structure, clarity, and predictability  
- Add comments only where they explain intent or constraints  
- Avoid unnecessary abstractions  
- Preserve functionality unless clarity requires a small adjustment  

---

### **3. Validation Loop**
After rewriting:

1. Re‑review the rewritten code against the style guide  
2. Report any remaining violations  
3. If violations remain, repeat the rewrite + validation cycle  
4. Continue until the code is fully compliant  

---

### **4. Final Output**
Provide:

1. The **final cleaned‑up code**  
2. A **short summary** of the most important improvements  
3. No additional commentary  

---

## **Input**
```
[INSERT YOUR DIFF OR CODE HERE]
```

---

# ✔️ **End of PR Cleanup Prompt**

---
