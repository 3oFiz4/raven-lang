# Conlang Development Logbook

Tracks all user inputs, design decisions, and step-by-step progress.

---

## Turn 1: Initialization
- **User input:** "Ok, since you have written the workflow. Guide me in making my first conlang. For every reply, every response, every saying, I utter, you will write it in a separate folder to track everything."
- **Action:** Created `logbook/` tracking directory. Started Phase 1 (Purpose & Aesthetics).
- **Status:** Complete.

---

## Turn 2: Phase 1 Decisions (Purpose & Aesthetics)
- **User input:**
  - "1. My personal secret language that is based on simplicity while upholding clarity"
  - "2. It must be smooth and vocalic, while also being an extremely concise language."
- **Design Decisions:**
  - Type: Personal secret language (cryptolang).
  - Principles: Simplicity, absolute clarity, extreme conciseness (high information density, minimal redundant grammar).
  - Soundscape: Smooth, vocalic (open syllables, melodic vowels, soft consonants, no harsh clusters).
- **Status:** Phase 1 locked. Moving to Phase 2 (Phonology).

---

## Turn 3: Phase 2 Sound Inventory & Syllable Rules
- **User input:** Expanded vowels (14 listed in category/sound_inventory.md), all consonants from English, Japanese, Latin, German. Syllable rule requested: `(C)V(C)` with specific codas or generic codas, requested IPA-aligned markdown tables, full inventory reformatting, and exact syllable permutation calculation.
- **Action:** Reformatted `category/sound_inventory.md` with standard IPA charts, mapped consonants, computed syllable permutations.
- **Status:** Complete.

---

## Turn 4: Syllable Canon Decision
- **User input:** "I thought of 1. Because each concept, each words, must in the form of (C)V."
- **Design Decisions:**
  - Locked Syllable Canonical Form: `(C)V` (open syllables strictly: optional single consonant onset, mandatory single vowel nucleus, zero codas).
  - Sound inventory: 14 vowels, 32 consonants.
  - Total monosyllabic concept roots: 33 onsets (32 C + 1 null) × 14 V = 462 unique monosyllabic words.
  - V: 14 roots.
  - CV: 448 roots.
  - Eliminates all codas (`n`, `ng`, `sh`, `l` dropped from syllable ends) for maximum smoothness and simplicity.
- **Status:** Phase 2 locked. Ready for Phase 3 (Morphology & Word Formation).

---

## Turn 5: Morphology Explanation Request
- **User input:** "What's the different between all of them? Can you clarify more about it?"
- **Action:** Explained the three morphological systems (Pure Isolating, Agglutinative, Compound-Analytic) with simple real-world comparisons and concrete (C)V examples.
- **Status:** Complete.

---

## Turn 6: Clarifying Analytic vs Agglutinative
- **User input:** "I'd say Analytic, but I am also not sure whats the big difference with Agglutinative."
- **Action:** Illustrated the fundamental difference between Analytic (words stay loose like beads on a string) vs Agglutinative (words fuse into a single solid word).
- **Status:** Complete.

---

## Turn 7: Morphology Locked
- **User input:** "Probably analytic."
- **Design Decisions:**
  - Morphology: Pure Analytic / Isolating.
  - Words remain uninflected (C)V tokens separated by spaces.
  - No conjugations, no declensions, no bound affixes.
  - Tense, aspect, number, and roles are indicated strictly by position and optional standalone grammatical particles.
- **Status:** Complete.

---

## Turn 8: Syntax / Word Order Locked
- **User input:** "SOV"
- **Design Decisions:**
  - Constituent order: Subject - Object - Verb (SOV).
  - Sentence pattern: `[Subject] [Object] [Verb]`. Example: *I bread eat*.
  - Typical SOV correlates: Head-final tendencies (Adjective precedes Noun, Postpositions rather than Prepositions).
- **Status:** Complete.

---

## Turn 9: Phrase Structure Decisions
- **User input:** "Option A for Adj., Option B for Prep."
- **Design Decisions:**
  - Adjectives: Pre-nominal (Adjective + Noun, e.g., `[red] [house]`).
  - Adpositions: Prepositions (Preposition + Noun phrase, e.g., `[in] [red house]`).
  - Sentence template: `[Subject] [Prepositional Phrase] [Object] [Verb]`.
- **Status:** Complete.

---

## Turn 10: Vowel Grammatical Particle & Negation Prefix Proposal
- **User input:** "I'd like to emply a Vowel only that acts as a Tense, Aspect. This apply for Negation, Question, marker and many more. But I am not sure whether this idea is productive or not. For now, assume that `i` when attached in a prefix-like manner of an object signify negation. Test proof this. And I can continue with others, I will put them in category/"
- **Action:** Analyzed productivity and tested proof of V-only grammatical prefixes/particles against the 14-vowel inventory and (C)V canonical rule.
- **Status:** Test proof delivered, evaluation documented.
