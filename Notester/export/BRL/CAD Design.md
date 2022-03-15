# BRL.CAD Design --


    CAD DESIGN


    INPUTS:
    - Set of CAD component object specifications.    (Things that go into; mate with, are packed with, or somehow interact with product.)
    - Set of constraints between the CAD objects.
    - Set of goals on whole product.
    OUTPUT:
    - CAD model of the designed object along with explanation of how constraints and objectives are met.

    REPRESENTATION LADDER:
    - New constraint/goal vocabulary is learned and rooted in known (computable) vocabulary
    - New design strategies are acquired by example, or by instantiating known generic strategies



    Examples of constraints/objectives:
    - Use as little plastic as possible.
    - All user controls must be on at most 2 faces of the final product
    - The unit's tipping angle must be higher than 30 degrees
    - Three of the components generate heat and must be mounted on the outside surface
    - An air must flow of 30 cm per second must be maintained around one of the components
    - Sound from the fan must be no higher than 45 db outside the enclosure
    - All components must be able to be inserted and removed from final product
    - Product will fit into a car's glove box


    Relevant spatial representations:
    - 3D-polyhedria + gemometric operators
    - CAD models
    - Blob models
    - Topological models
    - tuples + types + inferences rules
    - any needed math operators

