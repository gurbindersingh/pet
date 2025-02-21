# PET

1. [What is HTTP? What is HTTPS? Why use it instead of HTTP? How can an attacker circumvent HTTPS? What is a possible countermeasure against that attack? What side-channel attack on HTTPS can be used to infer information about the user?](#what-is-http-what-is-https-why-use-it-instead-of-http-how-can-an-attacker-circumvent-https-what-is-a-possible-countermeasure-against-that-attack-what-side-channel-attack-on-https-can-be-used-to-infer-information-about-the-user)
2. [How can we protect e-mails in transit? Why is that not enough to guarantee CIA? How can we ensure that these properties are fulfilled? Why is their usage not more widespread?](#how-can-we-protect-e-mails-in-transit-why-is-that-not-enough-to-guarantee-cia-how-can-we-ensure-that-these-properties-are-fulfilled-why-is-their-usage-not-more-widespread)
3. [What is DNS?  What structure does it have? What is the problem most default DNS resolvers? How can we fix that? What are shortcomings of these methods?](#what-is-dns--what-structure-does-it-have-what-is-the-problem-most-default-dns-resolvers-how-can-we-fix-that-what-are-shortcomings-of-these-methods)
4. [What is fingerprinting? Why is fingerprinting possible? Name two ways of browser fingerprinting. How do they work? How can fingerprinting be mitigated and what is a possible downside?](#what-is-fingerprinting-why-is-fingerprinting-possible-name-two-ways-of-browser-fingerprinting-how-do-they-work-how-can-fingerprinting-be-mitigated-and-what-is-a-possible-downside)
5. [What techniques can be used for censorship (detection, blocking)? And what techniques can be used to control access?](#what-techniques-can-be-used-for-censorship-detection-blocking-and-what-techniques-can-be-used-to-control-access)
6. [What are the key mechanism of the Great Firewall of China?](#what-are-the-key-mechanism-of-the-great-firewall-of-china)
7. [What are domain fronting and FEPs used for? How do they work? What is one weakness of FEPs?](#what-are-domain-fronting-and-feps-used-for-how-do-they-work-what-is-one-weakness-of-feps)
8. [What are pseudorandom functions (PRFs)? When is a function considered pseudorandom?](#what-are-pseudorandom-functions-prfs-when-is-a-function-considered-pseudorandom)
9. [What does CPA-secure mean? How can we construct a simple CPA-secure private-key encryption scheme? Why is CPA-security not enough in practice?](#what-does-cpa-secure-mean-how-can-we-construct-a-simple-cpa-secure-private-key-encryption-scheme-why-is-cpa-security-not-enough-in-practice)
10. [What are signatures? What property must a signature scheme have to be considered secure and explain what it means. What security game tests this property?](#what-are-signatures-what-property-must-a-signature-scheme-have-to-be-considered-secure-and-explain-what-it-means-what-security-game-tests-this-property)
11. [What are hash functions? What can they be used for? What is a possible downside of hashes? Why does it not matter in most cases?](#what-are-hash-functions-what-can-they-be-used-for-what-is-a-possible-downside-of-hashes-why-does-it-not-matter-in-most-cases)
12. [How does the Diffie-Hellman key exchange work? What problem does it rely on for its security? What does it mean? How is it different from the DLog problem? What security game do we use to test its security?](#how-does-the-diffie-hellman-key-exchange-work-what-problem-does-it-rely-on-for-its-security-what-does-it-mean-how-is-it-different-from-the-dlog-problem-what-security-game-do-we-use-to-test-its-security)
13. [What is public-key (asymmetric) encryption? What is the advantage of asymmetric encryption? What are the downsides? What public-encryption scheme did we learn about in the lecture? Explain how it works. What problem is the security of public-key encryption based on? Explain what it means. In a CPA-secure public-key encryption scheme, what makes two ciphertexts $c\_1$ and $c\_2$ indistinguishable?](#what-is-public-key-asymmetric-encryption-what-is-the-advantage-of-asymmetric-encryption-what-are-the-downsides-what-public-encryption-scheme-did-we-learn-about-in-the-lecture-explain-how-it-works-what-problem-is-the-security-of-public-key-encryption-based-on-explain-what-it-means-in-a-cpa-secure-public-key-encryption-scheme-what-makes-two-ciphertexts-c_1-and-c_2-indistinguishable)
14. [What is TLS used for? Why use TLS? How does the TLS (1.3) handshake work? How does TLS ensure forward secrecy? What is the TLS Record Protocol? Name 4 kinds of attacks on TLS and how to mitigate them.](#what-is-tls-used-for-why-use-tls-how-does-the-tls-13-handshake-work-how-does-tls-ensure-forward-secrecy-what-is-the-tls-record-protocol-name-4-kinds-of-attacks-on-tls-and-how-to-mitigate-them)
15. [What is Grover's algorithm? What is Shor's algorithm? What problem do they create for cryptography?](#what-is-grovers-algorithm-what-is-shors-algorithm-what-problem-do-they-create-for-cryptography)

---

## What is HTTP? What is HTTPS? Why use it instead of HTTP? How can an attacker circumvent HTTPS? What is a possible countermeasure against that attack? What side-channel attack on HTTPS can be used to infer information about the user?

- HTTP is communication protocol used for communication over a TCP/IP connection.
- HTTPS extends HTTP by adding encryption using SLL/TLS.
- By employing SSL stripping, a MITM attack.
- By configuring HSTS, HTTP Strict Transport Security, which only allows clients to connect via HTTPS.

---

## How can we protect e-mails in transit? Why is that not enough to guarantee CIA? How can we ensure that these properties are fulfilled? Why is their usage not more widespread?

- We can employ SSL/TLS to encrypt emails in transit.
- It only encrypts the connection but not the content of the email.
- We can use PGP/GPG or S/MIME to encrypt and sign the email.
- Setting these up requires a certain level of technical skills as they are not configured out of the box which has hindered the widespread adoption.

---

## What is DNS?  What structure does it have? What is the problem most default DNS resolvers? How can we fix that? What are shortcomings of these methods?

- DNS is used to resolve domain names to IP addresses.
- DNS is structured like a tree. At the top are the root DNS servers, followed by the TLD servers and then authoritative servers.
- Most DNS queries are performed over an unencrypted connection so enables tracking and censorship.
- By using DNS over TLS or HTTPS.
- They do not protect against data leaks on the recursive resolver level.

---

## What is fingerprinting? Why is fingerprinting possible? Name two ways of browser fingerprinting. How do they work? How can fingerprinting be mitigated and what is a possible downside?

- Fingerprinting is a technique where various data is collected in order to create a unique ID linked to user.
- Fingerprinting is possible because of the diversity of devices and how they implement certain features and because of legal and regulatory gaps allowing such practices.
- HTML Canvas fingerprinting and HSTS supercookies:
  - Canvas fingerprinting: Render an invisible image on a canvas and take its hash.
  - HSTS supercookies: Load resources from a number of different subdomains, set different HSTS settings for each subdomain. The browser will cache this settings and the next time the user visits the site, the pattern of requests to the subdomains that use HTTPS will reveal the user.
- Fingerprinting can be mitigated by using plugins designed to block those technique.
- The problem is that if not enough people use the plugin or if it introduces patterns, it might have the opposite effect.

---

## What techniques can be used for censorship (detection, blocking)? And what techniques can be used to control access?

- Detection:
  - Check destination IP 
  - Inspect DNS queries
  - Inspect Server Name Indication
    - (SNI is used when multiple hostnames are available on the same IP and port)
- Blocking:
  - IP null routing
  - TCP Reset injection
  - DNS Response injection 
  - HTTP redirect injection 
  - Packet dropping
  - URL filtering
- Access control:
  - DoS
    - disrupt services
  - BGP hijacking
    - To redirect traffic through compromised routes
  - Network disconnect

---

## What are the key mechanism of the Great Firewall of China?

- Website blocking
- Keyword filtering
- IP blocking and domain overblocking
- Traffic analysis and manipulation
- Geoblocking
- Forged IP injection

---

## What are domain fronting and FEPs used for? How do they work? What is one weakness of FEPs?

- For circumventing censorship.
- Domain fronting:
  - The client connects to domain that appears legitimate and is not blocked. As part of the (encrypted) headers it sends the actual domain it wants to connect to. The server than returns that resource.
- Using FEPs (fully encrypted protocols):
  - They are designed to hide all protocol metadata. They obfuscate all traffic, allowing users to bypass network filters.
- FEPs can only disguise the content but not the traffic volume.

---

## What are pseudorandom functions (PRFs)? When is a function considered pseudorandom?

- PRFs are functions that are indistinguishable from an actual random.
- A function is considered pseudorandom if an probabilistic polynomial time distinguisher if the probability of distinguishing between a PRF and an actual truly random function is negligible.

--- 

## What does CPA-secure mean? How can we construct a simple CPA-secure private-key encryption scheme? Why is CPA-security not enough in practice?

- CPA-secure means that an encryption scheme is secure against a Chosen Plaintext Attack:
  - A PPT adversary sends to chosen plaintext messages of equal length to the challenger. The challenger encrypts on of these messages and returns the ciphertext. The adversary must then guess which message was encrypted and wins if it guesses correctly.
- Encryption scheme:
  - As a random key generated using the security parameter $\lambda$ and a nonce $r$ to derive a pseudorandom value and mask the plaintext message using that value: $c \leftarrow Enc(F(k,r) \oplus m)$. Send the nonce alongside the ciphertext so that it can be decrypted again. The nonce is unique for each message so the same message encrypted two different times would produce different ciphertexts.
- It only considers a passive attacker (eavesdropper) and fails to account for their ability to manipulate messages.

---

## What are signatures? What property must a signature scheme have to be considered secure and explain what it means. What security game tests this property?

- Signatures are used to verify the authenticity of a message.
- They have to have Existential Unforgeability under Chosen Message Attack (EUF-CMA). That means that a PPT adversary cannot generate a valid signature for an arbitrary message of its choice.
- Security game:
  - The challenger generates a signing key and verification key and gives the latter to the adversary. 
  - The attacker can ask for signatures for messages of its choice.
  - The attacker must produce a valid message-signature pair that was not previously signed by the challenger.

---

## What are hash functions? What can they be used for? What is a possible downside of hashes? Why does it not matter in most cases?

- Hash functions produce a fixed-sized digest from arbitrary-sized data.
- They can be used for 
  - more efficient indexing and lookup
  - signatures 
  - securely storing passwords 
  - file integrity checks 
- Collisions are possible (pigeon hole principle) but pairs are unknowns and difficult to compute. Also in cases where cryptographic security is required, usually a salt is added to the data prior to hashing, making collisions very unlikely or even impossible.

---

## How does the Diffie-Hellman key exchange work? What problem does it rely on for its security? What does it mean? How is it different from the DLog problem? What security game do we use to test its security?

- DH key exchange:
  1. A: Chooses a number $x$, computes $h_A = g^x$ and sends it to B.
  2. B: Chooses a number $y$, computes $h_b = g^y$ and sends it to A.
  3. A and B compute $h_B^x = g^{yx}$ and $h_A^y = g^{xy}$ respectively.
- Its security is based on the Decisional Diffie-Hellman problem.
- The DDH involves distinguishing a random independent triple of elements $(g^a, g^b, g^c)$ from a cyclical group from $(g^a, g^b, g^{ab})$.
- The DDH assumes indistinguishabilty of group elements while DLog assumes difficultly of computing the discrete logarithm.
- Security game: 
  - Given a transcript of a key exchange and key, can a PPT distinguisher $A$ distinguish between an actual key and a random one?
---

## What is public-key (asymmetric) encryption? What is the advantage of asymmetric encryption? What are the downsides? What public-encryption scheme did we learn about in the lecture? Explain how it works. What problem is the security of public-key encryption based on? Explain what it means. In a CPA-secure public-key encryption scheme, what makes two ciphertexts $c_1$ and $c_2$ indistinguishable?

- Public-key encryption uses a key pair, one for encryption and one for decryption, instead a single secret key.
- It eliminates the key distribution problem and enables the use of signatures for message authenticity.
- Asymmetric encryption is more complex and slower than symmetric encryption.
- ElGamal Encryption:
  - Generate a key pair: 
    - `sk` (secret key): some number $x$
      - strictly speaking it's a tuple $\langle \mathbb{G}, q, g, x \rangle$ with
        - $x \in \Z_q$
        - a generator $g \in \mathbb{G}$
        - where $\Z_q$ and $\mathbb{G}$ are cyclical groups
    - `pk` (public key): $h = g^x$
      - this number will again be an element of $\mathbb{G}$
      - $x$ cannot be easily computed from $h$
  - Encryption step:
    - Pick random number $y \in \Z_q$ and compute $g^y$
    - Encrypt message using $h^y$
    - Send $\langle g^y, m \cdot h^y \rangle$
  - Decryption step:
    - $m = \frac{m \cdot h^y}{(g^y)^x} = \frac{m \cdot g^{xy}}{g^{xy}}$
- The discrete logarithm problem:
  - Given numbers $a$ and $b$, computing $c$ such that $b^c = a$ is difficult for a PPT adversary. In other words $c = \log_b(a)$.
- The encryption function hides all information about the plaintext except its length.

---

## What is TLS used for? Why use TLS? How does the TLS (1.3) handshake work? How does TLS ensure forward secrecy? What is the TLS Record Protocol? Name 4 kinds of attacks on TLS and how to mitigate them.

- TLS is used for secure communication over a network.
- TLS provides confidentiality, integrity and authenticity, which unencrypted protocols don't.
- The handshake is essentially an extension of the DH key exchange:
  1. The client sends its public DH key share and handshake parameters.
  2. Server generates the shared secret and sends back its public DH key share, its certificate, a signature of the transcript and handshake parameters.
  3. The client verifies the certificate and signature, generates the shared secret sends a confirmation message back to the server encrypted using the shared secret.
- TLS ensures forward secrecy by using new keys for each new session.
- TLS Record Protocol is used to encrypt and authenticate application data. It uses the parameters established during the handshake and ensures confidentiality and integrity by dividing traffic into a series of records and protecting each with traffic keys and MACs.
- Attacks:
  - MITM attacks: attacker tries intercept and redirect traffic through their own server.
    - Fix: Certificate pinning.
  - Downgrade attacks: force to use older less secure versions.
    - Fix: Enforce modern TLS versions.
  - Heartbleed
    - Fix: Enforce modern TLS versions.
  - TLS stripping.
    - Fix: Use HSTS.

---

## What is Grover's algorithm? What is Shor's algorithm? What problem do they create for cryptography?

- Grover's algorithm:
  - A quantum search algorithm for unstructured search problems that provides quadratic speed-up (finds specific item in an unordered database in $O(\sqrt{n})$ time).
- Shor's algorithm:
  - A quantum algorithm that can factor large numbers in polynomial time.
- Both are a threat to cryptography as it relies on problems being difficult to solve.
