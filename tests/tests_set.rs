#[cfg(test)]
mod test {
    use set_covering_problem::entity::set::Set; 

    // Estructura auxiliar para crear un Set de prueba estándar.
    fn setup_test_set() -> Set {
        // Universal Set U = {a, b, c, d, e} (5 elementos)
        // Elementos y sus índices: a=0, b=1, c=2, d=3, e=4
        let subsets_data = vec![
            vec!["a".to_string(), "b".to_string()],      // S0 (index 0)
            vec!["c".to_string(), "d".to_string()],      // S1 (index 1)
            vec!["e".to_string()],                       // S2 (index 2)
            vec!["a".to_string(), "c".to_string()],      // S3 (index 3)
            vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()], // S4 (index 4)
        ];
        Set::new(subsets_data)
    }

    // ====================================================================
    // 1. Pruebas de Inicialización (Set::new)
    // ====================================================================

    #[test]
    fn test_new_initialization() {
        let set = setup_test_set();

        // Verificar el número de subconjuntos y elementos
        assert_eq!(set.get_size_subsets(), 5, "Debe haber 5 subconjuntos");
        assert!(set.get_size_set() == 5, "Debe haber al menos 5 elementos únicos"); 
        
        // Verificar min y max subset size
        assert_eq!(set.max_subset, 5, "El subconjunto S4 tiene tamaño 5");
        assert_eq!(set.get_elements(), vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]);
    }

    // ====================================================================
    // 2. Pruebas de Funciones Auxiliares
    // ====================================================================

    #[test]
    fn test_get_elements_in_subset() {
        let set = setup_test_set();

        // S0 = {a, b}. Asumimos a=0, b=1
        let s0_elements = set.get_elements_in_subset(0).unwrap();
        assert_eq!(s0_elements.len(), 2, "S0 debe tener 2 elementos");

        // S2 = {e}. Asumimos e=4 (o el índice que corresponda)
        let s2_elements = set.get_elements_in_subset(2).unwrap();
        assert_eq!(s2_elements.len(), 1, "S2 debe tener 1 elemento");

        // Índice inválido
        assert!(set.get_elements_in_subset(5).is_err(), "Índice fuera de rango debe fallar");
    }

    #[test]
    fn test_union_subset() {
        let set = setup_test_set();

        // Unión de S0={a, b} y S1={c, d}
        let subsets_union_1 = vec![0, 1]; 
        let union_result_1 = set.union_subset(&subsets_union_1).unwrap();
        assert_eq!(union_result_1.len(), 4, "La unión debe contener 4 elementos (a,b,c,d)");

        // Unión de S0={a, b} y S3={a, c}. Intersección {a}.
        let subsets_union_2 = vec![0, 3]; 
        let union_result_2 = set.union_subset(&subsets_union_2).unwrap();
        assert_eq!(union_result_2.len(), 3, "La unión debe contener 3 elementos únicos (a,b,c)");
    }
    
    // ====================================================================
    // 3. Pruebas de Cobertura y Costo (is_cover_valid y calculate_cost)
    // ====================================================================
    
    #[test]
    fn test_is_cover_valid() {
        let set = setup_test_set();

        // S4 cubre todos los elementos (solución válida)
        let valid_cover = vec![4]; 
        assert!(set.is_cover_valid(&valid_cover), "S4 debe ser una cobertura válida");

        // S0={a,b}, S1={c,d}, S2={e}. Cubre todos (solución válida)
        let valid_cover_disjoint = vec![0, 1, 2]; 
        assert!(set.is_cover_valid(&valid_cover_disjoint), "S0, S1, S2 deben ser una cobertura válida");

        // S0={a,b}, S1={c,d}. No cubre {e} (solución inválida)
        let invalid_cover = vec![0, 1]; 
        assert!(!set.is_cover_valid(&invalid_cover), "S0 y S1 no deben ser una cobertura válida (falta e)");
    }
    
    // ====================================================================
    // 4. Pruebas de count_disjoint_subsets (La nueva lógica)
    // ====================================================================

    #[test]
    fn test_count_disjoint_subsets_fully_isolated() {
        let set = setup_test_set();
        println!("{:?}",set.elements);

        // Familia F1 = {S0, S1, S2}
        // S0={a,b}, S1={c,d}, S2={e}. Todos son disjuntos de los otros dos.
        // Esperado: 3
        let family_1: Vec<usize> = vec![0, 1, 2]; 
        assert_eq!(set.count_disjoint_subsets(&family_1), 3, "Los 3 subconjuntos son mutuamente disjuntos");

        // Familia F2 = {S0, S1, S3}
        // S0={a,b} interseca con S3={a,c}. -> FALLA
        // S2={e} es disjunto con S0 y S3. -> OK
        // S3={a,c} interseca con S0={a,b}. -> FALLA
        // Esperado: 1 (solo S1)
        let family_2: Vec<usize> = vec![0, 2, 3];
        let num_disjoint_subsets = set.count_disjoint_subsets(&family_2);
        assert_eq!(num_disjoint_subsets, 1, "Solo S1 debe ser totalmente disjunto en F2:\n{}",num_disjoint_subsets);
    }

    #[test]
    fn test_count_disjoint_subsets_all_intersect() {
        let set = setup_test_set();

        // Familia F3 = {S0, S3, S4}
        // S0={a,b} interseca con S3={a,c} y S4={a,b,c,d,e}. -> FALLA
        // S3={a,c} interseca con S0 y S4. -> FALLA
        // S4 (Universal) interseca con S0 y S3. -> FALLA
        // Esperado: 0
        let family_3 = vec![0, 3, 4]; 
        assert_eq!(set.count_disjoint_subsets(&family_3), 0, "Ningún subconjunto es totalmente disjunto en F3");
    }

    #[test]
    fn test_count_disjoint_subsets_edge_cases() {
        let set = setup_test_set();

        // Caso 1: Familia vacía
        let family_empty = vec![];
        assert_eq!(set.count_disjoint_subsets(&family_empty), 0, "Familia vacía debe retornar 0");

        // Caso 2: Un solo subconjunto (no hay otro con el que compararse)
        let family_one = vec![0];
        assert_eq!(set.count_disjoint_subsets(&family_one), 0, "Familia con un elemento debe retornar 0");
    }
}