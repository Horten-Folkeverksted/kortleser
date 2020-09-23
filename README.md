# Kortleser

Kortleser er et program og nixpakker for infoskjermen på [Horten Folkeverksted](https://folkeverkstedet.com)

Mesteparten av `image` pakken  er inspirert (diretke stjålet) fra Matthew Bauer's [nixiosk](https://github.com/matthewbauer/nixiosk)

## Installation

Bruk [nix](https://nixos.org/) for å bygge pakkene.


```bash
nix-build -A nativeBuild
```

Hvis du trenger en cross compilet build for Raspberry Pi av en eller annen grunn (WARNING: Dette tar lang tid):


```bash
nix-build -A rpiBuild
```

For å generere et SD-kort bilde for raspberry pi 0 (og kanskje 1):
```
nix-build -A image
```

Dette tar mange timer (3.5h på en i7-4790K). Men kan speedes opp ved å bruke [nixiosk](https://github.com/matthewbauer/nixiosk)'s binary cache:
```
cachix use nixiosk
```
## Usage


```bash
./kortleser
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

All the Nix files are [MIT](https://choosealicense.com/licenses/mit/)
