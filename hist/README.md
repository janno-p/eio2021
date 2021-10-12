# 1. Tähtede histogramm (`hist`)

*1 sek*  
*20 punkti*

Juku valmistab trükimasinat ette trükise tegemiseks. Selle käigus peab ta kontrollima, kas tal on
piisavas koguses tüüpe — ladumiseks kasutatavaid tähtede ja märkide (ehk tärkide) malle. Juku
abistamiseks on vaja koostada programm, mis teeb etteantud teksti kohta kindlaks, kui mitut
eksemplari igast tärgist on vaja.

**Sisend.** Sisendi esimesel real on järgnevate tekstiridade arv *N* (1 &le; *N* &le; 1 000). Iga
järgnev tekstirida sisaldab kuni 100 tärki. Tekst võib koosneda ladina tähestiku suurtest ja
väikestest tähtedest, numbritest ja kirjavahemärkidest.

**Väljund.** Väljastada sisendis esinevate tärkide histogramm. Igale reale väljastada tühikuga
eraldatud tärk ja täisarv: vastava tärgi esinemiste arv sisendis. Read võib väljastada mistahes
järjekorras. Tühikuid ja reavahetusi tärkidena mitte lugeda.

**Näide.**

`histsis0.txt`

    4
    A printing press is a mechanical device
    for applying pressure to an inked surface
    resting upon a print medium (such as paper
    or cloth), thereby transferring the ink.

`histval0.txt`

    ( 1
    ) 1
    , 1
    . 1
    A 1
    a 10
    b 1
    c 6
    d 3
    e 15
    f 3
    g 4
    h 5
    i 12
    k 2
    l 3
    m 3
    n 12
    o 5
    p 9
    r 14
    s 10
    t 8
    u 5
    v 1
    y 2
