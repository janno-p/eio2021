# 4. Vormelistrateegia (`vormel`)

*1 sek / 10 sek*  
*60 punkti*

Erika Isabella Orav osaleb võistkonnas Celsius MotoSport vormelivõistlusel FI (Formula Informatics).
Varsti on algamas järjekordne vormelietapp ning ilm tõotab tulla ilus. Tiim on korralikult trenni
teinud ja leidnud parimad algoritmid kiiresti rehvide vahetamiseks, kuid neil on vaja abi strateegia
välja arvutamisel.

Tuleval vormelietapil sõidetakse rajal *N* ringi. Võistlejatel on kasutada *M* erinevat tüüpi rehve,
millel on erinev koostis. Rehvi koostis määrab selle haarduvuse (mis mõjutab, kui kiiresti ringe
läbida saab) ja kulumiskiiruse (mis määrab, kui kiiresti rehv kehveneb). Iga tüüpi rehve on saadaval
piiramatus koguses.

Vormelietapi stardis on igal vormeliautol võistkonna valitud rehvid all (ja selleks boksipeatust
tegema ei pea). Edaspidi võivad võistlejad ükskõik millise ringi järel teha boksipeatuse, kus saab
vahetada rehvid uute vabalt valitud tüüpi rehvide vastu. Harjutamise tulemusena teavad nad, et
iga boksipeatus võtab *K* sekundit.

Celsius MotoSport on saadaolevaid rehve analüüsinud ja tuvastanud iga rehvitüübi *i* kohta kaks
parameetrit:

* *P<sub>i</sub>* — mitu sekundit võtab selle rehviga esimene ring,
* *W<sub>i</sub>* — mitu sekundit aeglasem on iga järgmine ring võrreldes eelnevaga.

Leida optimaalne rehvivahetuste strateegia, et etapp minimaalse ajaga läbida.

**Sisend.** Sisendi esimesel real on kolm tühikutega eraldatud täisarvu *M* (1 &le; *M* &le; 500),
*N* (1 &le; *N* &le; 200) ja *K* (1 &le; *K* &le; 1&nbsp;000). Järgneval *M* real on igaühel ühe
rehvitüübi kirjeldus: kaks tühikuga eraldatud täisarvu *P<sub>i</sub>* (1 &le; *P<sub>i</sub>* &le;
1&nbsp;000) ja *W<sub>i</sub>* (0 &le; *W<sub>i</sub>* &le; 1&nbsp;000). Rehvitüübid on nummerdatud
1,&hellip;,*M* nende sisendis kirjeldamise järjekorras.

**Väljund.** Väljundi esimesele reale väljastada tühikuga eraldatult täisarvud *i<sub>0</sub>* ja
*B*, vastavalt rehvitüübi number, millega võistlust alustada, ja boksipeatuste arv. Järgmisele *B*
reale väljastada igaühele ühe boksipeatuse kirjeldus kahe täisarvuna, vastavalt pärast mitmendat
ringi teha boksipeatus (ringid on nummerdatud 1,&hellip;,*N*) ning uute rehvide tüübi number.
Boksipeatused väljastada kronoloogilises järjekorras. Kui optimaalseid lahendusi on mitu, väljastada
ükskõik milline neist.

**Näide.**

`vormelsis0.txt`

    2 2 25
    45 11
    40 20

`vormelval0.txt`

    2 0

Läbida tuleb kaks ringi, boksipeatus võtab aega rohkem kui kummagi rehvi kulumine ühe ringiga.
Esimese rehviga läheks aega 45 + (45 + 11) = 101 sekundit, teisega 40 + (40 + 20) = 100 sekundit,
seega alustada tuleks teise rehviga ja sellega ilma boksipeatuseta lõpuni sõita.

**Näide.**

`vormelsis1.txt`

    2 44 170
    60 8
    30 29

`vormelval1.txt`

    1 6
    6 1
    12 1
    18 1
    24 1
    30 1
    37 1

**Näide.**

`vormelsis2.txt`

    3 1 25
    45 10
    40 20
    55 10

`vormelval2.txt`

    2 0

**Hindamine.** Testides koguväärtusega 10 punkti on *M* = 1. Järgmistes testides koguväärtusega
samuti 10 punkti on *N* = 2. Järgmistes testides koguväärtusega veel 10 punkti on *M* &le; 50.
Järgmistes testides koguväärtusega veel 10 punkti on *M* &le; 100. Ülejäänud testides koguväärtusega
20 punkti lisapiiranguid ei ole.
