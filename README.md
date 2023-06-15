Projeto de computação gráfica implementado em Rust utilizando OpenGL.

# Resumo
Reimplementação do projeto de computação gráfica feito em Rust utilizando OpenGL. <br/>
</br>
Nesse projeto, uma cena 3D deve ser implementada com uma ilha que possui dois tipos de plantas e animais. Os animais e plantas devem ficar na terra e não devem atravessar o chão. Além disso, os animais devem se movimentar sem colidir com outro animal ou uma planta.

# Comandos

### Movimentação
Tecla | Comando
|-----|--------
`W`| Anda para frente.
`S`| Anda para trás.
`A`| Anda para a esquerda.
`D`| Anda para a direita.

### Rotação da câmera
Tecla | Comando
|-----|--------
`Seta para esquerda` (&larr;) | Vira a câmera para a esquerda.
`Seta para direita` (&rarr;) | Vira a câmera para a direita.
`Seta para cima` (&uarr;) | Vira a câmera para a cima.
`Seta para baixo` (&darr;) | Vira a câmera para a baixo.

## Zoom in/out
Tecla | Comando
|-----|-------
`-` | Reduz o zoom.
`=` | Aumenta o zoom.

# Como rodar o projeto
Para rodar o projeto basta executar `cargo run` para executar a versão de debug(sem otimização) ou `cargo run --release` para executar a versão mais otimizada.

Caso você queria apenas gerar o binário utilize o comando `cargo build` para gerar a versão de debug  na pasta target/debug ou `cargo build --release` para gerar a versão mais otimizada na pasta target/release.

O nome do binário é `rusty-island`.

# Objetivos
O projeto deve ter: <br>
  - [X] Geração aleatória de terreno (ilha).
    - [X] Adicionar height map para geração de terreno.
  - [X] Geração aleatória de lagos na ilha.
  - [X] Implementar cena.
    - [X] Exibir height map.
    - [X] Armazenar height map.
    - [X] Exibir modelos 3D.
    - [X] Armazenar animais.
    - [X] Armazenar plantas.
  - [X] Adicionar leitura de modelos 3D.
  - [X] Exibir dois tipos de animais terrestres.
    - [X] Todos os animais devem ter um modelo 3D.
    - [X] Todos os animais devem poder se movimentar com o tempo.
      - [X] (Opcional) Utilizar automato para movimento.
        - Utilizando o automato para deifinir um ponto final.
        - Mover animal aos poucos até o ponto final.
      - [X] Os animais devem rotacionar para onde estão se movimentando.
    - [X] Todos os animais devem ter uma colisão.
    - [X] Todos os animais não devem atravessar o chão.
  - [X] Exibir dois tipos de plantas.
    - [X] Todas as plantas devem ter um modelo 3D.
    - [X] Adicionar uma rotação aleatória nas plantas.
    - [X] Todas as plantas devem ter uma colisão.
    - [X] Todas as plantas não devem atravessar o chão.
  - [X] Adicionar luz para a cena.
  - [X] Adicionar shading para a cena.
  - [X] Implementar controle de câmera.
    - [X] Adicionar movimentação da câmera.
    - [X] Adicionar rotação na câmera.
    - [X] Adicionar zoom na câmera.

# Notas
- Aparentemente não é possível mover o cursor utilizando a biblioteca `glium`, então a rotação da câmera só utiliza as setas do teclado.
- Aparentemente a biblioteca `obj-rs` não coleta as malhas dos arquivos `.obj` de forma separada, então as todos os elementos das plantas são uma malha só (o modelo 3D diferencia o troco das folhas).
- Como o projeto é apenas um exercício extra de computação gráfica, o projeto não possui uma cobertura de 100% nos testes e nem tudo está completamente documentado.
