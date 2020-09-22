# Kortleser

Kortleser er et program og nixpakker for infoskjermen på [Horten Folkeverksted](https://folkeverkstedet.com)

## Installation

Bruk [nix](https://nixos.org/) for å bygge pakkene.


```bash
nix-build -A nativeBuild
```

Hvis du trenger en cross compilet build for Raspberry Pi av en eller annen grunn (WARNING: Dette tar lang tid):


```bash
nix-build -A rpiBuild
```

## Usage


```bash
./cardreader
```
```bash
nc localhost 3333
```

Alt som blir skrevet til stdin i `cardreader` blir hashet og sendt til netcat.

## Contributing
for et shell med cargo og riktig rust versjon:
```bash
nix-shell
```

## License
[AGPL](https://choosealicense.com/licenses/agpl-3.0/)
