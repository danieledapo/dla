
// POV-Ray 3.6 / 3.7 Scene File "povpawn.pov"
// author: Friedrich A. Lohmueller, Aug-2005/Jan-2011
// email:  Friedrich.Lohmueller_at_t-online.de
// homepage: http://www.f-lohmueller.de
//
#version 3.6; // 3.7;
global_settings{ assumed_gamma 1.0 }
#default{ finish{ ambient 0.1 diffuse 0.9 }}

#include "colors.inc"
#include "textures.inc"
// camera ------------------------------------------------------------------
#declare Cam0 =camera {ultra_wide_angle angle 45
                                        location  <0.0 , 0.6 ,-3.0>
                                        right     x*image_width/image_height
                                        look_at   <0.0 , 0.6 , 0.0>}
#declare Cam1 =camera {ultra_wide_angle angle 90
                                        location  <1.2 , 0.3 ,-0.9>
                                        right     x*image_width/image_height
                                        look_at   <0.0 , 0.7 , 0.5>}
camera{Cam0}                                                     //<1
// sun ---------------------------------------------------------------------
light_source{<1500,2500,-2500> color White}
// sky ---------------------------------------------------------------------
sphere{<0,0,0>,1 hollow
             texture{pigment{gradient <0,1,0>
                             color_map{[0 color White]
                                       [1 color Blue  ]}
		             quick_color White }
	             finish {ambient 1 diffuse 0} }
              scale 10000}
// ground------------------------------------------------------------------
plane{ <0,1,0>, 0
       texture{ pigment { color rgb <0.80,0.55,0.35>*1.1}
                normal  { bumps 0.75 scale 0.035  }
                finish  { phong 0.1 }
              } // end of texture
     } // end of plane
//==========================================================================
union{                                                                 //<2
sphere{<0,1,0>,0.35}
cone   {<0,0,0>,0.5,<0,1,0>,0.0}
texture {pigment{ color rgb<1,0.65,0>}
         finish { phong 0.5}} }
//------------------------------------------------------------- end ------------------------------------

/*
//Fuer das 2. Bild                                    - For the 2nd image
//aendert man die Zeile mit der Marke //<1 wie folgt: - change line marked //<1 as follows:
camera {Cam1}

//und die Zeilen ab der Marke //<2 wie folgt:         - change lines starting from mark //<2 as follows:
//------------------------------------------------------------------------------------------------------
#declare Pawn = union{     //[pawn = Bauer(Schachfigur)]
sphere{<0,1,0>,0.35}
cone   {<0,0,0>,0.5,<0,1,0>,0.0}
texture {pigment{ color rgb<1,0.65,0>}
         finish { phong 0.5}}}
//-----------------------------------------------------------------
union{
object{ Pawn translate < 0.0, 0.0, 0.0>}
object{ Pawn translate < 0.0, 0.0, 1.2>}
object{ Pawn translate < 0.0, 0.0, 2.4>}
rotate<0,0,0> translate<0,0.7,0>}
*/
//------------------------------------------------------------- end -----------------------------------
