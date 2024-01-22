The Ajtai hash function Over Goldilocks 
---

Hash function defined as

$$\mathcal{H}: \mathcal{R}_{t}\times\mathcal{R}_{t}\times\mathcal{R}_{t}\times\mathcal{R}_{t}\mapsto \mathcal{R}_q$$

with parameters 

| q | n | m | t |
|:-----:|--------|--------|--------|
|`0x10000000100000001` | 256 | 4 | 16 | 


where 
${R}_{t}:= \mathbb{Z}_{t}[X]/(x^{n} + 1)$ and ${R}_{q}:= \mathbb{Z}_{q}[X]/(x^{n} + 1)$ for $q = 2^{64} + 2^{32} + 1$ (i.e., Goldilocks field).

The hash function is defined as

- Setup: $\forall i \in [0,4), a_i \gets_\$ \mathcal{R}_q$ 
- Hashing messages $m_1, m_2, m_3, m_4\in\mathcal{R}_{16}$
$$\mathcal{H}({\bf m}) = a_1 m_1 + a_2 m_2 + a_3m_3 + a_4m_4 \in \mathcal{R}_q$$


Padding: we need to be careful about the input message. When the message does not fully fill ${R}_{16}^4$, extra cautions need to be taken about padding. We may parse the message as a vector of 4 bits elements, and pad with `t` to achieve a dimension of `256 * 4 = 1024`, and then parse the vector as 4 $\mathcal{R}_t$ elements.