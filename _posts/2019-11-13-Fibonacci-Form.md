---
layout: post
title: A Fibonacci Closed Form
description: >
    This article details an interesting derivation of the closed form of the $$n^\text{th}$$ fibonacci number.
noindex: false
---

I was reviewing some old exercises from my undergrad, and found a very interesting derivation that I thought would be worth sharing. From a mathematical perspective it amounts to relatively basic linear algebra, but from a programming perspective it demonstrates a non-iterative and non-recursive method of determining the $$n^\text{th}$$ Fibonacci number.

**Definition:** We define the Fibonacci sequence, $$F_1, F_2, ... $$ by 

$$F_1 = 1, F_2 = 1, F_n = F_{n-2} + F_{n-1} \ \text{for} \ n \geq 3$$

Moreover, we can define a function, $$T$$, in the space of linear transformations in the real plane, $$\mathscr{L}(\mathbb{R}^2)$$, by 

$$T\left(\left[\begin{array}{l}{x} \\ {y}\end{array}\right]\right)=\left[\begin{array}{c}{y} \\ {x+y}\end{array}\right]$$

We can prove by induction that $$T^{n}\left(\left[\begin{array}{l}{0} \\ {1}\end{array}\right]\right)=\left[\begin{array}{c}{F_{n}} \\ {F_{n+1}}\end{array}\right]$$

**Proof:** The base case, (0,1), is clear. Suppose then that $$T^n(0,1) = (F_n, F_{n+1})$$. We have then 

$$T^{n+1}=T\left(T^{n}(0,1)\right)=T\left(F_{n}, F_{n+1}\right)=\left(F_{n+1}, F_{+2}\right)$$

Hence, the result is true by induction. $$\square$$

Our end goal is to find a normalization of (0,1) with respect to an eigenbasis, which would require us to find and use a set of eigenvalues to generate linearly independent eigenvectors. To find the eigenvalues of $$T$$, we solve $$T(x,y) = \lambda (x,y)$$. This tells us that $$\lambda = \frac{y}{x}$$ and $$\lambda = \frac{x}{y} + 1$$. We can make a substitution to determine $$\lambda = \frac{1}{\lambda} + 1 = \frac{1 \pm \sqrt{5}}{2}$$.

From this, we can generate a pair of linearly independent eigenvectors which comprise our eigenbasis.

$$\left(1, \frac{1 + \sqrt{5}}{2} \right) \ \text{and} \ \left(1, \frac{1 - \sqrt{5}}{2} \right) $$

We write our initial vector (0,1) in terms of this basis to get $$\frac{1}{\sqrt{5}}\left(\left(\frac{1+\sqrt{5}}{2}\right)-\left(\frac{1-\sqrt{5}}{2}\right)\right)$$. We apply $$T^n$$ to yield the following:

$$
\begin{aligned}
T^{n}(0,1) &= T^{n}\left(\frac{1}{\sqrt{5}}\left(\left(\frac{1+\sqrt{5}}{2}\right)-\left(\frac{1-\sqrt{5}}{2}\right)\right)\right) \\ &=\frac{1}{\sqrt{5}}\left(T^{n}\left(\frac{1+\sqrt{5}}{2}\right)-T^{n}\left(\frac{1-\sqrt{5}}{2}\right)\right) \\ &= \frac{1}{\sqrt{5}}\left[\left(\frac{1+\sqrt{5}}{2}\right)^{n}-\left(\frac{1-\sqrt{5}}{2}\right)^{n}\right]=F_{n}
\end{aligned}
$$

Hence, we can compute the $$n^\text{th}$$ fibonacci number by simply calculating $$\frac{1}{\sqrt{5}}\left[\left(\frac{1+\sqrt{5}}{2}\right)^{n}-\left(\frac{1-\sqrt{5}}{2}\right)^{n}\right]$$. This approach requires no recursion or iteration, and is very fast.