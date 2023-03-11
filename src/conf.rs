use crate::weftui::define_conf;
//note, when you've tweaked the dynamic version of these to complete satisfaction, just write a procedure that generates code like this, you really don't have to manually copy things over
define_conf!{
    Conf,
    look_sensitivity: {f32, "in radians", 1.0},
    move_speed: {f32, "in meters per second, obviously", 1.7},
    gravity: {f32, "mpss" , 9.8},
    jump_height: {f32, "during a jump, the peak distance reached between the player's feet and the ground", 1.4},
    jump_dur: {f32, "how long acceleration in a jump is going on", 1.4},
    mantle_max: {f32, "the max height that can be mantled", 1.2},
    mantle_dur: {f32, "the amount of time the max mantle takes", 0.3},
    flight_friction: {f32, "friction in mpss when flying", 10.0},
    flight_acc: {f32, "acc mpss when flying", 30.0},
    standing_height: {f32, "player height when standing", 1.9},
    standing_eye_level: {f32, "eye level when standing", 1.82},
    prone_height: {f32, "height when fully prone", 0.8},
    crouch_dur: {f32, "time it takes to crouch", 0.25},
    prone_eye_level: {f32, "", 0.5},
    prone_speed: {f32, "", 0.7},
    mantling_speed_min: {f32, "the speed you move at when you're at the very beginning of the highest intensity mantle", 0.2},
    hold_dur: {f32, "", 0.3},
    max_beam_length: { f32, "The maximum length of the beam for drawing tiles or materials. If you're pointing at a surface further away than that, it will just appear at this distance", 5.0,},
    line_height: {f32, "The height of a line of text defined in px", 13.0},
    thumb_width: {f32, "The span a clickable element wants in px", 34.0},
}