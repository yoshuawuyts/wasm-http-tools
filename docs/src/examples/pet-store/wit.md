# WIT definition
https://raw.githubusercontent.com/yoshuawuyts/openapi-bindgen/main/fixtures/petstore.json

https://petstore3.swagger.io

## Routes

```rust
mod pets {
    fn list_pets() {}
    fn create_pets() {}
    async fn show_pet_by_id(pet_id: String) {}
}
```

```wit
interface pets {
    fun list-pets(limit: option<s32>);
    fun create-pets() {}
    fun show-pet-by-id(pet_id: string) {}
}
```

## Schemas
```wit
record Pet {
    id: s64,
    name: string,
    tag: string,
}

type Pets = list<Pet>;

record Error {
    code: s32,
    message: string,
}
```
