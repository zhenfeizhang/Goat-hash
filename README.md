The Ajtai hash function Over Goldilocks (TAOG)
---
__Caution__: A lot things here are still very preliminary...


Hash function defined as

$$\mathcal{H}: \mathcal{R}_{t}\times\mathcal{R}_{t}\times\mathcal{R}_{t}\times\mathcal{R}_{t}\mapsto \mathcal{R}_q$$

with parameters 

| q | n | m | t |
|:-----:|--------|--------|--------|
|`0xffffffff00000001` | 256 | 4 | 16 | 


where 
$\mathcal{R}_{t}:= \mathbb{Z}_{t}[X]/(x^{n} + 1)$ and $\mathcal{R}_{q}:= \mathbb{Z}_{q}[X]/(x^{n} + 1)$ for $q = 2^{64} - 2^{32} + 1$ (i.e., Goldilocks field).

The hash function is defined as

- Setup: $\forall i \in [0,4), a_i \gets_R \mathcal{R}_q$ 
- Hashing messages $m_1, m_2, m_3, m_4\in\mathcal{R}_{16}$
$$\mathcal{H}({\bf m}) = a_1 m_1 + a_2 m_2 + a_3m_3 + a_4m_4 \in \mathcal{R}_q$$


Padding: we need to be careful about the input message. When the message does not fully fill ${R}_{16}^4$, extra cautions need to be taken about padding. We may parse the message as a vector of 4 bits elements, and pad with `t` to achieve a dimension of `256 * 4 = 1024`, and then parse the vector as 4 $\mathcal{R}_t$ elements.

# Performance
## native cost
Single thread, none optimized. Poseidon over Goldilocks with width = 12.
```
poseidon hash cost 5.142µs
goat hash cost 11.846µs 
```
## in circuit cost
Metic: multiplication gates
- Poseidon: 
    - Partial S-box takes 4 muls
    - Full S-box takes 48 muls
    - Permutation takes 144 muls (may accelerate with NTT muls)
    - total costs 22 partial rounds * 4 + 8 full rounds * 48 + 30 total rounds * 144 = `4792` muls

- Goat
    - requires 4 forward NTT and 1 backward NTT which adds up to `10496` muls
        - forward NTT: converting a single poly into NTT poly takes `n * log n = 2048` muls
        - backward NTT: converting a single NTT poly into normal poly takes `n * log n + n = 2304` muls
    - requires additionally 4 NTT multiplications which takes `4 n =  1024` muls
    - total cost `10496 + 1024 = 11520` muls

- Sanity check:
    - 11520/4792 ~ 2.4
    - 11.85/5.15 ~ 2.3