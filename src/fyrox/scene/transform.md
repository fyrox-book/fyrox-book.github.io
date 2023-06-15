# Transformation

Transformation (transform for short) - is a special entity that changes coordinate system from one to another. It is used
primarily in scene nodes to store their position/rotation/scale/pivots/etc. Fyrox has quite complex transformations, that
supports:

1) Position (`T`)
2) Rotation (`R`)
3) Scale (`S`)
4) Pre-rotation (`Rpre`) 
5) Post-rotation (`Rpost`) 
6) Rotation Pivot (`Rp`)
7) Rotation Offset (`Roff`)
8) Scaling Offset (`Soff`)
9) Scaling Pivot (`Sp`)

Final transformation matrix will be `Transform = T * Roff * Rp * Rpre * R * Rpost * Rp⁻¹ * Soff * Sp * S * Sp⁻¹`. In 99.9%
cases first three are enough for pretty much every task. Other six components used for specific stuff (mainly for nodes
that imported from FBX file format).