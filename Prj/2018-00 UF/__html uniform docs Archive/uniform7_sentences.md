
# REST OF HTML


<!DOCTYPE html>

<!--


              This file is getting out of date.  The uniform Vocabulary file is starting to have more current content.


    -->
    
<!--suppress ALL -->
<html>
    <meta charset='utf-8'>
    <meta http-equiv="X-UA-Compatible" content="chrome=1">
    <link href='https://fonts.googleapis.com/css?family=Chivo:900' rel='stylesheet' type='text/css'>
    <link rel="stylesheet" type="text/css" href="stylesheets/stylesheet.css" media="screen">
    <link rel="stylesheet" type="text/css" href="stylesheets/print.css" media="print">
<!--[if lt IE 9]>
    <script src="http://html5shiv.googlecode.com/svn/trunk/html5.js"></script>
<![endif]-->
<title>Essential Elements</title>
<body><div id="container" class="inner">
<header><h1><div style="text-align:center"> Essential Elements Of Software Systems</div></h1></header>
    <!-- Essential aspects of software -->


    <!-- DATA              is about  STRUCTURES  of values
         SEMANTICS         is about  MAPPINGS    to meanings
         COMPUTATION       is about  PROCEDURES  for derivation
         META PROGRAMMING  is about  PROGRAMS    operating on programs
         -->


    <H5>XXX</H5>
    <highlight>
        In essence a <d>YYY</d> is
    </highlight>
        In Lex
    <ul>
        <dt></dt>
        <dd></dd>
        <dt></dt>
        <dd></dd>
    </ul>


    <h1>Introduction</h1>

    <div  style="text-align:center">
        &nbsp; &nbsp; <q>That one is straight from <d><i>the book</i></d>.</q> <br>
        &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  -- Paul Erdos &nbsp; &nbsp; (his highest praise for a mathematical proof)
    </div>
    <br><br>
    <br><br>


    <p> An effective meta-programming framework must go beyond mere computational completeness.
        It must capture computation in a form suitable for manipulation as data &mdash; a "<d>natural</d>" form that
        cleaves constituent aspects of computation at breaking points which allow meaningful/useful translation and
        manipulation to be parsimoniously expressed as a meta-programs.</p>

    <p> Computation must be expressed using the "<d>simplest</d>" possible primitives.  Faithful translation
        of computation across environments build from novel primitives requires translating <i>all</i> aspects
        of that computation and those primitives.  If not chosen well, relatively unimportant details of the
        primitives and computation in the origination context can require enormous effort in the target context
        to faithfully replicate.  Simplicity is paramount.</p>

    <p> It is not enough for a meta-programming paradigm to be computationally complete (turing complete).
        Its computational primitives must be semantically "<d>complete</d>" as well.   They must express important
        semantic notions explicitly using primitives that map consistently onto these semantic notions, so
        it is possible to write meta-programs that manipulate these semantics in reliable ways.
        (For example writing a translator from some aspects of control flow in python onto 'natural' C code, might
        be possible, while translating that same control flow back into 'natural' C code from assembly code would
        be quite a bit harder.) </p>

    <p> An effective meta-programming framework requires a <i>simplest</i>, <i>natural</i>, <i>complete</i> encoding
        of the relevant semantics inherent in the underlying computation.  We call this an <d>essential capture</d>
        of those underlying semantics.</p>

    <br>
    <p> Uniform aspires to be an "essential capture" of the Modern Software Stack.
        This document is the evidence that Uniform might be on the way towards such an essential-capture of
        today's software stack.</p>

    <highlight>
        Each section below begins with a highlight (like this highlight.)
        Each highlights states some simlest, essential "truth" of software systems
    </highlight>

    <p> The intent is that each "truth" so basic that it is satisfied by nearly <i>all</i> software.  Each should
        be so obvious that the software engineer reads each, rolls their eyes an mutters
        <q>well of course.</q>  After each highlight we then outline how Uniform builds significant capability
        based on these common intersection concepts.  It is this basing only on the intersection areas, that allows
        Uniform code to be projected without distortion across software systems mapped into it.   The precise capture
        for each is expressed in the related LEXCORE document.  Here we express the essentials of Uniform in a way
        that allows a read to consider its candidacy as an essential-capture.</p>


    <!-- Old shorter intro
    <br><br>
    A formalization is an <i>essential-capture</i> is of a thing, if is is so simple
    that no part could be simplified or dropped without loosing "essential" aspects of the original thing.
    At the same time an essential-capture must also serve as the basis for a full and practical realizations of
    the original in ways that retain all of the the naturalness and easy of use and practical
    applicabilty of that thing in its originating context.
    <br><br>
    Most ambitious of all, Uniform aims to a <i><d>single</d></i> realization which serves as a practical
    bridging interface and implementing substrate for <i><d>all</d></i> of today's software stack.
    <br><br>
    This document is the evidence that Uniform might be on the way towards such an essential-capture of
    today's software stack. -->




   <!--  &mdash; that is so basic that
        it seems true for all software systems
        an element of modern software that
        states a central apect of all software in a ways that appears to capture a key
    part of it "essence", and is so simple that generally it could not be expressed more simply, nor is it
    reasonable to consider software that that opposes this basic notion.

    Each of the "essence" sections is followed by a partial description of how that notion is captured
    in Lex.  -->

    <!--H5>Bundle</H5>
    <highlight>
        The idea of <d>grouping</d> of things is essential for software.
    </highlight>
        Sets, lists, maps, packages, directory folders etc. are all examples of groupings
        that also have other specific properties.
        <dl>
            <dt>Bundle</dt>
            <dd>In Lex we use the term <d>"bundle"</d> or <d>"container"</d> to refer to a
                grouping that does not have any additional properties beyond being a grouping of elements.</dd>
        </dl-->


        <!--dt>__bindings__</dt>
        <dd>In uniform all semantics are expressed explicitly as the contents of a data structure.
            In uniform <code>__bindings__</code> maps logical names onto the structures that those names "mean".
            What makes a 5, a "five" and a list with five different elements in it a list of length five,
            is the way that they behave when the "+" or add operator is applied to each.  In uniform, all of this
            is controlled by their <code>__bindings__</code> (The Uniform Semantics section provides details.).
        </dd-->

            <!--This means the <code>__bindings__</code> data structure is
            derived, in principle, from the print form of the data is applies to.</dd-->



    <highlight>
        out of place
        - A <d>Spiral</d> <br>
        - A <d>type</d> is a designator on data which restricts its form and usage.<br>
        - A <d>langauge</d> is a representational system
        - <d>Execution</d> a kind of interpretaion <br>
        - <d>Code</d> data with a defined execution language.<br>
        - <d>Linking</d> is the act combining code.<br>
        - <d>Compilation</d> the mapping of code

    </highlight>


    <highlight>
        ORGANIZING SECTIONS<br>
        <d>Essence</d><br>
        <d>Data</d> is a thing that, in principle, can be written down &mdash; i.e. can be expressed symbolically.<br>
        <d>Syntax</d> is a set of rules governing the symbolic expression of data &mdash; in a storable/transmittable form
            (e.g. textual, graphical, binary, ...)<br>
        <d>Bind</d> the act of associated a kind of meaning to an identifier.<br>
        <d>Semantics</d> is the binding of meaning to data.<br>
        <d>Interpretation</d> is the <i>process</i> of mapping data onto the meaning of that data.<br>
        <d>Computation</d> is a specific interpretative process (<d>evaluation</d>) defined over a particular kind of
            data, called <d>Forms</d>.<br>
        <d>Meta-Programming</d> is computation that manipulates other computations, by manipulating
            the Forms used to define that computation.<br>
    </highlight>



    <h1>Central Notions</h1>



    <h2>Uniform</h2>


    <p> <d>Uniform</d> refers to the agenda of reducing all of computation to a single, interdependent, set of essential formulations.<br>
        <d>Uniform</d> also refers to the language definition resulting from this agenda.<br>
        Here we present Uniform <i>intuitively</i> and <i>abstractly</i> as a sequence of term definitions which capture
        notions essential to modern software systems.  In parallel to this sequence of terms, we also formally
        define in the Uniform Language in ways that formally capture the same essential notions intuivitely
        expressed here.</p>



    <!--   -->
    <!-- - The <d>essence</d> of a thing is that part which makes it what it is.-->
    <!-- - An <d>essence</d> is a simplest formulation of a thing.<br> -->
    <!-- -->
    <!--The essence of a thing is a formulation so simple, that any further simplification causes it to no 
        longer <i>be</i> a formulation of that thing. -->
    <!-- An essence is a formulation that is so simple, any further simplification leaves it no longer being a complete -->
    <!--An <d>essence</d> of a thing is a formulation of that thing which retains all important aspects in
        a way that is so simple, that the removal of any part causes it to cease to be a formulation of the thing.
        This requires an essence to be:<br-->
    <br><b>Essence</b>
    <highlight>
        The <d>essence</d> of a thing is a <i>simplest</i>, <i>natural</i> formulation which <i>completely</i> captures important aspects of the thing.<br>
        Specifically the formulation of an essence must be:<br>
        - <i>SIMPLEST</i> such that any further simplification of the formulation causes it to no longer BE a formulation of that thing.<br>
        - <i>COMPLETE</i> such that all important aspects of the thing are captured in the formulation.<br>
        - <i>NATURAL</i> such that important aspects of human-centric perception of the thing are preserved.
    </highlight>

    <p> The <d>Uniform Language</d> is an interdependent composition of essential formulations spanning element of modern software systems.</p>


    <!-- ALT approach

        All things in unifom are called 'Units' 

        Most all aspects of Uniform are expressed as 'Units'
        Units may have structural or functional nature.  All that is about a unit is captured in its structure and in its function


        UNIFORM -- Uniform is a least-commitment model of computation.
        UNIT -- Nearly all aspects of Uniform computation are encoded as _Unit_s.
        UNIT NATURE -- The nature of a unit, that which differentiate one unit from another is entirely define the the _structure_ of the unit and the _function_ of the unit.
        STRUCTURE -- The _structure_ of a unit (graph)


    -->


    <h2>DATA</h2>
    <p> Uniform's representations are build from these least commitment statement about data.</p>

    <br><b>Data</b>
    <highlight>
        <d>Data</d> is symbolically encoded information in a form that supports specific analyses.  Where:<br>
        - <d>Information</d> is a thing that reduces ambiguity or can be used to perform analysis to answer some question.<br>
        - Data is a thing that can be expressed symbolically &ndash; that can be, in principle, <d>written on paper.</d><br>
        - Data's written form must <d>conform to some specification</d> as required by the intended formalized analyses.<br>
    </highlight>



    <!--- Data may be <i>structured</i>; that is some of its elements may be composed from sub-data-elements.<br>
        - Structure affords <i>addressing or indexing</i> of sub-data-elements within a data structure.<br>--> 
    <br><b>Data Structure</b>
    <highlight>
        - Data may by organized into descrete <q><d>Units</d></q>.<br>
        - These units may be recursively <i>composed</i> of sub-data units.<br>
        - A unit that is composed of subunits is called a <d>composite</d> unit.<br>
        - A unit that is not composed of subunits is called an <d>atomic</d> unit.<br>
        - The graph of compositional relationships between data units is called the <d>structure</d> of that data.<br>
        - Structure-based <d>Addressing</d> or <i>Indexing</i> refers to the traversal of the Data Structure graph from one unit to another.<br>
        - Sub-units may be addressed by <d>position</d> using an integer index, or<br>
        - Sub-units may be addressed by <d>name</d> using an alphanumeric identifiers as an index.<br>
        - A <d>path</d> within this structure is the path of integer and alphanumeric indicies needed to traverse between a pair of data units.<br>
        - The <d>address</d> of a unit within a structure is simply the path to the unit from some designated <q>root</q> unit.<br>
        - <br>
        - ADDRESS path relative to some designated "root"<br>
        - VALUE a connected collection of units.<br>
        - PLACE the spot indicated by an address.  ??????????????  any place where a value may be stored
    </highlight>



    <br><b>Atomic Data</b>
    <highlight>
        There are four types of atomic data:<br>
        - <d>Integer</d> &ndash; an atomic unit may be an integer.<br>
        - <d>Number</d> &ndash; an atomic unit may be a floating point value.<br>
        - <d>String</d> &ndash; an atomic unit may be a string value&ndash;a sequence of character symbols.<br>
        - <d>Symbol</d> &mdash; an atomic unit may be a symbol value&ndash;a unit with a unique string value across all symbols.<br>
        (Note: Many implementations place limits on the infinite sets above.)
    </highlight>



    <br><b>Syntax</b>
    <highlight>
        <d>Syntax</d> is a set of rules governing the symbolic expression of data &mdash; in some 
        storable/transmittable/viewable form (e.g. textual, graphical, binary, ...)<br>
    </highlight>



    <h2>UNIFORM SEMANTICS</h2>

    <!--- Lexical Space or <d>Lexspace</d> an idealized page where data might be written
        is the recursively defined infinite space of all places of lex places on the idealized page.
        - A <d>Lex</d> is an idealized "place" on an idealize page where part of a data structure might be "written" -->
    <!--In Uniform we use the term <d>Lex</d> to refer to one of these "places" on the abstract page where data may be written,
        and <d>Lexspace</d> to refer to the infinite structured space of all possible Lex places.  In Uniform we formalize these 
        Lex as a special kind of Unit data with a fixed structure.  Lexspace is then defined as the recursive expansion of a single 
        Lexroot unit where  new sub-Lex are defined for every possible integer or symbol naming or fixed index on every Lex in Lexspace.
        Lexical semantics are then explicitly specfied as structural operations over these Lex units of Lexspace.</p-->
    <br><b>Semantic Space</b>
    <highlight>
        Many formalisms derive in some way from the very primal notion of writing data out on an idealized sheet of paper.
    </highlight>
    <p> In Uniform we formalize this notion as an single infinte "page" (called <d>Lexspace</d>) of "places" (called <d>Lex</d>) where 
        data might be written.  Each Lex <i>place</i> has an unchanging structural relationship with all other lex in Lexspace.
        All Uniform semantics are formalized in terms of this static, infinite Lexspace datastructure.
    </p>



    <!-- <d>Assignment Semantics</d> refers to the conventional model of <q>"assigning"</q> a 
            data <q>"value"</q> to some <q>"place"</q> whose value varies over time.<br-->
    <!--- <d>address</d> &ndash; A kind of data value that can be used to find a place with a data structure.<br>- A <d>name</d> a name is logical (non-structural) indicator of place.<br> -->
    <br><b>Assignment Semantics</b>
    <highlight>
        <d>Assignment Semantics</d> &mdash; the model of <q>assigning</q> a data <q>value</q> to some <q>place</q> where it might be <q>retreived</q> later.<br>
        According to assignment semantics:<br>
        - A <d>value</d> is any unit of data that can be <i>written</i> (assigned).<br>
        - A <d>place</d> is where such a value could be <i>written</i> (assigned).<br>
        - <d>assignment</d> is the <i>writing</i> of a <i>value</i> to a <i>place</i>.<br>
        - <d>access</d> is the <i>reading</i> of a value from a place.  <br>
        According to this widely utilized semantic, the value returned by accessing a place will be the temporally most recent value assigned to that place.
    </highlight>
    <p> The Uniform Language reduces all forms of state to values stored somewhere in lexspace, thus at its lowest level all change and execution in Uniform reduces in the end to as sequence of value assignments in lexspace.</p>



    <br><b>Binding Semantics</b>
    <highlight>
        - <d>Semantics</d> can be understood as the binding of <i>meaning</i> to <i>data</i>.<br>
        - <d>Meaning</d> as we used here it here can always be encoded as data&ndash;where the meaning is 
            implicit in the results obtained from that data when applied in different interpretation contexts.<br>
        - <d>Binding</d> as used here is an association of some atomic symbol with some meaning-giving data.<br>
        - <d>Form</d> &mdash; Data Units intended to underlie interpretation are referred to as <d>source code</d> units or as a <d>form</d>.<br>
    </highlight>



    <!--- A thing is <d>Lexical</d> if it centerally depends upon the written (structural) form of source code data.<br> -->
    <!-- &ndash; a sematic defined in terms of Lexspace.  Thus: -->
    <br><b>Lexical Semantics</b>
    <highlight>
        - <d>Lexical</d> refers to an any aspect of source code that is derived from its written form.  Accordingly:<br>
        - A <b>Lexical Binding</b> is a binding defined a specific lex <q>place</q> the within source code data.
            (For example many languages allow <code>HelloWorld</code> to be bound to one meaning at one place in the code and a different meaning in another.)<br>
        - A <d>Lexical Semantic</d> is one where the bindings themselves are in turn derived from the structure of the
             source code data itself.  (For example variable shadowing rules in many languages use the structure of the source code itself determine which meaning is bound to a given symbol at a given place in the code.)<br>
    </highlight>
    <p> Uniform uses the uniform language itself to define it default binding semantics as a function of source code written into lexspace.  But these are only default binding semantics, Uniform does not fix even this most primitive aspect of the language since its goal is to unify all languages. </p>


    <!-- A <d>Variable</d> is a designators within the structure of source code data which 
            indicates (and possibly names) a <q>place</q> that may be used as the target for assignment.<br-->


    <h2>Uniform Computation</h2>

    <br><d>Interpretation</d>
    <highlight>
        - <d>Interpretation</d> the action of explaining the meaning of something.<br> 
        - <d>Interpretation</d> is the <i>process</i> of mapping data onto the meaning of that data.<br>
        <d>Computation</d> is a specific interpretative process (<d>evaluation</d>) defined over a particular kind of
            data, called <d>Forms</d>.<br>
        <d>Meta-Programming</d> is computation that manipulates other computations, by manipulating
            the Forms used to define that computation.<br>
    </highlight>














    <!--
        - Data's format must <i>conform to some specification</i> required by some software system.<br>
        - A "<d>data type</d>" is a designator used to indicate that a particular datum conforms to a
          particular desigated specification.<br>
        - A datum is said to be an <d>"instance"</d> of a type in the case that it conforms to that type's specification.<br>
        - The term "<d>operator</d>" is used to refer to some software (code) that applies to (operates on) some data.<br>
        - Thus type designators indicate which operators can meaningfully be apply to which data.<br>
        - A "<d>type declaration</d>" is specification of the structure of a data type.<br>
        - A "<d>type indicator</d>" marker on a data instance which is used for indicating it structure, and thus
          what functions might be applied to it.<br>-->
    <br><d>Types</d>
    <highlight>
        - Data must be suitable for specific kinds of processing.  The kind of processing for which 
          a data unit is suitable is called its data <d>type</d>.<br>
        There are three important aspects to type:<br>
        - The <d>intrinsic</d> type (or "duck" type) for a unit of data refers to it inherent suitability 
            for some specific form of processing.<br>
        - The <d>indicated</d> type for a unit refers to some designator (usually a type symbol) 
            which can be efficiently derived from the unit itself.
        - A type <d>spec</d> (specification) is a declaration/computation used to determine of a data unit 
            conforms to some specific standard.
    </highlight>


        <br><br><br>
        ========================= NOT SURE ABOUT THESE ======================<br>

        
    <highlight>
        <b>BINDING</b> a process which occurs when combining software systems. During binding, "references" within
        the parts being combined "get resolved" to specific "targets" within the combined system.
    </highlight>

    <highlight>
        <b>code</b> is a written specification of a some computation which can be executed.
        If it completes it could return a result.<br>
        An <b>operator</b> is code which operates on a thing and returns a result.
    </highlight>

    <highlight>
        A software <dq>package</dq> is a source code grouping of elements which define semantics.
    </highlight>

        <pre><highlight>"Structured Data" is a recursive structuring of progressively
            smaller elements, ultimately terminating in some primitive data type.</highlight></pre>


        <pre><highlight>The very intuitive notion of writing out code or data on paper
            is "lexical" basis for most code and data semantics.</highlight></pre>

        <pre><highlight>The software notion of <i>scoping</i> is about which parts of a composite are "visible" (referencable)
            by other parts of the composite.</highlight></pre>


        <pre><highlight>The essence of <i>computation</i> is a process of following the steps of some structured "code" form
            with access to the data expressed in some structured "environment" form.</highlight></pre>


        <pre><highlight>An INVOCATION is some bit of <i>source</i> code that somehow causes a bit parameterized computation to occur
            over another bit of <i>targeted</i> or 'called' code.</highlight></pre>

        <pre><highlight>The specification of a "type" for some data is in essence a specification
            which values are and are not permissible for that data.</highlight></pre>


        <pre><highlight>A "template" is a computable structure that produces outputs whose structure
            parallels that of the structure of the template that constructed it.</highlight></pre>

        A <i>template</i> is an expression that yields another expression which is structurally analgous to itself
        when executed.  Lexlang provides the <b>template</b> special form to facilitate such templating:<br>

        <pre><highlight>"Compilation" is an execution which accepts "source" code, and returns "target"
            code, which is functionally equivelant to the soruce form, and is generally expected to be
            faster to execute.</highlight></pre>



        <br><br><br>
        ========================= END STUFF ======================

    <br><b>Software Terms</b>
    <highlight>
        - A <d>name</d> is a constant value which can be used to select a specific unit within some context.<br>
        - <d>named</d> indicates that a unit has an associated naming contant.<br>
        - A <d>package</d> is a collection of <i>code</i> intended to be managed as a logical unit.<br>
        - A <d>module</d> is a single <i>code</i> unit.<br>
        - A <d>version</d> is one of a set of units that might serve for some common purpose.<br>
        - A <d>version_number</d> is an identifier that uniquely identifies a version.<br>
    </highlight>

    <br><b>Society</b>
    <highlight>
        - 
        - <d>Agency</d> a unit with an exogenous processing element.
        - <d>Election</d><br>
        - <d>Governance</d><br>


        n externally 
        - An <d>agent</d> is a unit with "agency" &ndash; with a source of 
    </highlight>


        <br><br><br>
        ========================= NEEDS CLEANUP ======================


    <br><b>Syntax and Semantics</b>
    <p> The Uniform framing of computation is organized around the following organizing areas:</p>
    <highlight>
        <d>Data</d> is a thing that, in principle, can be written down &mdash; i.e. can be expressed symbolically.<br>
        <d>Syntax</d> is a set of rules governing the symbolic expression of data &mdash; in written (textual, graphical, ...) form.<br>
        <d>Semantics</d> is the binding of meaning to data.<br>
        <d>Interpretation</d> is the <i>process</i> of mapping data onto a meaning of that data.<br>
        <d>Computation</d> is a specific interpretative process (<d>evaluation</d>) defined over a particular kind of
            data, called <d>Forms</d>.<br>
        <d>Meta-Programming</d> is computation that manipulates other computations, by manipulating
            the Forms used to define that computation.<br>
    </highlight>
    <dl>
        <dt><d>Data Units</d></dt>
        <dd>In Uniform all data is represented as a graph of nodes called <q>Units</q>.
            (The Uniform Data section provides details.)</dd>

        <dt><d><q>Uniform</q> Syntax</d></dt>
        <dd><q>unit form</q> provides a simple uniform syntax for data and <d>Uniform Syntax</d>
            <a href="uniform2_example_syntax.html">described here</a> provides a much richer "markdown"
            style syntax for Unit data. </dd>

        <dt><d>Uniform Semantics</d></dt>
        <dd>In Uniform, semantics are formalized as in terms of a Unit interpretation process call
            &alpha;-interpretation &mdash; a generaliation of predicate calculus that applies to and is defined
            in terms of unit-data.</dd>

        <dt>Lexical Semantics</dt>
        <dd>The only thing we have committed to about data is that is a thing that can be written on paper.
            Thus Uniform semantics (&alpha;-interpretation) is lexically defined, that is
            &alpha;-interpretation is defined in terms of the lexical context of the data being interpreted.</dd>

        <dt>Computation</dt>
        <dd>In Uniform <q>code</q> is simply a kind of unit data, with the <b>eval</b>-interpretation defined
            over it.  (see the Uniform Computation section for details.)</dd>

        <dt>Meta-Programming</dt>
        <dd>Since in Uniform both <i>code</i> and <i>semantics</i> (the bindings mapping) are explicit
            data units "<d>meta-programming</d>" is nothing more than the derivation of the latter from the former.
            (see Uniform Meta-Programming for details.)</dd>
    </dl>




    <H2>DATA</H2>
    The following is an <i>essential capture</i> of the notion of <q>structured data</q>:




    <dl>
        <dt>Unit</dt>
        <dd>In Uniform we refer to a single datum as a <q>unit</q> of data.</dd>
        <dt>Unit-Form</dt>
        <dd>A text format for expressing trees of units as a string of characters that could in principle be presented on a page.</dd>
        <dt>Uniform Format</dt>
        <dd>An extension of <i>Unit Form</i> with variants allowing "beautiful" expression of all data used in modern software.</dd>
    </dl>
    <br>


    <dl>
        <dt>composite</dt>
        <dd>In Lex some data units are <d>"composite units"</d> they are a bundle of other units.</dd>
        <dt>parent / children</dt>
        <dd>The units of such a bundle are called <d>sub-units</d> or <d>children</d> of the
            <d>"parent"</d> unit which "comtains" them.</dd>
        <dt>subspace / root</dt>
        <dd>Since bundling is potentially recursive, each unit is the <b>"root"</b> of a tree of sub-units.
            We call this tree the unit's <b>"subspace"</b>, this include the unit itself and all descendents.</dd>
        <dt>key</dt>
        <dd>In Lex each contained unit is accessed from its parent using its <d>key</d>.
            The key can be any <em>string</em> or <em>integer</em> as long as it is unique among all keys
            for sub-units of that composite.</dd>
        <dt>value</dt>
        <dd>In Lex we refer to the sub-unit accessed by a key as the <d>"value"</d> of that key
            for the parent.</dd>
        <dt>fixed / named / meta</dt>
        <dd>There are three predefined kinds of keys and thus three kinds of sub-units for a composite:<br>
            - A <d>"fixed"</d> key is any non-negative integer.<br>
            - A <d>"named"</d> key is any string of alpha-numeric characters.<br>
            - A <d>"meta"</d> key is any integer or string taht is not a fixed or named key.</dd>
    </dl>


    <H5>Atomic Data</H5>
    <highlight>
        - <d>Atomic data</d> is data that is not composite &mdash; it cannot be further broken down. <br>
        - Most languages have a number of predefined kinds of atomic data.<br>
        - Each predefined kind has it own specialized operations that can be performed on that kind of data.
    </highlight>
    <dl>
        <dt>int / num / str / sym</dt>
        <dd>In Lex there are four kinds of atomic units: integers, numbers, strings and symbols.<br>
            They are defined in terms of the well-used JSON data types (symbol is just a specially marked string):<br></dd>
        <dt>as uniform</dt>
        <dd>For uniformity sake atomd are expressed in canonical uniform as a "fake" composite with
            a head and one key argument.<br>
            &nbsp; &nbsp; &nbsp; <code><d>int</d>(<d>"</d><i>12345600</i><d>"</d>)</code> &mdash; <d><i>integer</i></d> <br>
            &nbsp; &nbsp; &nbsp; <code><d>num</d>(<d>"</d><i>123.4560</i><d>"</d>)</code> &mdash; <d><i>numeric</i> </d> <br>
            &nbsp; &nbsp; &nbsp; <code><d>str</d>(<d>"</d><i>some string</i><d>"</d>)</code> &mdash; <d><i>string</i> </d> <br>
            &nbsp; &nbsp; &nbsp; <code><d>sym</d>(<d>"</d><i>some_symbol</i><d>"</d>)</code> &mdash; <d><i>symbol</i> </d> <br><br></dd>
        <dt> Importing JSON</dt>
        <dd> Lex adds two additional predefined heads in order have a well defined mapping of JSON into Canonical Uniform:<br>
            &nbsp; &nbsp; &nbsp; <code><d>lst</d>(<i>ele0</i>, <i>ele1</i>, ...)</code> <br>
            &nbsp; &nbsp; &nbsp; <code><d>obj</d>(<i>key0</i><d>:</d><i>value0</i>, <i>key1</i><d>:</d><i>value1</i>, ...)</code><br>
            Note the three symbols in JSON are mapped as:
            <code>sym("true"), sym("false")</code>, and <code>sym("null")</code></dd>
    </dl>




    <H2>TYPES</H2>
    <dl>
        <dt>type declaration</dt>
        <dd></dd>
        <dt>the type itself</dt>
        <dd></dd>
        <dt>the operators on instances of a type</dt>
        <dd></dd>
        <dt>head</dt>
        <dd>- In Lex all data units must have a <d>head</d>. <br>
            - A unit's "head" is the value indexed by the special  <code><i><d>"^"</d></i></code> meta key.<br>
            - A unit's head must be a non-empty alphanumberic string.</dd></dl>






    <h2>SYNTAX</h2>
    <h2>Uniform syntax -- Textual Form</h2>
    <dl>
        <dt>uniform</dt>
        <dd>Lex provides a flexible syntax for printing and parsing unit-data called <d>"uniform"</d>.<br>
            Uniform is a <b><i>single</i></b> syntax designed to capture <i><b>all</b></i> information used by
            <b><i>all</i></b> software systems in a form that is a beautiful and easy to use as the
            <b><i>thousands</i></b> of syntaxes in use today.  (See the Uniform Language Spec for details.)</dd>
        <dt>canonical uniform</dt>
        <dd>In this document we will limit ourselves to the subset called <b>canonical uniform</b>.<br>
            Canonical Uniform presents units textually as if it were a function call with keyword arguments:<br>
            &nbsp; <code><i>head</i>(<i>fixed_0</i>, <i>fixed_1</i>, ...,
                <i>key_a</i><b>:</b><i>val_a</i>, <i>key_b</i><b>:</b><i>val_b</i>, ...,
                <b>"</b><i>meta_a</i><b>":</b><i>met_a</i>, ...)</code></dd>
        <dt>arguments / properties / fields / pointer / etc. </dt>
        <dd>Sometimes a unit actually <i>is</i> a function call, so when convenient we will refer to a unit's subunits
            as fixed, keyword, and meta <d>"arguments"</d>  </dd>
        <dt>1-to-1</dt>
        <dd>The mapping from units to valid canonical uniform documents is one-to-one and onto, this
            means for every unit structure there is exactly one sequence of characters that expresses it, and
            for every valid canonical uniform expression there is exactly one composite unit that expresses it.</dd></dl>


    <H5>Canonical Form verses Native form</H5>
    <dl>
        <dt>as a Data Structure</dt>
        <dd>Canonical Uniform expressions are a mapping from integers and strings
            onto canonical uniform subexpressions or strings, this data structure is called
            <b>canonical form</b>.</dd>
        <dt>Uniform JSON Form</dt>
        <dd>Since strings and maps are both part of JSON.  Canonical Uniform  (and all unit data)
            is trivially expressed as recursive JSON objects.  This is call <b>"Uniform JSON form"</b></dd>
        <dt>Native Form</dt>
        <dd>Both in memory, and in JSON it is more natual to use predefined integers, numbers, and lists.
            These versions of Uniform are entirely equivelent (as long as the implementations are compatible
            with JSON primitive types.  In this document we use Canonical Uniform for uniformity, but
            the reader should imagine all of these structures are encoded and operated on in type efficient ways.</dd></dl>



    <!-- https://en.wikipedia.org/wiki/Semantics
     Semantics focuses on relationship between signifiers and what they stand for. -- wikipedia
    Semantics (from Ancient Greek: σημαντικός sēmantikos, "significant")[1][2] is the philosophical and scientific study of meaning—in language, programming languages, formal logics, and semiotics. It focuses on the relationship between signifiers—like words, phrases, signs, and symbols—and what they stand for, their denotation.-->
    <H2>SEMANTICS</H2>

    <highlight>
        <d>Semantics</d> is the ascribing of meaning to data.<br>

        <!-- A <d>Form</d> is data with meaning ascribed to it.-->
        <d>Form</d> is a kind of data with some form of interpretation defined over it.<br>
    </highlight>

    Modern programming builds upon several key semantic paradigms:
    Assignment, Structural Addressing, Logical Binding, Lexical Scoping,
    Interpretation, and Procedural Execution.  We define each in turn here.

    Many of these key notions, in turn, rest upon the very foundational and intuitive notion
    of writing data out on an imagined piece of paper.


      <!--ul>In Lex:  <dt>where-what duality</dt>
            <dd>The difference is important:
                Values known what they are, but dont know where they are, whereas places know
                where they are, but not what they are.
            </dd>  </ul-->

    <H3>Assignment Semantics</H3>

     &mdash; The Place, Address, Name, Value Tetrachotomy.

    <highlight>
        <d>Assignment Semantics</d> is widely utilized time-based paradigm of "storing" some data value in a "place"
        such that "retrieval" from that place always returns the data most recently put there.<br>
        - A <d>value</d> is data that can be assigned.<br>
        - A <d>place</d> is where a value can be assigned.<br>
        - An <d>address</d> is a special kind of data value which can be used along with the
          structure of place to find the specfic placed indicated by the address using the structure of place itself.<br>
        - A <d>name</d> a name is logical (non-structural) indicator of place.<br>
        - <d>assignment</d> is the putting of a value into a place.<br>
        - <d>access</d> is the getting of a value from a place.  According to the widely accepted semantics,
          the value returned by accessing a place is the temporally most recent value assigned to that place.
    </highlight>


    All modern programming languages make wide repeated use of assignment semantics &mdash; programmers expect and
    depend on these simple and intuitive semtantics.
    Sadly (without exception??) these languages make a vomitous hash of this simple and elegant semantic.  None
    decomplect value/place/name/address in internally consistent ways &mdash; e.g. a thing will mean a value in one spot and
    will mean the place where the value is, in another spot.  Further they do not uniformly apply the assignment
    operators they do have, but rather generate a multiplicative-cluster-fork collection of operators for doing
    different kinds of assignement acts with similar semantics but in different context.

    Because of this, operations defined on top of assignment cannot be uniformly applied, instead in cluster-forking
    fashion one must build different abstractions that only apply in patchy fashion across the langauge.



    Uniform tries hard to dot the i's can cross the t's here.

    addr  place     value  access          assign
    key   lex       unit   get             set                     Using assignement in DATA STRUCTURE as assignment
    path  lex       unit   value           set_value               Understanding SPACE as assignment
    ref   lex       unit   resolve+value   resolve+set_value       Understanding SEMANTICS as assignment
    form  form+env  unit   eval            assign                  Understanding COMUTATION as assignement



    <ul>
        <dt>universal place value assignment</dt>
        <dd>Popular software languages hoplessly confound these three notions in one bolixed mess.
            Lex retains the usability and expectations of these popular langauges but greatly simplifies the
            universe by keeping these three notions nicely separated and universally applied.
            <br><br>
            In Lex  <code><i>place</i> = <i>value</i></code>, does exactly what you think it should,
            no matter if <i>place</i> is an argument from a fn call, a closure element, an object field,
            the field in a data structure, a file in a directory folder, etc.</dd>
    </ul>


    <H5>Places</H5>
    <highlight>
        Many software systems have a notion of <d>Place</d> (e.g. a variable, or directory folder).
    </highlight>
        In the Lex language:
        <dl>
            <dd>Lexspace is the <i>singular</i> tree of all "places" in the Lex language &mdash;
                the infinite tree of all possible sub-places, indexed recursively by integers and strings.</dd>
            <dt>Lexroot</dt>
            <dd><d>"Lexroot"</d>is the root of Lexspace.</dd>
            <dt>Lex</dt>
            <dd>Lexspace is encode as a tree of <b><i>unit</i></b> just like data is, but
                we refer to unit that are part of the Lexspace tree as <b>Lex</b> to distinguish them
                from data &mdash; still all unit operators app to a Lex just as the apply to data.</dd>
            <dt>UP</dt>
            <dd>Lex unit are distinguished from data units in that they have a <r>UP</r>
                operator which returns it parent lex.</dd>
            <dt>parent / children / ^value </dt>
            <dd>The units of such a bundle are called <d>sub-units</d> or <d>children</d> of the
            <d>"parent"</d> unit which "comtains" them.</dd>
        </dl>


    <H5>Assignment</H5>
    <highlight>
        An <d>assignment</d> is the time-based notion of "storing" some data into a place
        such that "retrieval" from a place always returns the data most recently put there.
    </highlight>
    In Lex:
    <dl>
        <dt>set_value</dt>
            The <d>"set_value"</d> operation clears the infinite subspace below a lex of any values,
            and then places a data unit into that place, and recursively places the (possibly infinite)
            number of sub-unit below it into their corresponding sub-lex.
        <dt>value</dt>
        <dd>The <d>"value"</d> operation returns the data unit last assigned to a lex place,
            which of course will include the possibly infinite subunits under this lex.</dd>
        <dt>access</dt>
        <dd>Collectively we refer to these storing and retrieving actions as <d>"access"</d> both to
            the data unit, or to the Lex.</dd></dl>


    <H3>Addresses &ndash; Structural Semantics</H3>
    <highlight>
        Software systems sometimes have <d>"addresses"</d> &mdash; names that designate places.
    </highlight>
        In the Lex language:
        <dl>
            <dt>path</dt>
            <dd>The address of a Lex place is call is <d>"path"</d>, and it is the
                sequence of keys used to iteratively access that Lex from Lexroot.</dd>
            <dt>".."</dt>
            <dd>Using the special <code><b>".."</b></code> key to denote access "UP" the tree from subLex to
                parent lex, we are able to describe a traversal from any Lex place to any other in lexspace.
                indicate the indexing of a parent unit relative to of its children sub-units.</dd>
            <dt>relative paths</dt>
            <dd>
                There is a unique shortest relative path for all Lex of Lexspace relative to any designated "origin" Lex.
                Thus a paths may be interpreted as an absolute or relative paths, and we may view all addresses in
                lexspace from any one vangage point to any other without information loss.</dd></dl>




    <H5>Lexical</H5>
    <highlight>
        <d>Lexical</d> something apparent in the (1) <i>static</i>, (2) <i>printed</i> presentation of a thing.
    </highlight>
    <dl>
        <dt>Source Form as shown in lexspace</dt>
        <dd>In Lex</dd>
    </dl>


    <H5>Inheritance</H5>
    <highlight>
        <d>Inheritance</d> &mdash; a lexical indicator that one kind of thing implictly has behaviors derived from another kind of thing.
    </highlight>
    <dl>
        <dt><code>import <i>package_name</i></code></dt>
        <dd>In Lex we use the <d>"form"</d> modifier when referring to a
            <em>unit</em> having some expected semantics based on its usage context.
            (For example, we say "code form" when indicating a unit that should be
             understood as being executable code.)</dd></dl>
    <!-- written form, static form=-->




    <H5>Binding</H5>
    <highlight>
        As opposed to time-invarient, location based notion of addressing, a
        <d>BINDING</d> provide "dynamic" and "logical" naming of things.
        Bindings may be dynamic (may vary over time), and are logical (they do not refer to
        the structure or processes needed for access).
    </highlight>
    <dl>
        <dt>binding</dt>
        <dd>In Lex a <d>binding</d> is the association of an alhpanumberic name string with a unit.
            The name can be encoded as string, or a symbol whose arg0 is the same string.</dd></dl>



    <H5>Namespace</H5>
    <highlight>
        <d>NAMESPACE</d> is set of bindings of names to values.
    </highlight>
    <dl>
        <dt>ns</dt>
        <dd>In Lex <code><d>ns</d>(<i>name1</i><d>:</d><i>value1</i>,
                                             <i>name2</i><d>:</d><i>value2</i>, ...)</code>,
            is used to set of name/value bindings as a unit with an "ns" head, and named arguments.</dd></dl>



    <H5>Scoping</H5>
    <highlight>
        <d>LEXICAL SCOPING</d> is the computation of a namespace using the containment
        structure and assigned values "around" the places where scope is being computed.
    </highlight>
    <dl>
        <dt>bindings</dt>
        <dd>In Lex scope is computed by the <code><i>lex</i>.<d>bindings</d>() &rightarrow; <i>scope</i></code>
            operator which returns the scope for a lex given the current values assigned "around" that lex
            within Lexspace.</dd>
        <dt>"classic" scoping</dt>
        <dd>At the root of lexspace bindings are computed using the classic "visiblity" semantics for scoping,
            lower in the tree the bindings operator is bound to other bindings operators, thereby redefining
            both the structure and semantics of those sub-lexspaces.</dd></dl>



    <H5>Reference</H5>
    <highlight>
        Software systems often have multi-step reference mechanisms, these navigate nested/imported/inherited
        namespaces, in order to resolve refernces.
    </highlight>
    In lex much of this comes for free from the structure of Lexspace itself.
    <dl>
        <dt>reference</dt>
        <dd>A <d>"reference"</d> is sequence of name strings, canonically expressed as:
             <code><d>ref</d>(<i>name1</i>, <i>name2</i>, ...)</code></dd>
        <dt>referent resolution</dt>
        <dd>A referent is <d>"resolved"</d> within the Lexical scope of Lex, by</dd>
    </dl>






    <H2>COMPUTATION</H2>




    <H5>Form</H5>
    <highlight>
        <d>Form</d> is a kind of data with some form of interpretation defined over it.<br>
    </highlight>
    <dl>
        <dt>...form</dt>
        <dd>In Lex we use the <d>"form"</d> modifier when referring to a
            <em>unit</em> having some expected semantics based on its usage context.
            (For example, we say "code form" when indicating a unit that should be
             understood as being executable code.)</dd></dl>
    <!-- written form, static form=-->



    <h5>Interpretation</h5>

    <highlight>
        <d>Interpretation</d> is the <i>process</i> of deriving some <i>result</i>,
        from a <i>form</i> and agree upon <i>meanings</i> for that form.
        Either the process itself, or the result is understood to be the <i><q>interpretation</q></i> of the
    </highlight>


    <H5>Execution</H5>
    <highlight>
        - <d>Execution</d> is the time-based process of following the steps outlined in some
          "code" while have access to some "environment", in order to return a result.
        <br><br>
        In essence <d>code</d> is the "written" (static) structured-data which specifies steps
        of an execution which might occur over some interval of time.
        <br><br>
        In essence an execution <d>environment</d> is a dynamic structure where results from
        sub-executions might be assigned and retrieved during execution.
    </highlight>
        In the Lex:
        <ul>
            <li><d>execution</d> is indicated by the <code><d>EX</d>(<i>code</i>, <i>env</i>,
                <i>bindings</i>) &mdash; <i>result</i></code> form.  The result of this execution
                will be a function of code unit as interpreted with the given bindings and given
                execution environment.  By default the EX operator uses the scope derived from the
                place where the code is assigned, and uses the state of the execution environment
                of the execution that it is part of.</li>

            <li><d>code</d> is the term used to indicate a unit value or Lex place that we are
                interpreting as a specification for execution.  Treating a unit as code
                requires that a bindings scope also be supplied, whereas treating a Lex as
                code will use the scope computing by the <em>bindings</em> operator by default.</li>

            <li>In either case referring to an unit or Lex as code is an indication of <i>intent</i>
                since all lex and all data units have a well defined (and potentially meaningful)
                execution behavior.</li>

            <li>In Lex a <d>function</d> is code which executes all of the fixed arguments of
                its "calling" unit in sequential order, and executes all of its named and meta arguments
                in some unspecified order.  But in any case the function has no access to the original
                form of the calling form, only the results of their execution.</li>

            <li>In Lex we a <d>form</d> is an execution which does have direct access to its call form.</li>

            <li>In Lex an <d>operator</d> is any code which makes use of the value bound to the special
                <d>"self"</d> name.  During execution of operator code that operator will "operate on"
                (access and assign) the values assigned the sub-units of that
                The unit bound to self name is the subject of that operators execution.</li>

            <li>In Lex there are three </li>


            <li>Code is said execute <d>over</d> a type of unit in the case that the executionn
                is meaningful when all arguments are instances of the stated type.</li>

        </ul>




    <H4>Elements of Imparative Execution</H4>
    <highlight>
        The following elements are a simplest, natural, covering of <d>Procedural Interpretation</d>:<br>
        - <d>Operations on atomic data</d> (like square root for integers)<br>
        - <d>Operations on composite/collection data</d> (like reverse for lists)<br>
        - <d>Control flow primitives</d> (like an if/then or a while-loop)<br>
        - <d>Creation operations</d> (like heap allocation for an data.)<br>
    </highlight>
    <ul>
        <li>Lexcore expresses all computation as a structure of operators drawn from these four categories.</li>
        <li>The first two categories are embedded into the operators defined over the Unit data structure itself.
            Each atomic type has operators appropriate for that type, and list, map, and tree operators are defined
            over composite Units.</li>
        <li>Control Flow is expressed as a composition of five different primitives that together
            express a simplest complete capture of imparative control flow.</li>
        <li>A sufficient model of creation within imparative programming is captured by three creation primitives. </li>
        <li>NOTE: while the focus of Lexcore is the essential capture of single-threaded, imparative programming,
            it turns out the primitives require for this also seem to provide strong coverage over a much wider
            range of modern software including: functional programming, object oriented programming, stream processing,
            rule based systems, forward/back chaining systems, constain-based programming, eager/lazy evaluation,
            futures, promises, etc.  thus it does seem lexcore is a natural starting point for the essential
            capture of the modern software stack.</li>
    </ul>


    <H5>Creation Operations</H5>
    <highlight>
        The following three primitives provide a simplest, natural, covering of creation within <d>Imparative Execution</d>.<br>
        - <d>Thread Creation.</d>  The creation of computational itself.
        - <d>Heap Allocation.</d>  Creation of data with indefinite extent &mdash; data that exists as long as it is somehow
          reference in (can effect) on going computation.<br>
        - <d>Stack Allocation.</d>  Creation of data with definite extent &mdash; data that exists as the declaring
          form itself is still executing.<br>
    </highlight>
    <ul>
        <li><i>Thread Creation</i> is indicated by the <code><d>RUN</d>(<i>code</i>)</code> operator.  This operator
            is use to begin computation within lexspace itself, and potentially to create asychrnously executing
            sub-threads within lexspace.</li>
        <li><i>Heap Allocation</i> is indicated by the <code><i>unit</i>.<d>NEW</d>(...)</code> operator.  It uses
            <code>var(...)</code> declarations under <i>unit</i> to indicate the structure and types of the data
            allocated by the new operator.</li>
        <li><i>Stack Allocation</i> is indicated by the <code><i>unit</i>.<d>LET</d>(vars...):body</code> operator.  The lets
            immediate sub-units define the structure and types of the data stack allocated.  This data will persist
            and be accessible during the execution of "body".</li>
    </ul>


    <H5>Imparative Control Flow</H5>

    Procedural control dictates the temporal flow of control between computational actions.
    Here we list s simplest covering of procedural control flow:

    <highlight>
        <d>Procedural Control</d> allows for recursive combiation of the following flows:<br>
        - <d> sequencing </d> &mdash; the execution sub-elements of code in temporal order
          based on the code's structure. <br>
        - <d> chaining </d> &mdash; the executing of a sequence of code pieces while the special <d>self</d>
          name bound to result of the prior execution.<br>
        - <d> branching </d> &mdash; the selecting among a finite number of different sub-parts of the
          code to execute next.<br>
        - <d> iterating </d> &mdash; the repeatedly executing of some "body" code once for each element in
          some "iteration" set.<br>
        - <d> terminating </d> &mdash; the stopping an execution before its completion and non-local transfer
          of control to the point in the execution that would have followed the completed execution.
    </highlight>
        In Lexcore:
        <ul>
            <li><i>Sequencing</i> is indicated by the <code><d>BLK</d>(<i>code1</i>, <i>code2</i>, ...)</code>
                operator.  The "BLK" (block) operator executes each of form its fixed argument in sequential order.</li>
            <li><i>Chaining</i> is indicated by the <code><d>CHN</d>(<i>form1</i>, <i>form2</i>,
                ...)</code> operator.  The "CHN" (chaining) operator is executed stack allocating
                an environment for each chained form, resolving the chained operator using the
                curent <code>__bindings__</code> and assigning the special <d>self</d> symbol in each stack
                allocated environment.  </li>
            <li><i>Branching</i> is indicated by the <code><d>BRA</d>(<i>[self]</i>, <i>case1</i>, <i>case2</i>, ...)</code>
                operator, where each case is encoded as: <code><d>if</d>(<i>condition</i>, <i>action</i>)</code>.
                The "BRA" (branch) operator executes the conditional expressions in sequential order,
                stopping on the first <i>satisfied</i> condition, then executes that case's action expression.</li>
            <li><em>Iterating</em> is indicated by the <code><d>ITR</d>(<i>[subject]</i>,
                <i>[value_ref]</i>, <i>[key_ref]</i>, <i>[body]</i>)</code> Operator.  An "ITR" iteration is executed
                executed once for each sub-unit of <i>subject</i> (or forever if no subject is provided).  Both
                <i>key_ref</i> and <i>value_ref</i> are assigned for each sub-element if they are provided, and
                <i>body</i> is executed once for each sub-element if it is provided.</li>
            <li><em>Terminating</em> is indicated by the <code><d>RET</d>(<i>return_form</i>, [<i>return_tag</i>])</code>.
                The <code>TRY(<i>args...</i>, <i>__finally__</i>:form):<i>body</i></code> operator provides a named
                target for termination.  RETurning to this target (or natural)


                ....
                and the
                and
                <code><d>catch</d>(<i>catch_critera</i>, <d>body:</d><i>body_code</i>)</code> forms.
                When an execution reaches an "exit" forn in the code, that execution stops and the
                <i>return_form</i> is executed in its place.  When execution of <i>return_form</i> and
                complete, it is used to a the result returned by the
                most recently executed <i>catch</i> form whose <i>catch_critera</i>
                is satisfied when invoked on the exit's <i>return_tag</i>.</li>
        </ul>


    idempotent -- constant
    executable -- causes execution




<H4>COMMON COMPUTATIONAL TYPES</H4>




    <H5>Container Types</H5>
    <highlight>
        In essence a <d>YYY</d> is
    </highlight>
        In Lex
    <ul>
        <dt>set / list / tuple / multiset / map / </dt>
        <dd></dd>
        <dt></dt>
        <dd></dd>
    </ul>
    <!-- we do not have generalized maps -->


    <H5>XXX</H5>
    <highlight>
        In essence a <d>YYY</d> is
    </highlight>
        In Lex
    <ul>
        <dt></dt>
        <dd></dd>
        <dt></dt>
        <dd></dd>
    </ul>



    <H5></H5>
    <highlight>
        A <d>predicate</d> or <d>critera</d> is code whose execution returns
        true or false depending whether the predicate or criteria is "satisfied".
    </highlight>
        In Lex we use the terms predicate and critera interchangeably, they both
        refer to an operator (without arguments) except the expect <d>self</d> bind
        which returns either the <code>sym("true")</code> or <code>sym("false")</code> symbols.
        <br><br>
        In Lex we say that a predicate or critera is <d>"over"</d> some type, this means
        that is execution is only meaningful when <d>self</d> is bound to an instance of that type.


    <H5></H5>
    <highlight>
        An <d>n-ary</d> execution is one that accepts N inputs.
    </highlight>
        In Lex an <d>n-ary function</d> is a function with n fixed argument, an <d>n-ary form</d> is a form
        that accepts n fixed arguments, and an <d>n-ary operator</d> is an operator that executes over
        the <d>self</d> binding, and <code>n - 1</code> fixed arguments.


    <H5>Ordering</H5>
    <highlight>
        An <d>ordering</d> for a set of things is some specification of which of those things come "before"
        others according to the ordering.
    </highlight>
        In Lex an <d>ordering</d> is specified by a binary predicate over the data type being ordered.<br>
        <ol>
            <li>This could be code that is such a binary function.</li>
            <li>A unit whose value associated with its <code>before</code> key is such a predicate, or</li>
            <li>An operator directly associated with the type being ordered &mdash; i.e. it is
                bound to the  <code>before</code> name in the scope used by instances of the ordering type. </li>
        </ol>

    <H2>TYPING</H2>

    SYSTEMATICALLY ARRANGED DATA




    <H5>Data Type</H5>
    <highlight>
        A software <dq>package</dq> is a source code grouping of elements that each define some aspect of execution semantics.
    </highlight>

    <highlight>
        In software a <dq>type</dq> is an expection of data being structured in a particular way.
    </highlight>
        Software execution operates over code with a fixed structure, thus execution necessarily follows repeating themes,
        So it is important for the data operated on to also follow parallel themes.  Declaration and use of types
        is software's way of ensuring that those expectations are met.
    <dl>
        <dt>type</dt>
        <dd>In Lex a <d>"type"</d> is a specifier for a subset of all uniform experessions that
            satisfy the requirements of the type.  Thus any predicate over <em>unit</em> is a type in Lex.</dd>
        <dt>instance of</dt>
        <dd>A data unit is an <d>instance of</d> if the type predicate returns true for that unit.</dd>
        <dt>named type</dt>
        <dd>A <d>"named type"</d> is some scope is simply a type predicate which is bound under the
            <code><d>"is"</d></code> namespace for that scope.  For example, if "person" were a
            type defined within scope, then <code>is.person</code> would be bound the the
            person type-checking predicate such that <code>x.is.person()</code> would return true the case
            that x was bound to a unit satisfying the person type predicate.</dd>
        <dt>validity</dt>
        <dd>In Lex the head of a unit specifies the type it claims to be,
            So the unit <code><d>person</d>(...)</code> is indicating that it is of type person.
            A unit is said to be <d>valid</d> within the given scope in the case that the unit's
            head binds to a type operator that returns true when executed on that unit.
            <d>validity</d> is a central conformance spec for Lex.  Asserting that a
            structure is valid within a some scope means that the validity conformance spec
            returns true when applied to that structure in that scope.  Intuitively validity
            means that a structure is conforming to the constraints indicated
            heads values contained within that structure.</dd>
    </dl>



    <H5>Templates</H5>

    <highlight>
        A <d>template</d> is a structure that may contain placeholders (<q>template variables</q>) that structure.<br>
        - <d>instantiation</d> &ndash; Given variable bindings a template can be <i>instantiated</i> to produce a
            structure parallel to that of the template itself with placeholders filled in as appropriate.<br>
        - <d>matching</d> &ndash; Given a structure a template can be <i>matched</i> against it, potentially producing variable bindings.
    </highlight>

    <dl>
        <dt>form</dt>
        <dd>In Lex, templating is indicated by <code><d>form</d>(<i>head</i>, <i>arg0</i>, <i>arg1</i>...)</code>.<br>
        By default the structure of the form is recursively copied and returned, except for the special
        template variable indicators in the tree:  <code><d>v</d>(<i>code</i>, ...)</code>, those are executed
        and their results (not themselves) are inserted into the growing template instance (see XXXX for details)</dd></dl>


    <H5>Linking</H5>
    <highlight>
        <d>Linking</d> combines two or more Forms with defined semaantics into a resulting <q>linked</q> form with
        a semantics spanning the entire result.<br>
        <d>Resolving</d> a link between Forms means selecting one or another binding for each resolved link.
    </highlight>




    <!------------------------------------------------------------------------------------------------------------->
    <h2>THE "SPEC" SUPER PATTERN</h2>


    <!--
    <H5>Declarative verses Imparative</H5>
    <highlight>
      - The difference between <d>declarative</d> versus <d>imparative</d> forms are how they are understood.<br>
      - Declarative form can be understood somehow by inspection of the static form itself. <br>
      - Imparative forms by contrast only understood by considering their exeuction.
    </highlight>
    <dl>
        <dt>...spec</dt>
        <dd>In Lex we append "...spec" suffix to indicate forms that can be understood declaratively, by inspecction of the
            form itself along with the surrounding static lexical structure.
        </dd>
    </dl>  -->




    <H5></H5>
    <highlight>
        In essence a <d>PRIORITIZATION</d> is a dynamic ordering of things.
    </highlight>
        In Lex






    In essence a <d>CONFORMANCE</d> is a "soft spec" &mdash; a critera which might be applied to and
    possibly satisfied by a thing.

    <p>
        An 80/20-capture of conformance in Lex is a <i>typespec</i> having a <code><d>modifies:</d><i>type</i></code>
        key.  Whereas other typespecs define the nature of a thing, these conformance define a property
        of that type of thing.  The difference is mostly one of interpretation, when a thing fails its
        typespec, it ceases to be interpretable <i>as</i> that kind of thing, with a conformance the thing
        retains its nature, it just no longer conforms to
    </p>






    <h2>Advanced Computational Forms</h2>



    In essence a <d>RULE</d> is thing with a condition and an action, with the idea that
    the <i>action</i> can/should/might/must be executed when the <i>condition</i> is satisfied.
    <p>
        An 80/20-capture of this notion in Lex is the when clause,
        <code><d>when</d>(<i>condition_predicate</i>, <i>action_code</i>)</code>
    </p>

    In essence a <d>REWRITE SYSTEM</d> is a process accepting an input specimine and transformation
    rules.  The rewrite system iteratively updates the input specimin by executing transformation rules
    until some stopping critera is met, and the final version of the specimin is returned.
    <p>
        An 80/20-capture of this notion in lex is
        <code><d>expand</d>(<i>input_unit</i>, [<d>rules:</d><i>pkg</i>]) &rightarrow; <i>output_unit</i></code> &mdash;
        The rules pkg must specify <i>rules</i>, an <i>ordering</i> which applies to the rules,
        and a stopping <i>criteria</i> which applies to the state of the rewrite system itself.


    In essence <d>COMPLATION</d> is a rewrite system that operates on code, where all transformations
    preserve essential aspects of the code's computation while attempting to produce versions that
    compute faster on some targeted hardware.
    <p>
        An 80/20 capture of this in Lex the already defined <code><d>expand</d>(<i>code</i>)</code> function
        executed without arguments.  It gathers compilation rules, ordering, and stopping critera
        from the current environment to compile the passed code.
    </p>


    <H2>Key Semantics</H2>

    <H3>HEADS</H3>
    <dl>
        <dt>fixed</dt>
        <dt>named</dt>

    </dl>

    <H2>KEYS</H2>
    <dl>
        <dt>key</dt>

    </dl>





    <H5></H5>
    <highlight>
        In essence a <d>PRIORITIZATION</d> is a dynamic ordering of things.
    </highlight>
        In Lex



    <H5></H5>
    <highlight>
        In essence a <d>PRIORITIZATION</d> is a dynamic ordering of things.
    </highlight>
        In Lex


    <h1>quick test H1</h1>
    <h2>quick test H2</h2>
    <h3>quick test H3</h3>
    <h4>quick test H4</h4>
    <h5>quick test H5</h5>

    here is some body text
    <ul>
        <li>first thing<br>part two</li>
        <li>second thing</li>
        <li>third thing</li>
    </ul>
    here is some body text
    <ol>
        <li>first thing<br>part two</li>
        <li>second thing</li>
        <li>third thing</li>
    </ol>
    here is some body text
    <dl>
        <dd>first thing<br>part two</dd>
        <dd>second thing</dd>
        <dd>third thing</dd>
    </dl>



    <H5></H5>
    <highlight>
        In a <d>HIGHLIGHT</d> is a dynamic ordering of things.
    </highlight>
        In Lex


    <h1>OLDER STUFF -- Advanced Computational Forms</h1>

    <H2>UNIFORM/LEX OPERVIEW</H2>

    Just as JSON is a simplest, practical, sufficient-enough model of data for machine collaboration and interchange,
    Uniform aspires to be a simplest, practical, sufficient-enough model of data for human collaboration and interchange.  And
    Lex aspires to be a simplest, practical, sufficient-enough model of computation for collaboration and interchange.

    <p>
        Modern computation involves many more ideas, thus longer spec than JSON.
        Still here we aspire to the same bare-bones simplicity for each concept.
    </p>



    <highlight>
        <d>DATA</d> is JSON-ish:  Recursively structured forms, whose sub-forms are indexed by position or
        by name, and where the atomic units are numbers, strings.
    </highlight>
    <ul>
        <dt>Unit Form</dt>
        <dd>Expresses structured data as a tree of "<i>UNITS</i>".  Unit form has is bi-directionally mappable with JSON,
            and can be expressed textually as:  HEAD(pos_0, pos_1, pos_2, ..., key1=val1, ...).</dd>
        <dt>Uniform</dt>
        <dd></dd>
    </ul>



    <highlight>
        <d>COMPUTATION</d> is is the recursive process of following steps outlined in <d>CODE</d> in
        order to succeively update computation state-data towards some useful end. <br>
        <d>CODE</d> is executable-data, that is also expressible textually in ways that make the consequences
        of execution beautifully aparent to human viewers of its textual form.
        <d>INFORMATION</d> is execution-state-data, it encodes computation's input, processing, or output data,
        and information also has a textual form which makes the meaning of the execution data beautifully
        aparent to the human observer.
    </highlight>
        Lex uses Uniform as its human-beautiful textual representation its unit-data.  Society has been textually
        encoding information and code for mellinea and centuries respectively.  Thus Uniform attempts an
        80/20-capture of the great wealth of practices there.
    <ul>
        <dt>Recursion is captured spatially and via nested bracing.</dt>
        <dd>Standard notation for atomic types is used</dd>
        <dt>Standard infix mathematical operators are used, with fixed "standard" operator precedence.  </dt>
        <dd></dd>
    </ul>




    In essence a <d>RULE</d> is thing with a condition and an action, with the idea that
    the <i>action</i> can/should/might/must be executed when the <i>condition</i> is satisfied.
    <p>
        An 80/20-capture of this notion in Lex is the when clause,
        <code><d>when</d>(<i>condition_predicate</i>, <i>action_code</i>)</code>
    </p>

    In essence a <d>REWRITE SYSTEM</d> is a process accepting an input specimine and transformation
    rules.  The rewrite system iteratively updates the input specimin by executing transformation rules
    until some stopping critera is met, and the final version of the specimin is returned.
    <p>
        An 80/20-capture of this notion in lex is
        <code><d>expand</d>(<i>input_unit</i>, [<d>rules:</d><i>pkg</i>]) &rightarrow; <i>output_unit</i></code> &mdash;
        The rules pkg must specify <i>rules</i>, an <i>ordering</i> which applies to the rules,
        and a stopping <i>criteria</i> which applies to the state of the rewrite system itself.


    <H5>Form</H5>
    <highlight>
        <d>Semantics</d> is the ascribing of meaning to data.
    </highlight>
    <dl>
        <dt>...form</dt>
        <dd>In Lex we use the <d>"form"</d> modifier when referring to a
            <em>unit</em> having an expected semantics based on its usage context.
            (For example, we say "code form" when indicating a unit that should be
             understood as being executable code.)</dd></dl>
    <!-- written form, static form=-->


    <h2>OTHER STUFF</h2>

    <highlight>
        An <strong>N-putation</strong> is an n-way computation where any one of the variables can be computed from the others.
    </highlight>

    <footer>
    Copyright (c)  Daniel Oblinger.  All rights reserved.
    </footer>


</div>
</body>
</html>


