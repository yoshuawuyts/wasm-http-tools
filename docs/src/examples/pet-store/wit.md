# WIT definition

## Routes

```wit
interface pets {
    record list-pets-input {
        limit: option<s32>
    }

    record list-pets-response {
        pet-array: list<pet>,
        x-next: string,
    }

    func list-pets(input: list-pets-input) -> result<list-pets-response, error>;

    func create-pets() -> result<_, error> {}

    record show-pet-by-id-input {
        pet-id: string,
    }
    func show-pet-by-id(show-pet-by-id-input) -> result<pet, error> {}
}
```

## Schemas
```wit
record pet {
    id: s64,
    name: string,
    tag: string,
}

type pets = list<pet>;

record error {
    code: s32,
    message: string,
}
```
