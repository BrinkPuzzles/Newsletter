Assuming one of the callers is lying means they have a profile, but their answers don’t match it. Looping through, assuming each is the hacker in turn and validating the resulting truth table:

### Python

```python

for (i, answer) in enumerate(answers):
    mt = match_table(profiles, answers)
    for (j, profile) in enumerate(profiles):
        mt[i][j] = not mt[i][j]
    if is_valid(mt):
        print("Hacker found: ", i, answers[i])
```

### Javascript

```javascript

callers.forEach((answer, i) => {
  const mt = matchTable(profiles, callers)
  profiles.forEach((profile, j) => {
    mt[i][j] = !mt[i][j]
  })
  if (isValid(mt)) {
    console.log('Hacker found: ', i, callers[i], mt)
  }
})
```

### Rust

```rust

    for i in 0..callers.len() {
        let mut mt = match_table(&profiles, &callers);
        for j in 0..profiles.len() {
           mt[i][j] = !mt[i][j];
        }
        if is_valid(&mt) {
            println!("Hacker found: {} ({:?})\n{:#?}", i, callers[i], mt);
        }
    }

```
