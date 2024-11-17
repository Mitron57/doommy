#[cfg(test)]
mod tests {
    // Простой словарь с числами
    #[test]
    fn test_simple_dict_with_numbers() {
        let input = r#"
        $[
            NAME1: 10,
            NAME2: 20
        ]
        "#;

        let expected_output = r#"
        NAME1: 10
        NAME2: 20
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Массив с числами и константами
    #[test]
    fn test_list_with_numbers_and_constants() {
        let input = r#"
        (list 10 20 30)
        (def CONSTANT_1 40);
        (def CONSTANT_2 5);
        (list CONSTANT_1 CONSTANT_2)
        "#;

        let expected_output = r#"
        - 10
        - 20
        - 30
        - 40
        - 5
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Сложное вычисление с константами
    #[test]
    fn test_complex_expression_with_constants() {
        let input = r#"
        (def BASE 10);
        (def OFFSET 5);
        ^{BASE + OFFSET}
        "#;

        let expected_output = r#"
        15
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Вложенные массивы и словари
    #[test]
    fn test_nested_lists_and_dicts() {
        let input = r#"
        $[
            NAME1: (list 1 2 3),
            NAME2: $[
                NAME3: 100,
                NAME4: (list 4 5)
            ]
        ]
        "#;

        let expected_output = r#"
        NAME1:
          - 1
          - 2
          - 3
        NAME2:
          NAME3: 100
          NAME4:
            - 4
            - 5
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Ошибка в синтаксисе — неправильный формат массива
    #[test]
    fn test_invalid_list_syntax() {
        let input = r#"
        (list 10 20 30
        "#;

        let expected_error = "Ошибка: не закрыта скобка в списке (list)";

        let result = parse_config(input);
        assert_eq!(result, expected_error);
    }

    // Ошибка в синтаксисе — неправильный формат словаря
    #[test]
    fn test_invalid_dict_syntax() {
        let input = r#"
        $[
            NAME1: 10,
            NAME2 20
        ]
        "#;

        let expected_error = "Ошибка: неправильный формат словаря (имя: значение)";

        let result = parse_config(input);
        assert_eq!(result, expected_error);
    }

    // Ошибка вычисления константного выражения — недопустимая операция
    #[test]
    fn test_invalid_constant_expression() {
        let input = r#"
        (def BASE 10);
        (def OFFSET 5);
        ^{BASE * OFFSET}
        "#;

        let expected_error = "Ошибка: операция '*' не поддерживается для константных выражений";

        let result = parse_config(input);
        assert_eq!(result, expected_error);
    }

    // Пример вычисления с функцией pow()
    #[test]
    fn test_pow_function_in_expression() {
        let input = r#"
        (def BASE 2);
        (def EXPONENT 3);
        ^{pow(BASE, EXPONENT)}
        "#;

        let expected_output = r#"
        8
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Использование констант в массиве и словаре
    #[test]
    fn test_constants_in_lists_and_dicts() {
        let input = r#"
        (def BASE 10);
        (def OFFSET 5);
        $[
            NAME1: BASE,
            NAME2: OFFSET,
            NAME3: ^{BASE + OFFSET}
        ]
        "#;

        let expected_output = r#"
        NAME1: 10
        NAME2: 5
        NAME3: 15
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Сложная структура с несколькими уровнями вложенности
    #[test]
    fn test_complex_nested_structure() {
        let input = r#"
        (def A 10);
        (def B 20);
        $[
            NAME1: (list A B),
            NAME2: $[
                NAME3: ^{A + B},
                NAME4: (list ^{A + 1} ^{B - 1})
            ]
        ]
        "#;

        let expected_output = r#"
        NAME1:
          - 10
          - 20
        NAME2:
          NAME3: 30
          NAME4:
            - 11
            - 19
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }

    // Пример с ошибкой в вычислении константного выражения
    #[test]
    fn test_invalid_constant_expression_format() {
        let input = r#"
        (def A 10);
        ^{A +}
        "#;

        let expected_error = "Ошибка: некорректный формат выражения в константе";

        let result = parse_config(input);
        assert_eq!(result, expected_error);
    }

    // Пример с использованием оператора вычитания
    #[test]
    fn test_subtraction_in_expression() {
        let input = r#"
        (def BASE 15);
        (def OFFSET 5);
        ^{BASE - OFFSET}
        "#;

        let expected_output = r#"
        10
        "#;

        let result = parse_config(input);
        assert_eq!(result, expected_output);
    }
}