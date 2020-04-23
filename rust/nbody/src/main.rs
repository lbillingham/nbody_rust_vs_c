pub mod vec3;

// #[derive(Clone, Debug)]
// struct Body {
//     position: vec3::Vec3, //[f64; 3],
//     velocity: vec3::Vec3, //[f64; 3],
//     mass: f64,
// }

// const BODIES_COUNT: usize = 5;
// const TIMESTEP: f64 = 0.01; // in days... right??

// const SOLAR_MASS: f64 = 4. * std::f64::consts::PI * std::f64::consts::PI;
// const DAYS_PER_YEAR: f64 = 365.24;

// const INTERACTIONS: usize = (BODIES_COUNT * (BODIES_COUNT - 1)) / 2;
// // const ROUNDED_INTERACTIONS_COUNT: usize = INTERACTIONS_COUNT + (INTERACTIONS_COUNT % 2);
// // const TIMESTEP: f64 = 0.01; // in days... right??

// /// Initial state of the simulation.
// const STARTING_STATE: [Body; BODIES_COUNT] = [
//     // Sun
//     Body {
//         mass: SOLAR_MASS,
//         position: vec3::Vec3 {
//             x: 0.,
//             y: 0.,
//             z: 0.,
//         },
//         velocity: vec3::Vec3 {
//             x: 0.,
//             y: 0.,
//             z: 0.,
//         },
//     },
//     // Jupiter
//     Body {
//         position: vec3::Vec3 {
//             x: 4.841_431_442_464_72e0,
//             y: -1.160_320_044_027_428_4e0,
//             z: -1.036_220_444_711_231_1e-1,
//         },
//         velocity: vec3::Vec3 {
//             x: 1.660_076_642_744_037e-3 * DAYS_PER_YEAR,
//             y: 7.699_011_184_197_404e-3 * DAYS_PER_YEAR,
//             z: -6.904_600_169_720_63e-5 * DAYS_PER_YEAR,
//         },
//         mass: 9.547_919_384_243_266e-4 * SOLAR_MASS,
//     },
//     // Saturn
//     Body {
//         position: vec3::Vec3 {
//             x: 8.343_366_718_244_58e0,
//             y: 4.124_798_564_124_305e0,
//             z: -4.035_234_171_143_214e-1,
//         },
//         velocity: vec3::Vec3 {
//             x: -2.767_425_107_268_624e-3 * DAYS_PER_YEAR,
//             y: 4.998_528_012_349_172e-3 * DAYS_PER_YEAR,
//             z: 2.304_172_975_737_639_3e-5 * DAYS_PER_YEAR,
//         },
//         mass: 2.858_859_806_661_308e-4 * SOLAR_MASS,
//     },
//     // Uranus
//     Body {
//         position: vec3::Vec3 {
//             x: 1.289_436_956_213_913_1e1,
//             y: -1.511_115_140_169_863_1e1,
//             z: -2.233_075_788_926_557_3e-1,
//         },
//         velocity: vec3::Vec3 {
//             x: 2.964_601_375_647_616e-3 * DAYS_PER_YEAR,
//             y: 2.378_471_739_594_809_5e-3 * DAYS_PER_YEAR,
//             z: -2.965_895_685_402_375_6e-5 * DAYS_PER_YEAR,
//         },
//         mass: 4.366_244_043_351_563e-5 * SOLAR_MASS,
//     },
//     // Neptune
//     Body {
//         position: vec3::Vec3 {
//             x: 1.537_969_711_485_091_1e1,
//             y: -2.591_931_460_998_796_4e1,
//             z: 1.792_587_729_503_711_8e-1,
//         },
//         velocity: vec3::Vec3 {
//             x: 2.680_677_724_903_893_2e-3 * DAYS_PER_YEAR,
//             y: 1.628_241_700_382_423e-3 * DAYS_PER_YEAR,
//             z: -9.515_922_545_197_159e-5 * DAYS_PER_YEAR,
//         },
//         mass: 5.151_389_020_466_114_5e-5 * SOLAR_MASS,
//     },
// ];

