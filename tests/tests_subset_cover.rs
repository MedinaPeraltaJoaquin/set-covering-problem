#[cfg(test)]
mod test {
    use set_covering_problem::entity::subset_cover::SubsetCover; 
    use set_covering_problem::entity::set::Set; 

    // Constantes para el Set de prueba estándar:
    // |X| = 5
    // |S| = 5
    // max(S) = 5 (Subconjunto S4)
    // Cobertura completa: {a, b, c, d, e}

    fn setup_test_set() -> Set {
        // Universal Set U = {a, b, c, d, e}
        let subsets_data = vec![
            vec!["a".to_string(), "b".to_string()],                     // S0 (index 0)
            vec!["c".to_string(), "d".to_string()],                     // S1 (index 1)
            vec!["e".to_string()],                                      // S2 (index 2)
            vec!["a".to_string(), "c".to_string()],                     // S3 (index 3)
            vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()], // S4 (index 4)
        ];
        Set::new(subsets_data)
    }

    // ====================================================================
    // 1. Pruebas de Inicialización (SubsetCover::new)
    // ====================================================================

    #[test]
    fn test_new_initialization() {
        let set = setup_test_set();
        let cover = SubsetCover::new(set.clone());

        assert!(cover.elements.is_empty(), "La cobertura debe iniciar vacía");
        assert_eq!(cover.size, 0, "El tamaño debe ser 0");
        
        // El costo inicial debe ser el costo de una cobertura vacía:
        // Costo = (0 + (5 - 0) * 5 * 5) / 5 = 25.0
        assert_eq!(cover.cost, 25.0, "El costo de una cobertura vacía debe ser 25.0");
    }

    // ====================================================================
    // 2. Pruebas de Cálculo de Costo (SubsetCover::calculate_cost)
    // ====================================================================

    #[test]
    fn test_calculate_cost_empty_cover() {
        let set = setup_test_set();
        let empty_subsets = vec![];
        
        // Costo = (0 + (5 - 0) * 5 * 5) / 5 = 25.0
        let cost = SubsetCover::calculate_cost(&empty_subsets, &set);
        assert_eq!(cost, 25.0);
    }

    #[test]
    fn test_calculate_cost_incomplete_cover() {
        let set = setup_test_set();
        // C = {S0, S1} -> Cubre {a, b, c, d} -> Unión = 4
        // |C| = 2
        let subsets = vec![0, 1]; 

        // Costo = (|C| + (|X| - |Union|) * max(S) * |S|) / |S|
        // Costo = (2 + (5 - 4) * 5 * 5) / 5 = (2 + 25) / 5 = 27 / 5 = 5.4
        let cost = SubsetCover::calculate_cost(&subsets, &set);
        assert_eq!(cost, 5.4);
    }

    #[test]
    fn test_calculate_cost_optimal_cover() {
        let set = setup_test_set();
        // C = {S4} (El subconjunto universal) -> Unión = 5
        // |C| = 1
        let subsets = vec![4]; 

        // Costo = (1 + (5 - 5) * 5 * 5) / 5 = (1 + 0) / 5 = 0.2
        let cost = SubsetCover::calculate_cost(&subsets, &set);
        assert_eq!(cost, 0.2);
    }

    #[test]
    fn test_calculate_cost_redundant_cover() {
        let set = setup_test_set();
        // C = {S0, S1, S2, S4} -> Unión = 5
        // |C| = 4
        let subsets = vec![0, 1, 2, 4]; 

        // Costo = (4 + (5 - 5) * 5 * 5) / 5 = (4 + 0) / 5 = 0.8
        let cost = SubsetCover::calculate_cost(&subsets, &set);
        assert_eq!(cost, 0.8);
    }

    // ====================================================================
    // 3. Pruebas de Adición (SubsetCover::add_subset)
    // ====================================================================

    #[test]
    fn test_add_subset_updates_state() {
        let set = setup_test_set();
        let mut cover = SubsetCover::new(set.clone());

        // 1. Agregar S0. C={S0}. |C|=1. Unión={a,b}. |Union|=2.
        // Costo = (1 + (5 - 2) * 5 * 5) / 5 = (1 + 75) / 5 = 76 / 5 = 15.2
        let cost_1 = cover.add_subset(0, &set);
        assert_eq!(cover.elements, vec![0], "Debe contener el índice 0");
        assert_eq!(cover.size, 1);
        assert_eq!(cost_1, 15.2);
        assert_eq!(cover.cost, 15.2);

        // 2. Agregar S1. C={S0, S1}. |C|=2. Unión={a,b,c,d}. |Union|=4.
        // Costo = (2 + (5 - 4) * 5 * 5) / 5 = (2 + 25) / 5 = 27 / 5 = 5.4
        let cost_2 = cover.add_subset(1, &set);
        assert_eq!(cover.elements, vec![0, 1], "Debe contener los índices 0 y 1 ordenados");
        assert_eq!(cover.size, 2);
        assert_eq!(cost_2, 5.4);
        assert_eq!(cover.cost, 5.4);
    }

    #[test]
    fn test_add_subset_duplicate() {
        let set = setup_test_set();
        let mut cover = SubsetCover::new(set.clone());

        // 1. Agregar S0. Costo = 15.2
        cover.add_subset(0, &set);

        // 2. Intentar agregar S0 de nuevo. El costo y el estado no deben cambiar.
        let cost_after_duplicate = cover.add_subset(0, &set);
        assert_eq!(cover.elements, vec![0], "No debe agregar el duplicado");
        assert_eq!(cover.size, 1);
        assert_eq!(cost_after_duplicate, 15.2, "El costo no debe cambiar");
    }
}