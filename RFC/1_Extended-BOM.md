Variants of the Backward-Oracle-Matching Algorithm

```
1. Extended-BOM(p, m, t, n)

δ ← precompute-factor-oracle(p)
for a ∈ Σ do
	q ← δ(m, a)
	for b ∈ Σ do
		if q = ⊥ then λ(a, b) ← ⊥
		else λ(a, b) ← δ(q, b)
t[n .. n + m − 1] ← p
j ← m − 1
while j < n do
	q ← λ(t[j], t[j − 1])
	while q = ⊥ do
		j ← j + m − 1
		q ← λ(t[j], t[j − 1])
	i ← j − 2
	while q 6 = ⊥ do
		q ← δ(q, t[i])
		i ← i − 1
	if i < j − m + 1 then
		output(j)
		i ← i +1
	j ← j + i + m

```

```
2. Forward-Bom(p, m, t, n)

δ ← precompute-factor-oracle(p)
for a ∈ Σ do
	q ← δ(m, a)
	for b ∈ Σ do
		if q = ⊥ then λ(a, b) ← ⊥
		else λ(a, b) ← δ(q, b)
q ← δ(m, p[m − 1])
for a ∈ Σ do λ(a, p[m − 1]) ← q
t[n .. n + m − 1] ← p
j ← m − 1
while j < n do
	q ← λ(t[j + 1], t[j])
	while q = ⊥ do
		j ← j + m
		q ← λ(t[j + 1], t[j])
	i ← j − 1
	while q 6 = ⊥ do
		q ← δ(q, t[i])
		i ← i − 1
	if i < j − m + 1 then
		output(j)
		i ← i +1
	j ← j + i + m
```

```
3. Forward-SBNDM(p, m, t, n)

for all c ∈ Σ do B[i] ← 1
for i = 0 to m − 1 do B[p[i]] ← B[p[i]] | (1 << (m − i))
j ← m − 1
while j < n do
	D ← (B[t[j + 1]] << 1) & B[t[j]]
	if D 6 = 0 then pos ← j
	while D ← (D + D) & B[t[j − 1]] do j ← j − 1
	j ← j + m − 1
	if j = pos then
		output(j)
		j ← j +1
	else j ← j + m
```