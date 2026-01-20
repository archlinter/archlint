# Feature Envy

**ID:** `feature_envy` | **Severity:** Medium (default)

Feature envy is like that nosy neighbor who knows more about what's going on in your house than you do. It happens when a method seems way more interested in another class's data than its own.

## Why this is a smell

It’s a classic sign of misplaced logic. If a method is constantly reaching into another object to pull out data and do calculations, that logic probably belongs inside the other object. It breaks encapsulation and makes your classes tightly coupled.

## How to fix

Move the method (or the part of the method that has envy) to the class whose data it is using.
