#[cfg(test)]
mod tests {
    use cgmath::{Vector3, Vector4};

    use crate::{
        extensions::matrix_extension::{MatrixExtension, Purifiable},
        init_gl,
        objects::{
            active_vorticies,
            boundary_vorticies::{self, BoundaryVorticies},
        },
        structures::vortex::Vortex,
        support::magnitude_statistics::MagnitudeStatistics,
    };

    use all_asserts::assert_le;

    #[test]
    fn edge_case() {
        let value = 1000000000.0f32;
        let vorticies = vec![
            Vortex {
                position: Vector3::new(-1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                vorticity: Vector3::new(0f32, -value, 0.0f32).extend(1.0f32),
                ..Default::default()
            },
            Vortex {
                position: Vector3::new(1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                vorticity: Vector3::new(0.0f32, -value, 0.0f32).extend(1.0f32),
                ..Default::default()
            },
        ];
        let active_vorticies = vec![Vortex {
            position: Vector4::new(0., 0., 0., 1.),
            vorticity: Vector4::new(0., value, 0., 1.),
            lifetime: Vector4::new(100., 100., 0., 1.),
            ..Default::default()
        }];

        let cpu_errors =
            BoundaryVorticies::create_error_vector(&vorticies, &active_vorticies, true);

        let stats = MagnitudeStatistics::from_vectors(&cpu_errors);

        println!("Stats: {:?}", stats);

        assert_le!(stats.max, 0.0001f32);
    }

    #[test]
    fn it_works() {
        let boundary_vorticies = vec![
            Vortex {
                position: Vector3::new(-1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                vorticity: Vector3::new(0f32, -1.0f32, 0.0f32).extend(1.0f32),
                ..Default::default()
            },
            Vortex {
                position: Vector3::new(1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                vorticity: Vector3::new(1.0f32, 1.0f32, 1.0f32).extend(1.0f32),
                ..Default::default()
            },
            // Vortex {
            //     position: Vector3::new(3.0f32, 4.0f32, 2.0f32).extend(1.0f32),
            //     vorticity: Vector3::new(1.0f32, 1.0f32, 1.0f32).extend(1.0f32),
            //     ..Default::default()
            // },
            // Vortex {
            //     position: Vector3::new(5.0f32, 6.0f32, 3.0f32).extend(1.0f32),
            //     vorticity: Vector3::new(1.0f32, 1.0f32, 1.0f32).extend(1.0f32),
            //     ..Default::default()
            // },
        ];
        let active_vorticies = vec![
            Vortex {
                position: Vector4::new(0., 0., 0., 1.),
                vorticity: Vector4::new(0., 1., 0., 1.),
                lifetime: Vector4::new(100., 100., 0., 1.),
                ..Default::default()
            },
            Vortex {
                position: Vector4::new(0.5, 0.5, 0., 1.),
                vorticity: Vector4::new(1., 1., 0., 1.),
                lifetime: Vector4::new(100., 100., 0., 1.),
                ..Default::default()
            },
        ];

        let (cpu_errors_before, cpu_errors_after) =
            calculate_cpu(boundary_vorticies.clone(), active_vorticies.clone());
        println!();
        println!("---------------");
        println!();
        let (gpu_errors_before, gpu_errors_after) =
            calculate_gpu(boundary_vorticies, active_vorticies);
        println!();
        println!("---------------");
        println!();

        for (i, (cpu, gpu)) in cpu_errors_before
            .iter()
            .zip(gpu_errors_before.iter())
            .enumerate()
        {
            println!("{}: {:?} {:?}", i, cpu, gpu);
        }

        println!();

        for (i, (cpu, gpu)) in cpu_errors_after
            .iter()
            .zip(gpu_errors_after.iter())
            .enumerate()
        {
            println!("{}: {:?} {:?}", i, cpu, gpu);
        }
    }

    fn calculate_gpu(
        boundary_vorticies: Vec<Vortex>,
        active_vorticies: Vec<Vortex>,
    ) -> (Vec<Vector3<f32>>, Vec<Vector3<f32>>) {
        let (_glfw, _window, _events) = init_gl();

        let mut boundary_vorticies =
            boundary_vorticies::BoundaryVorticies::from_vorticies(vec![boundary_vorticies]);
        let _active_vorticies =
            active_vorticies::ActiveVorticies::new(active_vorticies, 0., 0., 0., 0., 0);
        let dt = 0.1f32;
        boundary_vorticies.step_errors(dt);
        let errors_before = boundary_vorticies.get_errors();
        boundary_vorticies.step_correction(dt);
        boundary_vorticies.step_errors(dt);
        let errors_after = boundary_vorticies.get_errors();
        let stats = MagnitudeStatistics::from_vectors(&errors_after);
        println!("Stats: {:?}", stats);

        assert_le!(stats.max, 0.0001f32);

        (errors_before, errors_after)
    }

    fn calculate_cpu(
        boundary_vorticies: Vec<Vortex>,
        active_vorticies: Vec<Vortex>,
    ) -> (Vec<Vector3<f32>>, Vec<Vector3<f32>>) {
        let matrix = BoundaryVorticies::create_matrix(&boundary_vorticies)
            .pseudo_inverse(0.0001f32)
            .unwrap();
        // .purify();
        matrix.printstd();
        let errors_before =
            BoundaryVorticies::create_error_vector(&boundary_vorticies, &active_vorticies, false);

        let corrections = BoundaryVorticies::calculate_corrections(&matrix, &errors_before);

        let errors_after = BoundaryVorticies::create_error_vector(
            &boundary_vorticies
                .iter()
                .enumerate()
                .map(|(i, v)| Vortex {
                    vorticity: corrections[i].extend(1.0f32),
                    ..v.clone()
                })
                .collect::<Vec<_>>(),
            &active_vorticies,
            true,
        );

        let stats = MagnitudeStatistics::from_vectors(&errors_after);
        println!("Corrections: {:?}", corrections);
        println!("Stats: {:?}", stats);

        assert_le!(stats.max, 0.0001f32);

        (errors_before, errors_after)
    }

    #[test]
    fn test_cpu() {
        let boundary_vorticies = vec![
            Vortex {
                position: Vector3::new(-1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                vorticity: Vector3::new(0f32, -1.0f32, 0.0f32).extend(1.0f32),
                normal: Vector3::new(2.0f32, 0.0f32, 1.0f32).extend(1.0f32),
                ..Default::default()
            },
            Vortex {
                position: Vector3::new(1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                vorticity: Vector3::new(1.0f32, 1.0f32, 1.0f32).extend(1.0f32),
                normal: Vector3::new(1.0f32, 0.0f32, 0.0f32).extend(1.0f32),
                ..Default::default()
            },
            // Vortex {
            //     position: Vector3::new(3.0f32, 4.0f32, 2.0f32).extend(1.0f32),
            //     vorticity: Vector3::new(1.0f32, 1.0f32, 1.0f32).extend(1.0f32),
            //     ..Default::default()
            // },
            // Vortex {
            //     position: Vector3::new(5.0f32, 6.0f32, 3.0f32).extend(1.0f32),
            //     vorticity: Vector3::new(1.0f32, 1.0f32, 1.0f32).extend(1.0f32),
            //     ..Default::default()
            // },
        ];
        let active_vorticies = vec![
            Vortex {
                position: Vector4::new(0., 0., 0., 1.),
                vorticity: Vector4::new(0., 1., 0., 1.),
                lifetime: Vector4::new(1., 1., 0., 1.),
                ..Default::default()
            },
            Vortex {
                position: Vector4::new(0.5, 0.5, 0., 1.),
                vorticity: Vector4::new(1., 1., 0., 1.),
                lifetime: Vector4::new(1., 1., 0., 1.),
                ..Default::default()
            },
        ];

        let (cpu_errors_before, cpu_errors_after) =
            calculate_cpu(boundary_vorticies.clone(), active_vorticies.clone());

        let stats = MagnitudeStatistics::from_vectors(&cpu_errors_after);
        println!("Stats: {:?}", stats);

        assert_le!(stats.max, 0.0001f32);
    }
}