// fn sqr(x: f64) -> f64 {
//     x * x
// }

// fn offset_momentum(bodies: &mut [Body; BODIES_COUNT]) {
//     let (sun, planets) = bodies.split_first_mut().unwrap();
//     sun.velocity = vec3::Vec3::default();
//     for planet in planets {
//         for m in 0..3 {
//             sun.velocity[m] -= planet.velocity[m] * planet.mass / SOLAR_MASS;
//         }
//     }
// }

// fn output_energy(bodies: &mut [Body; BODIES_COUNT]) {
//     let mut energy = 0.;

//     // for (i, body) in bodies.iter().enumerate() {
//     //     // kinetic energy of each body
//     //     energy += 0.5 * body.velocity.dot(body.velocity);
//     //     // potential energy between pairs of bodies
//     //     for body2 in &bodies[i + 1..BODIES_COUNT] {
//     //         let displacement = body.position - body2.position;
//     //         energy -= body.mass * body2.mass / f64::sqrt(displacement.dot(displacement))
//     //     }
//     // }
//     // Output the total energy of the system.
//     println!("{:.9}", energy);
// }

// // Steps the simultion forward by 1 timestep
// fn advance(bodies: &mut [Body; BODIES_COUNT]) {
//     // Calc position differences between each interacting pody pair
//     let mut position_deltas = [vec3::Vec3::default(); INTERACTIONS];
//     {
//         let mut k = 0;
//         for i in 0..BODIES_COUNT - 1 {
//             for j in i + 1..BODIES_COUNT {
//                 position_deltas[k] = bodies[i].position - bodies[j].position;
//                 // for (m, pd) in position_deltas[k].iter_mut().enumerate() {
//                 //     *pd = bodies[i].position[m] - bodies[j].position[m];
//                 // }
//                 k += 1;
//             }
//         }
//     }
//     // Calc force magnitudes between each bodies
//     let magnitudes = {
//         let mut magitudes = [0.; INTERACTIONS];
//         for (i, mag) in magitudes.iter_mut().enumerate() {
//             let distance_squared = sqr(position_deltas[i][2])
//                 + sqr(position_deltas[i][1])
//                 + sqr(position_deltas[i][0]);
//             *mag = TIMESTEP / (distance_squared * distance_squared.sqrt());
//         }
//         magitudes
//     };

//     // Update the  bodies' velocities
//     {
//         let mut k = 0;
//         for i in 0..BODIES_COUNT - 1 {
//             for j in i + 1..BODIES_COUNT {
//                 let i_mass_magnitude = bodies[i].mass * magnitudes[k];
//                 let j_mass_magnitude = bodies[j].mass * magnitudes[k];
//                 // for (m, pd) in position_deltas[k].iter().enumerate() {
//                 //     bodies[i].velocity[m] -= *pd * j_mass_magnitude;
//                 //     bodies[j].velocity[m] += *pd * i_mass_magnitude;
//                 // }
//                 k += 1;
//             }
//         }
//     }

//     // Update the  bodies' positions
//     for body in bodies {
//         // for (m, pos) in body.position.iter_mut().enumerate() {
//         //     *pos += TIMESTEP * body.velocity[m]
//         // }

//         body.position += body.velocity.scale(TIMESTEP)
//     }
// }

// fn main() {
//     let num_steps = std::env::args().nth(1).unwrap().parse().unwrap();

//     let mut solar_bodies = STARTING_STATE;

//     offset_momentum(&mut solar_bodies);
//     print!("initial energy: ");
//     output_energy(&mut solar_bodies);
//     for _ in 0..num_steps {
//         advance(&mut solar_bodies)
//     }
//     print!("Energy after {} steps: ", num_steps);
//     output_energy(&mut solar_bodies);
// }
