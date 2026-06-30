# TicTacToe in Rust

* Enum mit X/O: `Player`
* 2-dimensionales Array 3x3 Feldern mit Enum: `[[Option<Player>; 3]; 3]`
* Struct für TTT Feld: `GameState`
  * Enthält das Feld (2-dim. Array, siehe oben); nicht `pub`
  * `next_player`: `Player`; nicht `pub`
  * Implements `Default`
  * Implements `Display`
  * Methode für Spielzug
    * `make_turn`
    * Wechselt den Spieler falls beim Zug kein Fehler aufgetreten ist
    * Input:
      * `Position`
    * Output:
      * `Result<??, ??>`
      * Erfolgsfall: `Option` of Enum `GameResult`
        * `Win(Player)` (drei in einer Reihe, einer Spalte, Diagonale)
        * `Draw` (alle Felder belegt, kein Win)
      * Fehlerfall: Enum `IllegalTurn`
        * Feld schon belegt
        * Spiel schon vorbei
  * Methode zum Abfragen des nächsten Spielers `get_next_player`: `Option<Player>`
* Struct, die eine Position im Spielfeld repräsentiert `Position`
  * Implements `FromStr`
  * `x` und `y`, dürfen pub sein
* `main` Methode:
  * Anlegen der `GameState` Instanz
    * `Default::default()`
  * Schleife `loop`
    * Ausgeben: Spielfeld, wer dran ist
    * Eingabe: Welche Koordinaten im Format `A1`, `B2`...
    * `parse` mit `FromStr` von `Position` -> im Fehlerfall, Fehler ausgeben und nächsten Schleifendurchlauf anstoßen
    * `make_turn` aufrufen und entsprechendes Ergebnis auf den Bildschirm ausgeben
    * Falls Erfolg, dann Schleife beenden
* Module/Files:
  * `tictactoe` - enthält alles, was nicht UI ist
  * `tictactoe` ist ein eigenes Modul in einem eigenen File
  * `tictactoe` hat ein Untermodul `tests`
