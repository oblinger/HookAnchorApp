# Prelim.Motivating-example -- Example to motivate pi w. multiple spaces

    >>> MOTIVATING EXAMPLES (two types to be combined)
    - Ex. of two uncertain stmts with different types of appropriate refinements.
      ``I visit my parents in Cincinnati on Christmas and easter.''
      ``I always visit my parents in Cincy on certain holidays.''
      ``Our group meets in BI2414 every Tueday at 2:30.''
      Since At(BI2414, time) implies Not At(Cincy, time) we can infer
      that Christmas never occurs on a Tuesday.
    - ANOTHER MOTIVATING EXAMPLE
      ``Cars can locomote''
      ``Broken things cannot locomote''
      ``Some cars don't function after a hard rain''
    XXXX But if x is left of y and y is left of z then I <<>>
     mean that x is left of z.  I means <<>> of the deductive closure.
    this sounds like a weak commitment.  we see later that this actually
    still provides a great deal of information (bias) for learning.
    - Computational account:
      Christmas(time) -> At(Cincy, time)
      Tuesday(time) and InRange(TimeOfDay(time), start, end) -> At(BI2414, time)
      Near(start, 2:30)
      Near(end, 5:00)
      At(x,t) and ~Eq(x,y) -> ~At(y,t)
      SPACES: override: At, threshold: Start End
      At(Cincy, time) <- ...
      Start near 2:30
    ===============================================================================
    RefType: can-locomote(x) overriding-combinations
    RefType: lots-of-rain, after-delta continuous thresholds
       Car(x) < CanLocomote(x)
       Broken(x) < Car(x) ^ LotsOfRain( InchesOfRain )
       ~CanLocomote(x) < Broken(x)
       LotsOfRain in [0..4]
