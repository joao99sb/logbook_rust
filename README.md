## 1) estrutura de arvore para implementar a hierarquia

exemplo:

    Root --
          |
          |--linux --
          |         |
          |         |--commands--
          |         |            |
          |         |            |---ps
          |         |            |---ls
          |         |             ---rm
          |         |--TODO--
          |                 |
          |                   --- Lista1
          |                   --- Lista2
          |--Others--
          |
           ...

## estrutura de comandos:

quando for ativado o modo commands do app, é para habilitar apenas os commandos de edição

proibir o modo key:Up e Down ativo se não estiver selecionado apenas os nodes na saida parão


preciso especificar um comando para o corpo saber se carrega os arquivos ou os nodes:
    primeiro eu preciso melhorar a estrutura de eventos para especificar o tipo de evento para poder passar para o corpo
    se tiver a flag has_command deixar entrar para não fazer chamadas desnecessarias:
        se tiver olhar para a prop command type e lidar com isso: 

    se não:
        deixar passar como as configurações anteriores    