# 3. Õhuleke kosmosejaamas (`leke`)

*1 sek*  
*40 punkti*

Kosmosejaam koosneb kuubikujulistest moodulitest, mis on paigutatud 25×25×25 võrena. Iga mooduli
asukohta võres näitavad selle X-, Y- ja Z-koordinaadid täisarvuliste väärtustega 1&nbsp;...&nbsp;25.
Nii X- kui Y- kui Z-telje suunas kõrvuti olevad moodulid on omavahel ühendatud käikudega. Iga käigu
mõlemas otsas on uksed. Kui mõne käigu mõlemas otsas olevad uksed on avatud, pääseb õhk nende
moodulite vahel vabalt liikuma.

Üks moodul on aga viga saanud ja sealt lekib õhk avakosmosesse. Õhku kaotavad ka kõik need moodulid,
kust see pääseb lekkega moodulisse kas otse või läbi muude moodulite. Leida kõik õhku kaotavad
moodulid. Võib eeldada, et ka viga saanud mooduli käikude uksed on siiski terved.

**Sisend.** Sisendi esimesel real on avatud uste arv *U* (1 &le; *U* &le; 10&nbsp;000). Järgmisel
*U* real on igaühel ühe avatud ukse kirjeldus: mooduli koordinaadid ning ukse suund (telje nimetus
ning '+' koordinaatide kasvava ja '-' kahaneva suuna näitajana). Viimasel real on viga saanud
mooduli koordinaadid.

**Väljund.** Väljastada kõigi õhku kaotavate moodulite koordinaadid, iga moodul eraldi reale. Read
järjestada kasvavalt kõigepealt Z-, seejärel Y- ja siis X-koordinaadi järgi.

**Näide.**

`lekesis0.txt`

    5
    2 1 1 X-
    1 1 1 X+
    1 1 1 Z+
    3 1 1 X-
    1 1 2 Z-
    1 1 1

`lekeval0.txt`

    1 1 1
    2 1 1
    1 1 2

Viga sai nurgas asuv moodul koordinaatidega (1; 1; 1). Sel on avatud uksed X- ja Z-telje kasvaval
suunal. Kuna X-teljel järgmise mooduli (2; 1; 1) uks X-telje kahaneval suunal on samuti avatud,
pääseb sealt õhk viga saanud moodulisse. Sarnane on lugu ka viga saanud mooduli Z-telje suunalise
naabriga (1; 1; 2). Moodul (3; 1; 1) õhku ei kaota, sest mooduli (2; 1; 1) uks X-telje kasvaval
suunal on kinni. Väljundis on moodul (1; 1; 2) viimane, sest järjestamisel on esimeseks aluseks
Z-koordinaadid.

**Hindamine.** Testides koguväärtusega 8 punkti on sisendis kõigi Y- ja Z-koordinaatide väärtused 1.
Järgmistes testides koguväärtusega 12 punkti on kõigi Z-koordinaatide väärtused 1. Ülejäänud
testides lisapiiranguid ei ole.
