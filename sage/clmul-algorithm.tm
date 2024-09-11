<TeXmacs|1.99.17>

<style|generic>

<\body>
  <section|Reduction algorithm>

  We are really looking to find a multiplicative inverse of\ 

  <\equation*>
    m p<rsub|16><around*|(|x|)>=x<rsup|16>+\<cdots\>+a<rsub|1>x+a<rsub|0>
  </equation*>

  \ in the Ring of <math|\<bbb-Z\><rsub|2><around*|[|x|]>/<around*|(|x<rsup|16>|)>>.

  Let call that <math|<wide|m p|^><around*|(|x|)>> then we have\ 

  <\equation*>
    m p<around*|(|x|)>*<wide|m p|^><around*|(|x|)>=x<rsup|16>*q<around*|(|x|)>+r<around*|(|x|)>
  </equation*>

  where <math|deg<around*|(||\<nobracket\>>r<around*|(|x|)>>)\<less\>32 but
  that's not enough for us. Because we have lots of <math|r<around*|(|x|)>>
  we need something specific. So I'm going to do this:

  <\math>
    a<around*|(|x|)>b<around*|(|x|)>=c<rsub|1><around*|(|x|)>x<rsup|16>+c<rsub|2><around*|(|x|)>
    mod m p<around*|(|x|)>=

    c<rsub|1><around*|(|x|)>*x<rsup|16> mod m p<around*|(|x|)>
    +c<rsub|2><around*|(|x|)>
  </math>

  now\ 

  <math|c<rsub|1><around*|(|x|)>*x<rsup|16> mod m
  p<around*|(|x|)>=r<rsub|1><around*|(|x|)>\<Rightarrow\>c<rsub|1><around*|(|x|)>*x<rsup|16>=m
  p<around*|(|x|)>*q<rsub|1><around*|(|x|)>+r<rsub|1><around*|(|x|)>\<Rightarrow\>>

  \;

  if I find <math|m p<around*|(|x|)><rsup|-1>> in <math|x<rsup|16>>

  so we need to find <math|q<rsub|1><around*|(|x|)>> becasue that's it now

  \;

  <\equation*>
    \;

    0=m p<around*|(|x|)>*q<rsub|1><around*|(|x|)>+r<rsub|1><around*|(|x|)>
    <around*|(|mod x<rsup|16>|)>\<Rightarrow\>r<rsub|1><around*|(|x|)>=mp<around*|(|x|)>q<rsub|1><around*|(|x|)>
    mod x<rsup|16>\<Rightarrow\>mp<around*|(|x|)>q<rsub|1><around*|(|x|)>=q<rsub|2><around*|(|x|)>x<rsup|16>+r<rsub|1><around*|(|x|)>
  </equation*>

  suppose we know the multiplicative inverse of\ 

  <\equation*>
    m p<around*|(|x|)>*<wide|m p|^><around*|(|x|)>=1
    <around*|(|x<rsup|16>|)>\<Rightarrow\>m p<around*|(|x|)>*<wide|m
    p|^><around*|(|x|)>=q<rsub|2<around*|(|x|)>>x<rsup|16>+1
  </equation*>

  <\math>
    c<around*|(|x|)>m p<around*|(|x|)>*q<rsub|2><around*|(|x|)>=c<around*|(|x|)>x<rsup|16>+c<around*|(|x|)>r<rsub|1><around*|(|x|)>
  </math>
</body>

<\initial>
  <\collection>
    <associate|page-height|auto>
    <associate|page-type|letter>
    <associate|page-width|auto>
  </collection>
</initial>

<\references>
  <\collection>
    <associate|auto-1|<tuple|1|?>>
  </collection>
</references>

<\auxiliary>
  <\collection>
    <\associate|toc>
      <vspace*|1fn><with|font-series|<quote|bold>|math-font-series|<quote|bold>|1<space|2spc>Reduction
      algorithm> <datoms|<macro|x|<repeat|<arg|x>|<with|font-series|medium|<with|font-size|1|<space|0.2fn>.<space|0.2fn>>>>>|<htab|5mm>>
      <no-break><pageref|auto-1><vspace|0.5fn>
    </associate>
  </collection>
</auxiliary>