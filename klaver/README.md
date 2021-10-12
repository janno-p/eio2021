# 2. Katkine klaver (`klaver`)

*0,5 sek / 3 sek*  
*30 punkti*

MIDI (ingl *Musical Instrument Digital Interface*) on tehniline standard muusikateoste
kirjeldamiseks, kus teose iga nooti esitab täisarv lõigust 0...1 023. Sul on 1 024 klahviga klaver,
millega soovid esitada MIDI vormingus antud muusikapala. Klaveri vasakpoolseim klahv vastab MIDI
süsteemi arvule 0, vasakult teine arvule 1 j.n.e.

Kahjuks on klaverit juba palju kasutatud ja seetõttu on mõned klahvid katki läinud. Sa oled
klaveri olukorraga kursis ja tead täpselt, millised klahvid töötavad ja millised on katki. Võib
eeldada, et pala esitamise ajal ei lähe ükski töötav klahv katki, aga ka ükski katkine klahv ei
hakka pala esitamise ajal tööle.

Kirjutada programm, mis leiab

* mitut nooti pole võimalik klaveril antud muusikapalas õigesti mängida;
* minimaalse transponeeritavate pooltoonide arvu, et muusikapala oleks võimalik klaveril esitada.

Transponeerimine tähendab kõikide nootide helikõrguse nihutamist üles või alla. Nootide ühe
pooltooni võrra üles transponeerimine tähendab, et klaveril mängitakse esialgse klahvi paremat
naaberklahvi. Nootide ühe pooltooni võrra alla transponeerimine tähendab, et klaveril mängitakse
esialgse klahvi vasakut naaberklahvi. Kahe pooltooni võrra transponeerimine tähendab, et klaveril
mängitakse klahvi, mis on esialgsest klahvist ülejärgmine j.n.e. Pane tähele, et kui transponeerida
kõiki noote ühe pooltooni võrra üles, siis ei saa mängida nooti, mille MIDI kood algses muusikapalas
on 1 023, sest sellele vastavat klahvi klaveril ei ole.

**Sisend.** Sisendi esimesel real on klaveri katkiste klahvide arv *N* (1 &le; *N* &le; 1 024).
Teisel real on *N* paarikaupa erinevat täisarvu *A<sub>i</sub>* (0 &le; *A<sub>i</sub>* &le; 1 023):
klaveri katkiste klahvide MIDI koodid. Kolmandal real on muusikateose nootide arv *M*
(1 &le; *M* &le; 10<sup>6</sup>). Neljandal real on *M* täisarvu *B<sub>i</sub>*
(0 &le; *B<sub>i</sub>* &le; 1 023): muusikateose nootide MIDI koodid.

**Väljund.** Väljastada täpselt kaks rida. Esimesele reale väljastada üks täisarv, mis näitab, mitut
nooti teoses ei saa klaveril õige klahviga mängida. Teisele reale väljastada muusikapala klaveril
mängimiseks vajalik transpositsioon pooltoonides. Kui pala on võimalik mängida mitmel moel
transponeerides, väljastada vähima absoluutväärtusega transpositsioon. Kui pala pole võimalik
ka transponeerides mängida, siis väljastada teisele reale `X`. Kui lahendus ei oska seda arvu leida,
siis väljastada teisele reale arv reale `E`.

**Näide.**

`klaversis0.txt`

    2
    7 8
    6
    4 5 6 7 8 7

`klaverval0.txt`

    3
    -2

Katki on kaks klahvi. Neile vastavad noodid esinevad palas kolm korda. Pala esitamiseks
transponeerime kõiki noote kahe pooltooni võrra alla. See tähendab, et pala esimese noodi
esitamiseks mängime vasakult teist klahvi.

**Näide.**

`klaversis1.txt`

    2
    0 1023
    4
    0 1023 1023 1023

`klaverval1.txt`

    4
    X

**Hindamine.** Selles ülesandes annab väljundi esimese rea leidmine 1/3 ja teise rea leidmine
2/3 punktidest. Lisaks on testid jagatud gruppidesse ja iga grupi eest saavad punkte ainult
need lahendused, mis läbivad kõik sellesse gruppi kuuluvad testid. Gruppides kehtivad järgmised
lisatingimused:

1. (15 punkti) *M* &le; 1 000 ning kõiki teoseid on võimalik mängida ja neid ei ole selleks vaja
transponeerida rohkem kui kahe pooltooni võrra.
2. (15 punkti) Lisapiirangud puuduvad.
