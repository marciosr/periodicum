pub fn make_table () -> &'static str {
	let s =
	r#"---
- name: Hydrogen
  symbol: H
  atomic_number: 1
  atomic_weigth: "1.00794"
  density: 0.0708 (@ -253 °C)
  melting_point: "14.01"
  boiling_point: "20.28"
  atomic_radius: 53-79
  covalent_radius: "32"
  ionic_radius: 54 (-1e)
  atomic_volume: "14.1"
  specific_heat: 14.267 (H-H)
  fusion_heat: 0.117 (H-H)
  evaporation_heat: 0.904 (H-H)
  thermal_conductivity: "0.1815"
  debye_temperature: "110.00"
  pauling_negativity_number: "2.20"
  first_ionizing_energy: "1311.3"
  oxidation_states: "1, 0, -1"
  eletronic_configuration: "1s&#185;"
  lattice_structure: HEX
  lattice_constant: "3.750"
  lattice_ca_ratio: "1.731"
  appearance: "Colorless, odorless, tasteless gas"
  discovery_date: 1766 (England)
  discovered_by: Henry Cavendish
  named_after: "Greek: hydro (water) and genes (generate)"
  pos_x: 0
  pos_y: 0
- name: Helium
  symbol: He
  atomic_number: 2
  atomic_weigth: "4.002602"
  density: 0.147 (@ -270 °C)
  melting_point: "0.95"
  boiling_point: "4.216"
  atomic_radius: 28-31
  covalent_radius: 28-140
  ionic_radius: "93"
  atomic_volume: "31.8"
  specific_heat: "5.188"
  fusion_heat: n/a
  evaporation_heat: "0.08"
  thermal_conductivity: "0.152"
  debye_temperature: n/a
  pauling_negativity_number: "4.5"
  first_ionizing_energy: "2361.3"
  oxidation_states: "0"
  eletronic_configuration: "1s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.570"
  lattice_ca_ratio: "1.633"
  appearance: "Inert, colorless, odorless, tasteless gas"
  discovery_date: 1895 (Scotland/Sweden)
  discovered_by: "Sir William Ramsey, Nils Langet, P.T.Cleve"
  named_after: "Greek: helios (sun)"
  pos_x: 17
  pos_y: 0
- name: Lithium
  symbol: Li
  atomic_number: 3
  atomic_weigth: "6.941"
  density: "0.534"
  melting_point: 453.69-553.69
  boiling_point: 1118.5-1613
  atomic_radius: 145-155
  covalent_radius: 134-163
  ionic_radius: 68 (+1e) or 76 (+1e)
  atomic_volume: "13.1"
  specific_heat: "3.489"
  fusion_heat: "2.89"
  evaporation_heat: "148"
  thermal_conductivity: "84.8"
  debye_temperature: "400.00"
  pauling_negativity_number: "0.98"
  first_ionizing_energy: "519.9"
  oxidation_states: "1"
  eletronic_configuration: "[He] 2s&#185;"
  lattice_structure: BCC
  lattice_constant: "3.490"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white metal"
  discovery_date: 1817 (Sweden)
  discovered_by: Johann Arfwedson
  named_after: "Greek: lithos (stone)"
  pos_x: 0
  pos_y: 1
- name: Beryllium
  symbol: Be
  atomic_number: 4
  atomic_weigth: "9.01218"
  density: "1.848"
  melting_point: "1551"
  boiling_point: "3243"
  atomic_radius: "112"
  covalent_radius: "90"
  ionic_radius: 35 (+2e)
  atomic_volume: "5.0"
  specific_heat: "1.824"
  fusion_heat: "12.21"
  evaporation_heat: "309"
  thermal_conductivity: "201"
  debye_temperature: "1000.00"
  pauling_negativity_number: "1.57"
  first_ionizing_energy: "898.8"
  oxidation_states: "2, 1"
  eletronic_configuration: "[He] 2s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.290"
  lattice_ca_ratio: "1.567"
  appearance: "Hard, brittle, steel-gray metal"
  discovery_date: 1798 (France)
  discovered_by: Louis-Nicolas Vauquelin
  named_after: "Greek: beryllos, 'beryl' (a mineral)"
  pos_x: 1
  pos_y: 1
- name: Boron
  symbol: B
  atomic_number: 5
  atomic_weigth: "10.811"
  density: "2.34"
  melting_point: "2573"
  boiling_point: "3931"
  atomic_radius: "98"
  covalent_radius: "82"
  ionic_radius: 23 (+3e)
  atomic_volume: "4.6"
  specific_heat: "1.025"
  fusion_heat: "23.60"
  evaporation_heat: "504.5"
  thermal_conductivity: "27.4"
  debye_temperature: "1250.00"
  pauling_negativity_number: "2.04"
  first_ionizing_energy: "800.2"
  oxidation_states: "3"
  eletronic_configuration: "[He] 2s&#178; 2p&#185;"
  lattice_structure: RHL
  lattice_constant: "8.730"
  lattice_ca_ratio: "0.576"
  appearance: "Hard, brittle, lustrous black semimetal"
  discovery_date: 1808 (England/France)
  discovered_by: "Sir H. Davy, J.L. Gay-Lussac, L.J. Thenard"
  named_after: The Arabic and Persian words for borax
  pos_x: 12
  pos_y: 1
- name: Carbon
  symbol: C
  atomic_number: 6
  atomic_weigth: "12.011"
  density: 2.25 (graphite)
  melting_point: "3820"
  boiling_point: "5100"
  atomic_radius: "91"
  covalent_radius: "77"
  ionic_radius: 16 (+4e) 260 (-4e)
  atomic_volume: "5.3"
  specific_heat: "0.711"
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: "1.59"
  debye_temperature: 1860.00 (diamond)
  pauling_negativity_number: "2.55"
  first_ionizing_energy: "1085.7"
  oxidation_states: "4, 3, 2, 1, 0, -1, -2, -3, -4"
  eletronic_configuration: "[He] 2s&#178; 2p&#178;"
  lattice_structure: HEX (graphite) DIA (diamond)
  lattice_constant: "3.570"
  lattice_ca_ratio: n/a
  appearance: "Dense, Black (graphite)"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Latin: carbo (charcoal)"
  pos_x: 13
  pos_y: 1
- name: Nitrogen
  symbol: N
  atomic_number: 7
  atomic_weigth: "14.00674"
  density: 0.808 (@ -195.8 °C)
  melting_point: "63.29"
  boiling_point: "77.4"
  atomic_radius: "92"
  covalent_radius: "75"
  ionic_radius: 13 (+5e) 171 (-3e)
  atomic_volume: "17.3"
  specific_heat: 1.042 (N-N)
  fusion_heat: 0.720 (N-N)
  evaporation_heat: 5.57 (N-N)
  thermal_conductivity: "0.026"
  debye_temperature: n/a
  pauling_negativity_number: "3.04"
  first_ionizing_energy: "1401.5"
  oxidation_states: "5, 4, 3, 2, 1, 0, -1, -2, -3"
  eletronic_configuration: "[He] 2s&#178; 2p&#179;"
  lattice_structure: HEX or CUB
  lattice_constant: 4.039 (HEX)
  lattice_ca_ratio: 1.651 (HEX)
  appearance: "Colorless, odorless, tasteless, and generally inert gas"
  discovery_date: 1772 (Scotland)
  discovered_by: Daniel Rutherford
  named_after: "Greek: nitron and genes (soda forming)"
  pos_x: 14
  pos_y: 1
- name: Oxygen
  symbol: O
  atomic_number: 8
  atomic_weigth: "15.9994"
  density: 1.149 (@ -183 °C)
  melting_point: "54.8"
  boiling_point: "90.19"
  atomic_radius: 60 (48)
  covalent_radius: "73"
  ionic_radius: 132 (-2e)
  atomic_volume: "14.0"
  specific_heat: 0.916 (O-O)
  fusion_heat: "0.444"
  evaporation_heat: "3.4099"
  thermal_conductivity: "0.027"
  debye_temperature: "155"
  pauling_negativity_number: "3.44"
  first_ionizing_energy: "1313.1"
  oxidation_states: "-2, -1, -1/2, -1/3, 0, 1/2, 1, 2"
  eletronic_configuration: "[He] 2s&#178; 2p&#8308;"
  lattice_structure: MCL or CUB
  lattice_constant: 6.830 (CUB)
  lattice_ca_ratio: n/a
  appearance: "Colorless, odorless, tasteless gas; pale blue liquid"
  discovery_date: 1774 (England/Sweden)
  discovered_by: "Joseph Priestly, Carl Wilhelm Scheele"
  named_after: "Greek: oxys and genes (acid former)"
  pos_x: 15
  pos_y: 1
- name: Fluorine
  symbol: F
  atomic_number: 9
  atomic_weigth: "18.998403"
  density: 1.108 (@ -189 °C)
  melting_point: "53.53"
  boiling_point: "85.03"
  atomic_radius: "71"
  covalent_radius: "72"
  ionic_radius: (-1e) 133
  atomic_volume: "17.1"
  specific_heat: 0.824 (F-F)
  fusion_heat: 0.51 (F-F)
  evaporation_heat: 6.54 (F-F)
  thermal_conductivity: "0.028"
  debye_temperature: n/a
  pauling_negativity_number: "3.98"
  first_ionizing_energy: "1680.0"
  oxidation_states: "-1, 0"
  eletronic_configuration: "[He] 2s&#178; 2p&#8309;"
  lattice_structure: MCL
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Greenish-yellow, pungent, corrosive gas"
  discovery_date: 1886 (France)
  discovered_by: Henri Moissan
  named_after: "Latin: fluere (flow)"
  pos_x: 16
  pos_y: 1
- name: Neon
  symbol: Ne
  atomic_number: 10
  atomic_weigth: "20.1797"
  density: 1.204 (@ -246 °C)
  melting_point: "24.55"
  boiling_point: "27.1"
  atomic_radius: (38)
  covalent_radius: 58-71
  ionic_radius: "112"
  atomic_volume: "16.8"
  specific_heat: "1.029"
  fusion_heat: n/a
  evaporation_heat: "1.74"
  thermal_conductivity: (0.0493)
  debye_temperature: "63.00"
  pauling_negativity_number: "4.4"
  first_ionizing_energy: "2079.4"
  oxidation_states: n/a
  eletronic_configuration: "[He] 2s&#178; 2p&#8310;"
  lattice_structure: FCC
  lattice_constant: "4.430"
  lattice_ca_ratio: n/a
  appearance: "Colorless, odorless, tasteless gas"
  discovery_date: 1898 (England)
  discovered_by: "Sir William Ramsey, M.W. Travers"
  named_after: "Greek: neos (new)"
  pos_x: 17
  pos_y: 1
- name: Sodium
  symbol: Na
  atomic_number: 11
  atomic_weigth: "22.989769"
  density: "0.971"
  melting_point: "370.96"
  boiling_point: "1156.1"
  atomic_radius: "190"
  covalent_radius: "154"
  ionic_radius: 97 (+1e)
  atomic_volume: "23.7"
  specific_heat: "1.222"
  fusion_heat: "2.64"
  evaporation_heat: "97.9"
  thermal_conductivity: "142.0"
  debye_temperature: "150.00"
  pauling_negativity_number: "0.93"
  first_ionizing_energy: "495.6"
  oxidation_states: "1"
  eletronic_configuration: "[Ne] 3s&#185;"
  lattice_structure: BCC
  lattice_constant: "4.230"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white metal"
  discovery_date: 1807 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Medieval Latin: sodanum (headache remedy); symbol from Latin natrium (sodium carbonate)"
  pos_x: 0
  pos_y: 2
- name: Magnesium
  symbol: Mg
  atomic_number: 12
  atomic_weigth: "24.305"
  density: "1.738"
  melting_point: "923"
  boiling_point: "1363"
  atomic_radius: "160"
  covalent_radius: "136"
  ionic_radius: 66 (+2e)
  atomic_volume: "14.0"
  specific_heat: "1.025"
  fusion_heat: "9.20"
  evaporation_heat: "131.8"
  thermal_conductivity: "156"
  debye_temperature: "318.00"
  pauling_negativity_number: "1.31"
  first_ionizing_energy: "737.3"
  oxidation_states: "2"
  eletronic_configuration: "[Ne] 3s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.210"
  lattice_ca_ratio: "1.624"
  appearance: "Lightweight, malleable, silvery-white metal"
  discovery_date: 1808 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Magnesia, ancient city in district of Thessaly, Greece"
  pos_x: 1
  pos_y: 2
- name: Aluminum
  symbol: Al
  atomic_number: 13
  atomic_weigth: "26.981539"
  density: "2.6989"
  melting_point: "933.5"
  boiling_point: 2740-2792
  atomic_radius: "143"
  covalent_radius: "121&#177;4"
  ionic_radius: 51 (+3e)
  atomic_volume: "10.0"
  specific_heat: "0.900"
  fusion_heat: "10.75"
  evaporation_heat: "284.1"
  thermal_conductivity: "237"
  debye_temperature: "394.00"
  pauling_negativity_number: "1.61"
  first_ionizing_energy: 577.5; 1816.7
  oxidation_states: "3"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#185;"
  lattice_structure: FCC
  lattice_constant: "4.050"
  lattice_ca_ratio: n/a
  appearance: "Soft, lightweight, silvery-white metal"
  discovery_date: 1825 (Denmark)
  discovered_by: Hans Christian Oersted
  named_after: "Latin: alumen, aluminis (alum)"
  pos_x: 12
  pos_y: 2
- name: Silicon
  symbol: Si
  atomic_number: 14
  atomic_weigth: "28.0855"
  density: "2.33"
  melting_point: "1688"
  boiling_point: "2623"
  atomic_radius: "132"
  covalent_radius: "111"
  ionic_radius: 42 (+4e) 271  (-4e)
  atomic_volume: "12.1"
  specific_heat: "0.703"
  fusion_heat: "50.6"
  evaporation_heat: "383"
  thermal_conductivity: "149"
  debye_temperature: "625.00"
  pauling_negativity_number: "1.90"
  first_ionizing_energy: "786.0"
  oxidation_states: "4, 2, 0, -4"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#178;, [Ne] 3s 3p&#179;"
  lattice_structure: DIA
  lattice_constant: "5.430"
  lattice_ca_ratio: n/a
  appearance: Amorphous form is brown powder; crystalline form has a gray
  discovery_date: 1825 (Sweden)
  discovered_by: Jons Jacob Berzelius
  named_after: "Latin: silex, silicus (flint)"
  pos_x: 13
  pos_y: 2
- name: Phosphorus
  symbol: P
  atomic_number: 15
  atomic_weigth: "30.973762"
  density: 1.82 (white phosphorus)
  melting_point: "317.3"
  boiling_point: "553"
  atomic_radius: "128"
  covalent_radius: "106"
  ionic_radius: 35 (+5e) 212 (-3e)
  atomic_volume: "17.0"
  specific_heat: "0.757"
  fusion_heat: "2.51"
  evaporation_heat: "49.8"
  thermal_conductivity: (0.236)
  debye_temperature: n/a
  pauling_negativity_number: "2.19"
  first_ionizing_energy: "1011.2"
  oxidation_states: "5, 3, 1, 0, -1, -3"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#179;"
  lattice_structure: BCC
  lattice_constant: "7.170"
  lattice_ca_ratio: n/a
  appearance: "The most common white form is a waxy, phosphorescent solid"
  discovery_date: 1669 (Germany)
  discovered_by: Hennig Brand
  named_after: "Greek: phosphoros (bringer of light)"
  pos_x: 14
  pos_y: 2
- name: Sulfur
  symbol: S
  atomic_number: 16
  atomic_weigth: "32.066"
  density: "2.070"
  melting_point: "386"
  boiling_point: "717.824"
  atomic_radius: "127"
  covalent_radius: "102"
  ionic_radius: 30 (+6e) 184 (-2e)
  atomic_volume: "15.5"
  specific_heat: "0.732"
  fusion_heat: "1.23"
  evaporation_heat: "10.5"
  thermal_conductivity: "0.27"
  debye_temperature: n/a
  pauling_negativity_number: "2.58"
  first_ionizing_energy: "999.0"
  oxidation_states: "6, 4, 2, 1, 0, -1, -2"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#8308;"
  lattice_structure: ORC
  lattice_constant: "10.470"
  lattice_ca_ratio: n/a
  appearance: "Tasteless, odorless, light-yellow, brittle solid"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Latin: sulphur (brimstone)"
  pos_x: 15
  pos_y: 2
- name: Chlorine
  symbol: Cl
  atomic_number: 17
  atomic_weigth: "35.4527"
  density: 1.56 (@ -33.6 °C)
  melting_point: "172.2"
  boiling_point: "238.6"
  atomic_radius: "100"
  covalent_radius: "99"
  ionic_radius: (+7e)27 (-1e)181
  atomic_volume: "18.7"
  specific_heat: 0.477 (Cl-Cl)
  fusion_heat: 6.41 (Cl-Cl)
  evaporation_heat: 20.41 (Cl-Cl)
  thermal_conductivity: "0.009"
  debye_temperature: n/a
  pauling_negativity_number: "3.16"
  first_ionizing_energy: "1254.9"
  oxidation_states: "7, 6, 5, 4, 3, 1, 0, -1"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#8309;"
  lattice_structure: ORC
  lattice_constant: "6.240"
  lattice_ca_ratio: n/a
  appearance: "Greenish-yellow, disagreeable gas"
  discovery_date: 1774 (Sweden)
  discovered_by: Carl Wilhelm Scheele
  named_after: "Greek: chloros (greenish yellow)"
  pos_x: 16
  pos_y: 2
- name: Argon
  symbol: Ar
  atomic_number: 18
  atomic_weigth: "39.948"
  density: "1.40 (@ -189,35 °C)"
  melting_point: "83.8"
  boiling_point: "87.3"
  atomic_radius: (71)
  covalent_radius: 98-106
  ionic_radius: "154"
  atomic_volume: "24.2"
  specific_heat: "0.138"
  fusion_heat: "7.05"
  evaporation_heat: 6.45-6.52
  thermal_conductivity: "0.0177"
  debye_temperature: "85.00"
  pauling_negativity_number: 0.0-4.3
  first_ionizing_energy: "1519.6"
  oxidation_states: "0"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#8310;"
  lattice_structure: FCC
  lattice_constant: "5.260"
  lattice_ca_ratio: n/a
  appearance: "Colorless, tasteless, odorless noble gas"
  discovery_date: 1894 (Scotland)
  discovered_by: "Sir William Ramsey, Baron Rayleigh"
  named_after: "Greek: argos (inactive)"
  pos_x: 17
  pos_y: 2
- name: Potassium
  symbol: K
  atomic_number: 19
  atomic_weigth: "39.0983"
  density: "0.856"
  melting_point: 336.53-336.8
  boiling_point: "1047"
  atomic_radius: "235"
  covalent_radius: "203"
  ionic_radius: (+1e)133
  atomic_volume: "45.3"
  specific_heat: "0.753"
  fusion_heat: "2.33"
  evaporation_heat: "76.9"
  thermal_conductivity: "79.0"
  debye_temperature: "100.00"
  pauling_negativity_number: "0.82"
  first_ionizing_energy: "418.5"
  oxidation_states: "1"
  eletronic_configuration: "[Ar] 4s&#185;"
  lattice_structure: BCC
  lattice_constant: "5.230"
  lattice_ca_ratio: n/a
  appearance: "Soft, waxy, silvery-white metal"
  discovery_date: 1807 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "English: pot ash; symbol from Latin: kalium (alkali)"
  pos_x: 0
  pos_y: 3
- name: Calcium
  symbol: Ca
  atomic_number: 20
  atomic_weigth: "40.078"
  density: "1.55"
  melting_point: "1112"
  boiling_point: "1757"
  atomic_radius: "197"
  covalent_radius: "174"
  ionic_radius: (+2e)99
  atomic_volume: "29.9"
  specific_heat: "0.653"
  fusion_heat: "9.20"
  evaporation_heat: "153.6"
  thermal_conductivity: (201)
  debye_temperature: "230.00"
  pauling_negativity_number: "1.00"
  first_ionizing_energy: "589.4"
  oxidation_states: "2"
  eletronic_configuration: "[Ar] 4s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.580"
  lattice_ca_ratio: n/a
  appearance: "Fairly hard, silvery-white metal"
  discovery_date: 1808 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Latin: calx, calcis (lime)"
  pos_x: 1
  pos_y: 3
- name: Scandium
  symbol: Sc
  atomic_number: 21
  atomic_weigth: "44.95591"
  density: "2.99"
  melting_point: "1814"
  boiling_point: 3104-3110
  atomic_radius: "162"
  covalent_radius: "144"
  ionic_radius: (+3e)72.3
  atomic_volume: "15.0"
  specific_heat: "0.556"
  fusion_heat: "15.8"
  evaporation_heat: "332.7"
  thermal_conductivity: "15.8"
  debye_temperature: n/a
  pauling_negativity_number: "1.36"
  first_ionizing_energy: "630.8"
  oxidation_states: "3"
  eletronic_configuration: "[Ar] 3d&#185; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.310"
  lattice_ca_ratio: "1.594"
  appearance: "Fairly soft, silvery-white metal"
  discovery_date: 1879 (Sweden)
  discovered_by: Lars Nilson
  named_after: "Latin: Scandia (Scandinavia)"
  pos_x: 2
  pos_y: 3
- name: Titanium
  symbol: Ti
  atomic_number: 22
  atomic_weigth: "47.867"
  density: "4.54"
  melting_point: "1933&#177;20"
  boiling_point: "3560"
  atomic_radius: "147"
  covalent_radius: "132"
  ionic_radius: (+4e)68 (+2e)94
  atomic_volume: "10.6"
  specific_heat: "0.523"
  fusion_heat: "18.8"
  evaporation_heat: "422.6"
  thermal_conductivity: "21.9"
  debye_temperature: "380.00"
  pauling_negativity_number: "1.54"
  first_ionizing_energy: "657.8"
  oxidation_states: "2, 3, 4"
  eletronic_configuration: "[Ar] 3d&#178; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.950"
  lattice_ca_ratio: n/a
  appearance: "Shiny, dark-gray metal"
  discovery_date: 1791 (England)
  discovered_by: William Gregor
  named_after: "Greek: titanos (Titans)"
  pos_x: 3
  pos_y: 3
- name: Vanadium
  symbol: V
  atomic_number: 23
  atomic_weigth: "50.9415"
  density: "6.11"
  melting_point: "2160"
  boiling_point: "3650"
  atomic_radius: "134"
  covalent_radius: "122"
  ionic_radius: (+5e)59 (+3e)7
  atomic_volume: "8.35"
  specific_heat: "0.485"
  fusion_heat: "17.5"
  evaporation_heat: "460"
  thermal_conductivity: "30.7"
  debye_temperature: "390.00"
  pauling_negativity_number: "1.63"
  first_ionizing_energy: "650.1"
  oxidation_states: "5, 4, 3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#179; 4s&#178;"
  lattice_structure: BCC
  lattice_constant: "3.020"
  lattice_ca_ratio: n/a
  appearance: "Soft, ductile, silvery-white metal"
  discovery_date: 1830 (Sweden)
  discovered_by: Nils Gabriel Sefstrom
  named_after: "The scandinavian goddess, Vanadis"
  pos_x: 4
  pos_y: 3
- name: Chromium
  symbol: Cr
  atomic_number: 24
  atomic_weigth: "51.9961"
  density: "7,18-7.19"
  melting_point: "2130"
  boiling_point: "2945"
  atomic_radius: "130"
  covalent_radius: "118"
  ionic_radius: (+6e)52 (+3e)63
  atomic_volume: "7.23"
  specific_heat: "0.488"
  fusion_heat: "21"
  evaporation_heat: "342"
  thermal_conductivity: "93.9"
  debye_temperature: "460.00"
  pauling_negativity_number: "1.66"
  first_ionizing_energy: "652.4"
  oxidation_states: "6, 3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#8309; 4s&#185;"
  lattice_structure: BCC
  lattice_constant: "2.880"
  lattice_ca_ratio: n/a
  appearance: "Very hard, crystalline, steel-gray metal"
  discovery_date: 1797 (France)
  discovered_by: Louis Vauquelin
  named_after: "Greek: chroma (color)"
  pos_x: 5
  pos_y: 3
- name: Manganese
  symbol: Mn
  atomic_number: 25
  atomic_weigth: "54.93805"
  density: "7.21"
  melting_point: "1517"
  boiling_point: "2235"
  atomic_radius: "135"
  covalent_radius: "117"
  ionic_radius: (+7e) 46 (+2e) 80
  atomic_volume: "7.39"
  specific_heat: "0.477"
  fusion_heat: "13.4"
  evaporation_heat: "221"
  thermal_conductivity: (7.8)
  debye_temperature: "400.00"
  pauling_negativity_number: "1.55"
  first_ionizing_energy: "716.8"
  oxidation_states: "7, 6, 5, 4, 3, 2, 0, 1, -1"
  eletronic_configuration: "[Ar] 3d&#8309; 4s&#178;"
  lattice_structure: CUB
  lattice_constant: "8.890"
  lattice_ca_ratio: n/a
  appearance: "Hard, brittle, gray-white metal"
  discovery_date: 1774 (Sweden)
  discovered_by: Johann Gahn
  named_after: "Latin: magnes (magnet); Italian: manganese"
  pos_x: 6
  pos_y: 3
- name: Iron
  symbol: Fe
  atomic_number: 26
  atomic_weigth: "55.847"
  density: "7.874"
  melting_point: 1808-1812
  boiling_point: 3023-3134
  atomic_radius: "126"
  covalent_radius: "117"
  ionic_radius: (+3e) 64 (+2e) 74
  atomic_volume: "7.1"
  specific_heat: "25.14"
  fusion_heat: "13.8"
  evaporation_heat: ~340
  thermal_conductivity: "80.4"
  debye_temperature: "460.00"
  pauling_negativity_number: "1.83"
  first_ionizing_energy: "759.1"
  oxidation_states: "6, 3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#8310; 4s&#178;"
  lattice_structure: BCC
  lattice_constant: "2.870"
  lattice_ca_ratio: n/a
  appearance: "Malleable, ductile, silvery-white metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: iron; symbol from Latin: ferrum (iron)"
  pos_x: 7
  pos_y: 3
- name: Cobalt
  symbol: Co
  atomic_number: 27
  atomic_weigth: "58.9332"
  density: "8.9"
  melting_point: "1768"
  boiling_point: "3143"
  atomic_radius: "125"
  covalent_radius: "116"
  ionic_radius: (+3e) 63 (+2e) 72
  atomic_volume: "6.7"
  specific_heat: "0.456"
  fusion_heat: "15.48"
  evaporation_heat: "389.1"
  thermal_conductivity: "100"
  debye_temperature: "385.00"
  pauling_negativity_number: "1.88"
  first_ionizing_energy: "758.1"
  oxidation_states: "3, 2, 0, -1"
  eletronic_configuration: "[Ar] 3d&#8311; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.510"
  lattice_ca_ratio: n/a
  appearance: "Hard, ductile, lustrous bluish-gray metal"
  discovery_date: 1735 (Sweden)
  discovered_by: George Brandt
  named_after: "German: kobold (goblin)"
  pos_x: 8
  pos_y: 3
- name: Nickel
  symbol: Ni
  atomic_number: 28
  atomic_weigth: "58.6934"
  density: "8.902"
  melting_point: "1726"
  boiling_point: "3005"
  atomic_radius: "124"
  covalent_radius: "115"
  ionic_radius: (+2e) 69
  atomic_volume: "6.6"
  specific_heat: "0.443"
  fusion_heat: "17.61"
  evaporation_heat: "378.6"
  thermal_conductivity: "90.9"
  debye_temperature: "375.00"
  pauling_negativity_number: "1.91"
  first_ionizing_energy: "736.2"
  oxidation_states: "3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#8312; 4s&#178;"
  lattice_structure: FCC
  lattice_constant: "3.520"
  lattice_ca_ratio: n/a
  appearance: "Hard, malleable, silvery-white metal"
  discovery_date: 1751 (Sweden)
  discovered_by: Axel Cronstedt
  named_after: "German: kupfernickel (false coppernickel (goblin)"
  pos_x: 9
  pos_y: 3
- name: Copper
  symbol: Cu
  atomic_number: 29
  atomic_weigth: "63.546"
  density: 8.92-8.96
  melting_point: 1356.55-1356.6
  boiling_point: 2840-2840.15
  atomic_radius: "128"
  covalent_radius: "117"
  ionic_radius: (+2e) 72 (+1e) 96
  atomic_volume: "7.1"
  specific_heat: "0.385"
  fusion_heat: "13.01"
  evaporation_heat: "304.6"
  thermal_conductivity: "401"
  debye_temperature: "315.00"
  pauling_negativity_number: "1.90"
  first_ionizing_energy: "745.0"
  oxidation_states: "3, 2, 1, 0"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#185;"
  lattice_structure: FCC
  lattice_constant: "3.610"
  lattice_ca_ratio: n/a
  appearance: "Malleable, ductile, reddish-brown metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Symbol from Latin: cuprum (island of Cyprus famed for its copper mines)"
  pos_x: 10
  pos_y: 3
- name: Zinc
  symbol: Zn
  atomic_number: 30
  atomic_weigth: "65.38"
  density: "7.133"
  melting_point: 692.73-692.75
  boiling_point: 1179.35-1180
  atomic_radius: "138"
  covalent_radius: "125"
  ionic_radius: (+2e) 74
  atomic_volume: "9.2"
  specific_heat: "0.388"
  fusion_heat: "7.28"
  evaporation_heat: "114.8"
  thermal_conductivity: "116"
  debye_temperature: "234.00"
  pauling_negativity_number: "1.65"
  first_ionizing_energy: "905.8"
  oxidation_states: "2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.660"
  lattice_ca_ratio: n/a
  appearance: "Bluish-silver, ductile metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "German: Zink (German for zinc)"
  pos_x: 11
  pos_y: 3
- name: Gallium
  symbol: Ga
  atomic_number: 31
  atomic_weigth: "69.723"
  density: 5.904-5.91
  melting_point: "302.93"
  boiling_point: "2477"
  atomic_radius: 130-141
  covalent_radius: 122-126
  ionic_radius: (+3e) 62 (+1e) 81
  atomic_volume: "11.8"
  specific_heat: "0.372"
  fusion_heat: "5.59"
  evaporation_heat: "270.3"
  thermal_conductivity: "28.1"
  debye_temperature: "240.00"
  pauling_negativity_number: "1.81"
  first_ionizing_energy: "578.7"
  oxidation_states: "3"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#185;"
  lattice_structure: ORC
  lattice_constant: "4.510"
  lattice_ca_ratio: n/a
  appearance: "Soft, blue-white metal"
  discovery_date: 1875 (France)
  discovered_by: Paul-Emile Lecoq de Boisbaudran
  named_after: "Latin: Gallia (France)"
  pos_x: 12
  pos_y: 3
- name: Germanium
  symbol: Ge
  atomic_number: 32
  atomic_weigth: "72.630"
  density: "5.323"
  melting_point: "1210.6"
  boiling_point: "3103"
  atomic_radius: 122.5-137
  covalent_radius: "122"
  ionic_radius: (+4e)53 (+2e)73
  atomic_volume: "13.6"
  specific_heat: "0.322"
  fusion_heat: "36.8"
  evaporation_heat: "328"
  thermal_conductivity: "60.2"
  debye_temperature: "360.00"
  pauling_negativity_number: "2.01"
  first_ionizing_energy: "760.0"
  oxidation_states: "4, 2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#178;"
  lattice_structure: DIA
  lattice_constant: "5.660"
  lattice_ca_ratio: n/a
  appearance: Grayish-white metal
  discovery_date: 1886 (Germany)
  discovered_by: Clemens Winkler
  named_after: "Latin: Germania (Germany)"
  pos_x: 13
  pos_y: 3
- name: Arsenic
  symbol: As
  atomic_number: 33
  atomic_weigth: "74.92160"
  density: 5.73 (grey arsenic)
  melting_point: "1090"
  boiling_point: 876-886
  atomic_radius: "139"
  covalent_radius: "120"
  ionic_radius: (+5e)46 (-3e)222
  atomic_volume: "13.1"
  specific_heat: "0.328"
  fusion_heat: 24.44 (grey arsenic)
  evaporation_heat: "32.4"
  thermal_conductivity: (50.2)
  debye_temperature: "285.00"
  pauling_negativity_number: "2.18"
  first_ionizing_energy: "946.2"
  oxidation_states: "5, 3, -3"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#179;"
  lattice_structure: TRG
  lattice_constant: "4.130"
  lattice_ca_ratio: n/a
  appearance: "Steel gray, brittle semimetal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Greek: arsenikon; Latin: arsenicum (both names for yellow pigment)"
  pos_x: 14
  pos_y: 3
- name: Selenium
  symbol: Se
  atomic_number: 34
  atomic_weigth: "78.96"
  density: "4.79"
  melting_point: "490"
  boiling_point: "958.1"
  atomic_radius: "140"
  covalent_radius: "116"
  ionic_radius: (+6e)42 (-2e)191
  atomic_volume: "16.5"
  specific_heat: 0.321 (Se-Se)
  fusion_heat: "5.23"
  evaporation_heat: "59.7"
  thermal_conductivity: "0.52"
  debye_temperature: "90"
  pauling_negativity_number: "2.55"
  first_ionizing_energy: "940.4"
  oxidation_states: "6, 4, -2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#8308;"
  lattice_structure: HEX
  lattice_constant: "4.360"
  lattice_ca_ratio: n/a
  appearance: A soft metalloid similar to sulfur
  discovery_date: 1817 (Sweden)
  discovered_by: Jons Jacob Berzelius
  named_after: "Greek: selena (moon)"
  pos_x: 15
  pos_y: 3
- name: Bromine
  symbol: Br
  atomic_number: 35
  atomic_weigth: "79.904"
  density: 3.102-3.12
  melting_point: "265.9"
  boiling_point: "331.9"
  atomic_radius: n/a
  covalent_radius: "114"
  ionic_radius: (+5e)47 (-1e)196
  atomic_volume: "23.5"
  specific_heat: 0.473 (Br-Br)
  fusion_heat: 10.57 (Br-Br)
  evaporation_heat: 29.56 (Br-Br)
  thermal_conductivity: "0.005"
  debye_temperature: n/a
  pauling_negativity_number: "2.96"
  first_ionizing_energy: "1142.0"
  oxidation_states: "7, 5, 3, 1, 0, -1"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#8309;"
  lattice_structure: ORC
  lattice_constant: "6.670"
  lattice_ca_ratio: n/a
  appearance: Reddish-brown liquid
  discovery_date: 1826 (France)
  discovered_by: Antoine J. Balard
  named_after: "Greek: bromos (stench)"
  pos_x: 16
  pos_y: 3
- name: Krypton
  symbol: Kr
  atomic_number: 36
  atomic_weigth: "83.798"
  density: 2.155 (@ -153 °C)
  melting_point: "116.6"
  boiling_point: "120.85"
  atomic_radius: (88)
  covalent_radius: 112-116
  ionic_radius: "169"
  atomic_volume: "32.2"
  specific_heat: "0.247"
  fusion_heat: n/a
  evaporation_heat: "9.05"
  thermal_conductivity: "0.0095"
  debye_temperature: "72"
  pauling_negativity_number: 0.0-3.0
  first_ionizing_energy: "1350.0"
  oxidation_states: "2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#8310;"
  lattice_structure: FCC
  lattice_constant: "5.720"
  lattice_ca_ratio: n/a
  appearance: "Dense, colorless, odorless, and tasteless gas"
  discovery_date: 1898 (Great Britain)
  discovered_by: "Sir William Ramsey, M.W. Travers"
  named_after: "Greek: kryptos (hidden)"
  pos_x: 17
  pos_y: 3
- name: Rubidium
  symbol: Rb
  atomic_number: 37
  atomic_weigth: "85.4678"
  density: "1.532"
  melting_point: "312.2"
  boiling_point: 961-961.15
  atomic_radius: "248"
  covalent_radius: "216"
  ionic_radius: (+1e)147
  atomic_volume: "55.9"
  specific_heat: "0.360"
  fusion_heat: "2.20"
  evaporation_heat: "75.8"
  thermal_conductivity: "58.2"
  debye_temperature: "56"
  pauling_negativity_number: "0.82"
  first_ionizing_energy: "402.8"
  oxidation_states: "1"
  eletronic_configuration: "[Kr] 5s&#185;"
  lattice_structure: BCC
  lattice_constant: "5.590"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white, highly reactive metal"
  discovery_date: 1861 (Germany)
  discovered_by: "Gustov Kirchoff, Robert Bunsen"
  named_after: "Latin: rubidus (deep red); the color its salts impart to flames"
  pos_x: 0
  pos_y: 4
- name: Strontium
  symbol: Sr
  atomic_number: 38
  atomic_weigth: "87.62"
  density: "2.54"
  melting_point: "1042"
  boiling_point: "1657"
  atomic_radius: "215"
  covalent_radius: "191"
  ionic_radius: (+2e) 112
  atomic_volume: "33.7"
  specific_heat: "0.301"
  fusion_heat: "9.20"
  evaporation_heat: "144"
  thermal_conductivity: (35.4)
  debye_temperature: "147"
  pauling_negativity_number: "0.95"
  first_ionizing_energy: "549.0"
  oxidation_states: "2"
  eletronic_configuration: "[Kr] 5s&#178;"
  lattice_structure: FCC
  lattice_constant: "6.080"
  lattice_ca_ratio: n/a
  appearance: "Silvery, malleable metal"
  discovery_date: 1790 (Scotland)
  discovered_by: A. Crawford
  named_after: "The Scottish town, Strontian"
  pos_x: 1
  pos_y: 4
- name: Yttrium
  symbol: Y
  atomic_number: 39
  atomic_weigth: "88.90585"
  density: "4.47"
  melting_point: "1795"
  boiling_point: "3611"
  atomic_radius: "178"
  covalent_radius: "162"
  ionic_radius: (+3e) 89.3
  atomic_volume: "19.8"
  specific_heat: "0.284"
  fusion_heat: "11.5"
  evaporation_heat: "367"
  thermal_conductivity: (17.2)
  debye_temperature: "280"
  pauling_negativity_number: "1.22"
  first_ionizing_energy: "615.4"
  oxidation_states: "3"
  eletronic_configuration: "[Kr] 4d&#185; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.650"
  lattice_ca_ratio: "1.571"
  appearance: "Silvery, ductile, fairly reactive metal"
  discovery_date: 1794 (Finland)
  discovered_by: Johann Gadolin
  named_after: "Named after the Swedish town, Ytterby, where one of its minerals was first found"
  pos_x: 2
  pos_y: 4
- name: Zirconium
  symbol: Zr
  atomic_number: 40
  atomic_weigth: "91.224"
  density: "6.506"
  melting_point: "2125"
  boiling_point: "4650"
  atomic_radius: "160"
  covalent_radius: "145"
  ionic_radius: (+4e)79
  atomic_volume: "14.1"
  specific_heat: "0.281"
  fusion_heat: "19.2"
  evaporation_heat: "567"
  thermal_conductivity: "22.7"
  debye_temperature: 250-291
  pauling_negativity_number: "1.33"
  first_ionizing_energy: "659.7"
  oxidation_states: "0, 1, 2, 3, 4"
  eletronic_configuration: "[Kr] 4d&#178; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.230"
  lattice_ca_ratio: "1.593"
  appearance: "Gray-white, lustrous, corrosion-resistant metal"
  discovery_date: 1789 (Germany)
  discovered_by: Martin Klaproth
  named_after: "The mineral, zircon"
  pos_x: 3
  pos_y: 4
- name: Niobium
  symbol: Nb
  atomic_number: 41
  atomic_weigth: "92.90638"
  density: "8.57"
  melting_point: "2741"
  boiling_point: "5015"
  atomic_radius: "146"
  covalent_radius: 134-164
  ionic_radius: (+5e)69
  atomic_volume: "10.8"
  specific_heat: "0.268"
  fusion_heat: "26.8"
  evaporation_heat: "680"
  thermal_conductivity: "53.7"
  debye_temperature: "275.00"
  pauling_negativity_number: "1.6"
  first_ionizing_energy: "663.6"
  oxidation_states: "5, 4, 3, 2, 1"
  eletronic_configuration: "[Kr] 4d&#8308; 5s&#185;"
  lattice_structure: BCC
  lattice_constant: "3.300"
  lattice_ca_ratio: n/a
  appearance: "Shiny white, soft, ductile metal"
  discovery_date: 1801 (England)
  discovered_by: Charles Hatchet
  named_after: Niobe; daughter of the mythical Greek king Tantalus
  pos_x: 4
  pos_y: 4
- name: Molybdenum
  symbol: Mo
  atomic_number: 42
  atomic_weigth: "95.96"
  density: "10.22"
  melting_point: "2890"
  boiling_point: "4885"
  atomic_radius: "139"
  covalent_radius: "130"
  ionic_radius: " (+6e) 62 (+4e) 70"
  atomic_volume: "9.4"
  specific_heat: "0.251"
  fusion_heat: "28"
  evaporation_heat: ~590
  thermal_conductivity: (138)
  debye_temperature: 380.00-450.00
  pauling_negativity_number: "2.16"
  first_ionizing_energy: "684.8"
  oxidation_states: "6, 5, 4, 3, 2, 0"
  eletronic_configuration: "[Kr] 4d&#8309; 5s&#185;"
  lattice_structure: BCC
  lattice_constant: "3.150"
  lattice_ca_ratio: n/a
  appearance: "Silvery white, hard metal"
  discovery_date: 1778 (Sweden)
  discovered_by: Carl Wilhelm Scheele
  named_after: "Greek: molybdos (lead)"
  pos_x: 5
  pos_y: 4
- name: Technetium
  symbol: Tc
  atomic_number: 43
  atomic_weigth: "97.9072"
  density: "11.5"
  melting_point: 2430-2445
  boiling_point: 4538-5150
  atomic_radius: "136"
  covalent_radius: "127"
  ionic_radius: (+7e)56
  atomic_volume: "8.5"
  specific_heat: "0.243"
  fusion_heat: "23.8"
  evaporation_heat: "585"
  thermal_conductivity: "50.6"
  debye_temperature: "453"
  pauling_negativity_number: "1.9"
  first_ionizing_energy: "702.2"
  oxidation_states: "-1, 0, 1, 2, 3, 4, 5, 6, 7"
  eletronic_configuration: "[Kr] 4d&#8309; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.740"
  lattice_ca_ratio: "1.604"
  appearance: Silvery-gray metal
  discovery_date: 1937 (Italy)
  discovered_by: "Carlo Perrier, Emilio Segre"
  named_after: "Greek: technetos (artificial)"
  pos_x: 6
  pos_y: 4
- name: Ruthenium
  symbol: Ru
  atomic_number: 44
  atomic_weigth: "101.07"
  density: "12.41"
  melting_point: 2583-2607.15
  boiling_point: 4173-4350.15
  atomic_radius: "134"
  covalent_radius: "125"
  ionic_radius: (+4e) 67
  atomic_volume: "8.3"
  specific_heat: "0.238"
  fusion_heat: (25.5)
  evaporation_heat: n/a
  thermal_conductivity: "117.0"
  debye_temperature: "600"
  pauling_negativity_number: "2.2"
  first_ionizing_energy: "710.3"
  oxidation_states: "2, 3, 4, 6, 8, 0, -2"
  eletronic_configuration: "[Kr] 4d&#8311; 5s&#185;"
  lattice_structure: HEX
  lattice_constant: "2.700"
  lattice_ca_ratio: "1.584"
  appearance: "Rare, silver-gray, extremely brittle metal"
  discovery_date: 1844 (Russia)
  discovered_by: Karl Klaus
  named_after: "Latin: Ruthenia (Russia)"
  pos_x: 7
  pos_y: 4
- name: Rhodium
  symbol: Rh
  atomic_number: 45
  atomic_weigth: "102.9055"
  density: "12.41"
  melting_point: "2239"
  boiling_point: "4000"
  atomic_radius: "134"
  covalent_radius: "125"
  ionic_radius: (+3e)68
  atomic_volume: "8.3"
  specific_heat: "0.244"
  fusion_heat: "21.8"
  evaporation_heat: "494"
  thermal_conductivity: "150"
  debye_temperature: "480"
  pauling_negativity_number: "2.28"
  first_ionizing_energy: "719.5"
  oxidation_states: "5, 4, 3,  2, 1, 0"
  eletronic_configuration: "[Kr] 4d&#8312; 5s&#185;"
  lattice_structure: FCC
  lattice_constant: "3.800"
  lattice_ca_ratio: n/a
  appearance: "Silvery white, hard metal"
  discovery_date: 1803 (England)
  discovered_by: William Wollaston
  named_after: "Greek: rhodon (rose); its salts give a rosy solution"
  pos_x: 8
  pos_y: 4
- name: Palladium
  symbol: Pd
  atomic_number: 46
  atomic_weigth: "106.42"
  density: "12.02"
  melting_point: 1825-1827
  boiling_point: 2940-3413
  atomic_radius: "137"
  covalent_radius: "128"
  ionic_radius: (+4e) 65 (+2e) 80
  atomic_volume: "8.9"
  specific_heat: "0.244"
  fusion_heat: "17.24"
  evaporation_heat: "372.4"
  thermal_conductivity: "71.8"
  debye_temperature: 274.00-275.00
  pauling_negativity_number: "2.20"
  first_ionizing_energy: "803.5"
  oxidation_states: "0, 1, 2, 3, 4, 5, 6"
  eletronic_configuration: "[Kr] 4d&#185;&#8304;"
  lattice_structure: FCC
  lattice_constant: "3.890"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, soft, malleable and ductile metal"
  discovery_date: 1803 (England)
  discovered_by: William Wollaston
  named_after: "Named after the asteroid, Pallas, discovered in 1803"
  pos_x: 9
  pos_y: 4
- name: Silver
  symbol: Ag
  atomic_number: 47
  atomic_weigth: "107.8682"
  density: "10.5"
  melting_point: "1235.1"
  boiling_point: "2485"
  atomic_radius: "144"
  covalent_radius: "134"
  ionic_radius: (+2e) 89 (+1e) 126
  atomic_volume: "10.3"
  specific_heat: "0.237"
  fusion_heat: "11.95"
  evaporation_heat: "254.1"
  thermal_conductivity: "429"
  debye_temperature: 215.00-225.00
  pauling_negativity_number: "1.93"
  first_ionizing_energy: 730.5; 2070; 3361
  oxidation_states: "2, 1"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#185;"
  lattice_structure: FCC
  lattice_constant: "4.090"
  lattice_ca_ratio: n/a
  appearance: "Silvery-ductile, and malleable metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: siolful (silver); symbol from Latin: argentum"
  pos_x: 10
  pos_y: 4
- name: Cadmium
  symbol: Cd
  atomic_number: 48
  atomic_weigth: "112.411"
  density: "8.65"
  melting_point: 594.1-594.22
  boiling_point: 1038-1040
  atomic_radius: 154-161
  covalent_radius: "148"
  ionic_radius: (+2e) 97
  atomic_volume: "13.1"
  specific_heat: "0.232"
  fusion_heat: "6.11"
  evaporation_heat: 59.1-100
  thermal_conductivity: "96.9"
  debye_temperature: "209.00"
  pauling_negativity_number: "1.69"
  first_ionizing_energy: 867.2-867.8
  oxidation_states: "2"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.980"
  lattice_ca_ratio: "1.886"
  appearance: "Soft, malleable, blue-white metal"
  discovery_date: 1817 (Germany)
  discovered_by: Fredrich Stromeyer
  named_after: "Greek: kadmia (ancient name for calamine (zinc oxide))"
  pos_x: 11
  pos_y: 4
- name: Indium
  symbol: In
  atomic_number: 49
  atomic_weigth: "114.818"
  density: "7.31"
  melting_point: "429.32"
  boiling_point: "2353"
  atomic_radius: "166"
  covalent_radius: "144"
  ionic_radius: (+3e) 81
  atomic_volume: "15.7"
  specific_heat: "0.234"
  fusion_heat: "3.24"
  evaporation_heat: "225.1"
  thermal_conductivity: "81.8"
  debye_temperature: "129.00"
  pauling_negativity_number: "1.78"
  first_ionizing_energy: "558.0"
  oxidation_states: "1, 3"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#185;"
  lattice_structure: TET
  lattice_constant: "4.590"
  lattice_ca_ratio: n/a
  appearance: "Very soft, silvery-white metal"
  discovery_date: 1863 (Germany)
  discovered_by: "Ferdinand Reich, T. Richter"
  named_after: "Latin: indicum (color indigo), the color it shows in a spectroscope"
  pos_x: 12
  pos_y: 4
- name: Tin
  symbol: Sn
  atomic_number: 50
  atomic_weigth: "118.71"
  density: "7.31"
  melting_point: 505.05-505.1
  boiling_point: 2543-2873
  atomic_radius: "162"
  covalent_radius: "141"
  ionic_radius: (+4e) 71 (+2) 93
  atomic_volume: "16.3"
  specific_heat: "0.222"
  fusion_heat: "7.07"
  evaporation_heat: "296"
  thermal_conductivity: "66.8"
  debye_temperature: "170.00"
  pauling_negativity_number: "1.96"
  first_ionizing_energy: "708.2"
  oxidation_states: "4, 2"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#178;"
  lattice_structure: TET
  lattice_constant: "5.820"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, soft, malleable and ductile metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Named after Etruscan god, Tinia; symbol from Latin: stannum (tin)"
  pos_x: 13
  pos_y: 4
- name: Antimony
  symbol: Sb
  atomic_number: 51
  atomic_weigth: "121.760"
  density: "6.691"
  melting_point: "903.9"
  boiling_point: "1908"
  atomic_radius: "159"
  covalent_radius: "140"
  ionic_radius: (+6e)62 (−3e)245
  atomic_volume: "18.4"
  specific_heat: "0.205"
  fusion_heat: "20.08"
  evaporation_heat: "195.2"
  thermal_conductivity: "24.43"
  debye_temperature: "200.00"
  pauling_negativity_number: "2.05"
  first_ionizing_energy: "833.3"
  oxidation_states: "5, 3, -3"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#179;"
  lattice_structure: TRG
  lattice_constant: "4.510"
  lattice_ca_ratio: n/a
  appearance: "Hard, silvery-white, brittle semimetal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Greek: anti and monos (not alone); symbol from mineral stibnite"
  pos_x: 14
  pos_y: 4
- name: Tellurium
  symbol: Te
  atomic_number: 52
  atomic_weigth: "127.6"
  density: "6.24"
  melting_point: "722.7"
  boiling_point: "1263"
  atomic_radius: "160"
  covalent_radius: "136"
  ionic_radius: (+6e) 56 211 (−2e)
  atomic_volume: "20.5"
  specific_heat: "0.201"
  fusion_heat: "17.91"
  evaporation_heat: "49.8"
  thermal_conductivity: "14.3"
  debye_temperature: n/a
  pauling_negativity_number: "2.1"
  first_ionizing_energy: "869.0"
  oxidation_states: "6, 4, 2, -2"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#8308;"
  lattice_structure: HEX
  lattice_constant: "4.450"
  lattice_ca_ratio: "1.330"
  appearance: "Silvery-white, brittle semimetal"
  discovery_date: 1782 (Romania)
  discovered_by: Franz Joseph Meller von Reichenstein
  named_after: "Latin: tellus, telluris (Planet Earth)"
  pos_x: 15
  pos_y: 4
- name: Iodine
  symbol: I
  atomic_number: 53
  atomic_weigth: "126.90447"
  density: "4.93"
  melting_point: 386.65-386.85
  boiling_point: 457.4-457.5
  atomic_radius: "136"
  covalent_radius: "133"
  ionic_radius: (+7e) 50 (-1e) 220
  atomic_volume: "25.7"
  specific_heat: 0.427 (I-I)
  fusion_heat: 15.52 (I-I)
  evaporation_heat: 41.95 (I-I)
  thermal_conductivity: (0.45)
  debye_temperature: n/a
  pauling_negativity_number: "2.66"
  first_ionizing_energy: "1008.3"
  oxidation_states: "7, 5, 3, 1, 0, -1"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#8309;"
  lattice_structure: ORC
  lattice_constant: "7.720"
  lattice_ca_ratio: n/a
  appearance: "Shiny, black nonmetallic solid"
  discovery_date: 1811 (France)
  discovered_by: Bernard Courtois
  named_after: "Greek: iodes (violet colored)"
  pos_x: 16
  pos_y: 4
- name: Xenon
  symbol: Xe
  atomic_number: 54
  atomic_weigth: "131.29"
  density: 3.52 (@ -107.05 °C)
  melting_point: "161.3"
  boiling_point: "166.1"
  atomic_radius: (108)
  covalent_radius: 131-140
  ionic_radius: "190"
  atomic_volume: "42.9"
  specific_heat: "0.158"
  fusion_heat: "2.27"
  evaporation_heat: "12.65"
  thermal_conductivity: "0.0057"
  debye_temperature: n/a
  pauling_negativity_number: 0.0-2.6
  first_ionizing_energy: "1170.0"
  oxidation_states: "0, 1, 2, 4, 6, 7, 8"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#8310;"
  lattice_structure: FCC
  lattice_constant: "6.200"
  lattice_ca_ratio: n/a
  appearance: "Heavy, colorless, and odorless noble gas"
  discovery_date: 1898 (England)
  discovered_by: "Sir William Ramsey, M.W. Travers"
  named_after: "Greek: xenos (strange)"
  pos_x: 17
  pos_y: 4
- name: Cesium
  symbol: Cs
  atomic_number: 55
  atomic_weigth: "132.90545"
  density: "1.873"
  melting_point: 301.6-301.85
  boiling_point: 940.75-951.6
  atomic_radius: "267"
  covalent_radius: "235"
  ionic_radius: (+1e) 167
  atomic_volume: "70.0"
  specific_heat: "0.241"
  fusion_heat: "2.09"
  evaporation_heat: "68.3"
  thermal_conductivity: "35.9"
  debye_temperature: "39.2"
  pauling_negativity_number: "0.79"
  first_ionizing_energy: "375.5"
  oxidation_states: "1"
  eletronic_configuration: "[Xe] 6s&#185;"
  lattice_structure: BCC
  lattice_constant: "6.050"
  lattice_ca_ratio: n/a
  appearance: "Very soft, ductile, light gray metal"
  discovery_date: 1860 (Germany)
  discovered_by: "Gustov Kirchoff, Robert Bunsen"
  named_after: "Latin: caesius (sky blue); for the blue lines of its spectrum"
  pos_x: 0
  pos_y: 5
- name: Barium
  symbol: Ba
  atomic_number: 56
  atomic_weigth: "137.327"
  density: "3.5"
  melting_point: "1002"
  boiling_point: "1910"
  atomic_radius: "222"
  covalent_radius: "198"
  ionic_radius: (+2e) 134
  atomic_volume: "39.0"
  specific_heat: "0.192"
  fusion_heat: "7.66"
  evaporation_heat: "142.0"
  thermal_conductivity: (18.4)
  debye_temperature: n/a
  pauling_negativity_number: "0.89"
  first_ionizing_energy: "502.5"
  oxidation_states: "2"
  eletronic_configuration: "[Xe] 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "5.020"
  lattice_ca_ratio: n/a
  appearance: "Soft, slightly malleable, silver-white metal"
  discovery_date: 1808 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Greek: barys (heavy or dense)"
  pos_x: 1
  pos_y: 5
- name: Lanthanum
  symbol: La
  atomic_number: 57
  atomic_weigth: "138.9055"
  density: 6.15-6.18
  melting_point: 920-1194
  boiling_point: 3447-3730
  atomic_radius: "187"
  covalent_radius: "169"
  ionic_radius: 101.(+3e) 6
  atomic_volume: "22.5"
  specific_heat: "0.197"
  fusion_heat: "8.5"
  evaporation_heat: "402"
  thermal_conductivity: "13.4"
  debye_temperature: "132.00"
  pauling_negativity_number: "1.10"
  first_ionizing_energy: "541.1"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 5d&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.750"
  lattice_ca_ratio: "1.619"
  appearance: "Silvery-white, soft, malleable, and ductile metal"
  discovery_date: 1839 (Sweden)
  discovered_by: Carl Mosander
  named_after: "Greek: lanthanein (to be hidden)"
  pos_x: 3
  pos_y: 7
- name: Cerium
  symbol: Ce
  atomic_number: 58
  atomic_weigth: "140.116"
  density: "6.757"
  melting_point: "1072"
  boiling_point: "3699"
  atomic_radius: "181"
  covalent_radius: "165"
  ionic_radius: (+4e) 92 103.(+3e) 4
  atomic_volume: "21.0"
  specific_heat: "0.205"
  fusion_heat: "5.2"
  evaporation_heat: "398"
  thermal_conductivity: "11.3"
  debye_temperature: n/a
  pauling_negativity_number: "1.12"
  first_ionizing_energy: "540.1"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Xe] 4f&#185; 5d&#185; 6s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.160"
  lattice_ca_ratio: n/a
  appearance: "Malleable, ductile, iron-gray metal"
  discovery_date: 1803 (Sweden/Germany)
  discovered_by: "Wilhelm von Hisinger, Jons Jacob Berzelius, Martin Klaproth"
  named_after: "Named after the asteroid, Ceres, discovered two years before the element"
  pos_x: 4
  pos_y: 7
- name: Praseodymium
  symbol: Pr
  atomic_number: 59
  atomic_weigth: "140.90765"
  density: "6.773"
  melting_point: "1204"
  boiling_point: "3785"
  atomic_radius: "182"
  covalent_radius: "165"
  ionic_radius: (+4e) 90 101.(+3e) 3
  atomic_volume: "20.8"
  specific_heat: "0.192"
  fusion_heat: "11.3"
  evaporation_heat: "331"
  thermal_conductivity: "12.5"
  debye_temperature: n/a
  pauling_negativity_number: "1.13"
  first_ionizing_energy: "526.6"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Xe] 4f&#179; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.670"
  lattice_ca_ratio: "3.22"
  appearance: "Silvery white, moderately soft, malleable, and ductile metal"
  discovery_date: 1885 (Austria)
  discovered_by: C.F. Aver von Welsbach
  named_after: "Greek: prasios and didymos (green twin); from its green salts"
  pos_x: 5
  pos_y: 7
- name: Neodymium
  symbol: Nd
  atomic_number: 60
  atomic_weigth: "144.242"
  density: "7.007"
  melting_point: "1294"
  boiling_point: "3341"
  atomic_radius: "182"
  covalent_radius: "184"
  ionic_radius: 99.(+3e) 5
  atomic_volume: "20.6"
  specific_heat: "0.205"
  fusion_heat: "7.1"
  evaporation_heat: "289"
  thermal_conductivity: (16.5)
  debye_temperature: n/a
  pauling_negativity_number: "1.14"
  first_ionizing_energy: "531.5"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#8308; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.660"
  lattice_ca_ratio: "1.614"
  appearance: "Silvery-white, rare-earth metal that oxidizes easily in air"
  discovery_date: 1925 (Austria)
  discovered_by: C.F. Aver von Welsbach
  named_after: "Greek: neos and didymos (new twin)"
  pos_x: 6
  pos_y: 7
- name: Promethium
  symbol: Pm
  atomic_number: 61
  atomic_weigth: "144.9127"
  density: 7.2-7.26
  melting_point: "1441"
  boiling_point: 3000-~3273
  atomic_radius: 183-185(205)
  covalent_radius: "199"
  ionic_radius: (+3e)97.9 or (+3e)111
  atomic_volume: "19.96"
  specific_heat: "0.185"
  fusion_heat: "7.13"
  evaporation_heat: "330.5"
  thermal_conductivity: "17.9"
  debye_temperature: n/a
  pauling_negativity_number: 1.1-1.13
  first_ionizing_energy: "536"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#8309; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Light-grey, radioactive element"
  discovery_date: 1945 (United States)
  discovered_by: "J.A. Marinsky, L.E. Glendenin, C.D. Coryell"
  named_after: "Named for the Greek god, Prometheus"
  pos_x: 7
  pos_y: 7
- name: Samarium
  symbol: Sm
  atomic_number: 62
  atomic_weigth: "150.36"
  density: "7.520"
  melting_point: "1350"
  boiling_point: "2064"
  atomic_radius: "181"
  covalent_radius: "162"
  ionic_radius: (+3e) 96.4
  atomic_volume: "19.9"
  specific_heat: "0.180"
  fusion_heat: "8.9"
  evaporation_heat: "165"
  thermal_conductivity: (13.3)
  debye_temperature: "166.00"
  pauling_negativity_number: "1.17"
  first_ionizing_energy: "540.1"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#8310; 6s&#178;"
  lattice_structure: RHL
  lattice_constant: "9.000"
  lattice_ca_ratio: n/a
  appearance: Silvery rare-earth metal
  discovery_date: 1880 (France)
  discovered_by: Jean Charles Galissard de Marignac
  named_after: Named after the mineral samarskite
  pos_x: 8
  pos_y: 7
- name: Europium
  symbol: Eu
  atomic_number: 63
  atomic_weigth: "151.964"
  density: "5.243"
  melting_point: "1095"
  boiling_point: "1870"
  atomic_radius: "199"
  covalent_radius: "185"
  ionic_radius: (+3e) 95 (+2e) 109
  atomic_volume: "28.9"
  specific_heat: "0.176"
  fusion_heat: "1095"
  evaporation_heat: "176"
  thermal_conductivity: "13.9"
  debye_temperature: n/a
  pauling_negativity_number: 0.0-1.2
  first_ionizing_energy: "546.9"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#8311; 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "4.610"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white metal"
  discovery_date: 1901 (France)
  discovered_by: Eugene-Antole Demarcay
  named_after: Named for the continent of Europe
  pos_x: 9
  pos_y: 7
- name: Gadolinium
  symbol: Gd
  atomic_number: 64
  atomic_weigth: "157.25"
  density: "7.900"
  melting_point: "1586"
  boiling_point: "3539"
  atomic_radius: "179"
  covalent_radius: "161"
  ionic_radius: (+3e) 93.8
  atomic_volume: "19.9"
  specific_heat: "0.230"
  fusion_heat: "10.0"
  evaporation_heat: "398"
  thermal_conductivity: (10.5)
  debye_temperature: n/a
  pauling_negativity_number: "1.20"
  first_ionizing_energy: "594.2"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#8311; 5d&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.640"
  lattice_ca_ratio: "1.588"
  appearance: "Soft, ductile, silvery-white metal"
  discovery_date: 1880 (Switzerland)
  discovered_by: Jean Charles Galissard de Marignac
  named_after: Named after the mineral gadolinite
  pos_x: 10
  pos_y: 7
- name: Terbium
  symbol: Tb
  atomic_number: 65
  atomic_weigth: "158.92535"
  density: "8.229"
  melting_point: "1629"
  boiling_point: "3296"
  atomic_radius: "180"
  covalent_radius: "159"
  ionic_radius: (+4e) 84 (+3e) 92.3
  atomic_volume: "19.2"
  specific_heat: "0.183"
  fusion_heat: n/a
  evaporation_heat: "389"
  thermal_conductivity: "11.1"
  debye_temperature: n/a
  pauling_negativity_number: "1.2"
  first_ionizing_energy: "569"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Xe] 4f&#8313; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.600"
  lattice_ca_ratio: "1.581"
  appearance: "Soft, ductile, silvery-gray, rare-earth metal"
  discovery_date: 1843 (Sweden)
  discovered_by: Carl Mosander
  named_after: "Named after the Swedish town, Ytterby"
  pos_x: 11
  pos_y: 7
- name: Dysprosium
  symbol: Dy
  atomic_number: 66
  atomic_weigth: "162.50"
  density: "8.55"
  melting_point: "1685"
  boiling_point: "2835"
  atomic_radius: "180"
  covalent_radius: "159"
  ionic_radius: (+3e) 90.8
  atomic_volume: "19.0"
  specific_heat: "0.173"
  fusion_heat: n/a
  evaporation_heat: "291"
  thermal_conductivity: "10.7"
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: "567"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#8304; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.590"
  lattice_ca_ratio: "1.573"
  appearance: "Soft. lustrous, silvery metal"
  discovery_date: 1886 (France)
  discovered_by: Paul-Emile Lecoq de Boisbaudran
  named_after: "Greek: dysprositos (hard to get at)"
  pos_x: 12
  pos_y: 7
- name: Holmium
  symbol: Ho
  atomic_number: 67
  atomic_weigth: "164.93032"
  density: "8.795"
  melting_point: "1747"
  boiling_point: "2968"
  atomic_radius: "179"
  covalent_radius: "158"
  ionic_radius: (+3e) 89.4
  atomic_volume: "18.7"
  specific_heat: "0.164"
  fusion_heat: n/a
  evaporation_heat: "301"
  thermal_conductivity: (16.2)
  debye_temperature: n/a
  pauling_negativity_number: "1.23"
  first_ionizing_energy: "574"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.580"
  lattice_ca_ratio: "1.570"
  appearance: "Fairly soft, malleable, lustrous, silvery metal"
  discovery_date: 1879 (Switzerland)
  discovered_by: J.L. Soret
  named_after: "Holmia, the Latinized name for Stockholm, Sweden"
  pos_x: 13
  pos_y: 7
- name: Erbium
  symbol: Er
  atomic_number: 68
  atomic_weigth: "167.259"
  density: "9.06"
  melting_point: "1802"
  boiling_point: "3136"
  atomic_radius: "178"
  covalent_radius: "157"
  ionic_radius: (+3e) 88.1
  atomic_volume: "18.4"
  specific_heat: "0.168"
  fusion_heat: n/a
  evaporation_heat: "317"
  thermal_conductivity: (14.5)
  debye_temperature: n/a
  pauling_negativity_number: "1.24"
  first_ionizing_energy: "581"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#178; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.560"
  lattice_ca_ratio: "1.570"
  appearance: "Soft, malleable, silvery metal"
  discovery_date: 1843 (Sweden)
  discovered_by: Carl Mosander
  named_after: "Named after the Swedish town, Ytterby"
  pos_x: 14
  pos_y: 7
- name: Thulium
  symbol: Tm
  atomic_number: 69
  atomic_weigth: "168.93421"
  density: "9.321"
  melting_point: "1818"
  boiling_point: "2220"
  atomic_radius: "177"
  covalent_radius: "156"
  ionic_radius: (+3e) 87
  atomic_volume: "18.1"
  specific_heat: "0.160"
  fusion_heat: n/a
  evaporation_heat: "232"
  thermal_conductivity: (16.9)
  debye_temperature: n/a
  pauling_negativity_number: "1.25"
  first_ionizing_energy: "589"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#185;&#179; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.540"
  lattice_ca_ratio: "1.570"
  appearance: "Soft, malleable, ductile, silvery metal"
  discovery_date: 1879 (Sweden)
  discovered_by: Per Theodor Cleve
  named_after: "Thule, ancient name of Scandinavia"
  pos_x: 15
  pos_y: 7
- name: Ytterbium
  symbol: Yb
  atomic_number: 70
  atomic_weigth: "173.055"
  density: "6.9654"
  melting_point: "1097"
  boiling_point: "1466"
  atomic_radius: "194"
  covalent_radius: "170"
  ionic_radius: (+3e) 85.8 (+2e) 93
  atomic_volume: "24.8"
  specific_heat: "0.145"
  fusion_heat: "3.35"
  evaporation_heat: "159"
  thermal_conductivity: (34.9)
  debye_temperature: n/a
  pauling_negativity_number: "1.1"
  first_ionizing_energy: "603"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 6s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.490"
  lattice_ca_ratio: n/a
  appearance: "Silvery, lustrous, malleable, and ductile metal"
  discovery_date: 1878 (Switzerland)
  discovered_by: Jean Charles Galissard de Marignac
  named_after: "Named after the Swedish town, Ytterby"
  pos_x: 16
  pos_y: 7
- name: Lutetium
  symbol: Lu
  atomic_number: 71
  atomic_weigth: "174.9668"
  density: "9.8404"
  melting_point: "1936"
  boiling_point: "3668"
  atomic_radius: "175"
  covalent_radius: "156"
  ionic_radius: (+3e) 85
  atomic_volume: "17.8"
  specific_heat: "0.155"
  fusion_heat: n/a
  evaporation_heat: "414"
  thermal_conductivity: (16.4)
  debye_temperature: n/a
  pauling_negativity_number: "1.27"
  first_ionizing_energy: "513"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.510"
  lattice_ca_ratio: "1.585"
  appearance: "Silvery-white, hard, dense, rare-earth metal"
  discovery_date: 1907 (France)
  discovered_by: Georges Urbain
  named_after: "Named for the ancient name of Paris: Lutetia Parisorum"
  pos_x: 17
  pos_y: 7
- name: Hafnium
  symbol: Hf
  atomic_number: 72
  atomic_weigth: "178.49"
  density: "13.31"
  melting_point: 2503-2506
  boiling_point: 4876-5470
  atomic_radius: "167"
  covalent_radius: "144"
  ionic_radius: (+4e) 78
  atomic_volume: "13.6"
  specific_heat: "0.146"
  fusion_heat: (25.1)
  evaporation_heat: "575"
  thermal_conductivity: "23.0"
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "575.2"
  oxidation_states: "4"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#178; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.200"
  lattice_ca_ratio: "1.582"
  appearance: "Silvery, ductile metal"
  discovery_date: 1923 (Denmark)
  discovered_by: "Dirk Coster, Georg von Hevesy"
  named_after: "Hafnia, the Latin name of Copenhagen"
  pos_x: 3
  pos_y: 5
- name: Tantalum
  symbol: Ta
  atomic_number: 73
  atomic_weigth: "180.9479"
  density: 16.65-16.654
  melting_point: 3269-3290
  boiling_point: 5698-5731
  atomic_radius: "149"
  covalent_radius: "134"
  ionic_radius: (+5e) 68
  atomic_volume: "10.9"
  specific_heat: "0.140"
  fusion_heat: "24.7"
  evaporation_heat: "758"
  thermal_conductivity: "57.5"
  debye_temperature: "225.00"
  pauling_negativity_number: "1.5"
  first_ionizing_energy: "760.1"
  oxidation_states: "5"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#179; 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "3.310"
  lattice_ca_ratio: n/a
  appearance: "Gray, heavy, hard metal"
  discovery_date: 1802 (Sweden)
  discovered_by: Anders Ekeberg
  named_after: "King Tantalus of Greek mythology, father of Niobe"
  pos_x: 4
  pos_y: 5
- name: Tungsten
  symbol: W
  atomic_number: 74
  atomic_weigth: "183.84"
  density: 19.25-19.3
  melting_point: 3680-3695
  boiling_point: 5828-5930
  atomic_radius: "141"
  covalent_radius: 130-170
  ionic_radius: (+6e) 62 (+4e) 70
  atomic_volume: "9.53"
  specific_heat: "0.133"
  fusion_heat: "35"
  evaporation_heat: "824"
  thermal_conductivity: "173"
  debye_temperature: "310.00"
  pauling_negativity_number: 1.7-2.3
  first_ionizing_energy: "769.7"
  oxidation_states: "6, 5, 4, 3, 2, 0"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8308; 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "3.160"
  lattice_ca_ratio: n/a
  appearance: "Tough, steel-gray to white metal"
  discovery_date: 1783 (Spain)
  discovered_by: "Juan Jose, Fausto Elhuyar"
  named_after: "Swedish: tung sten (heavy stone); symbol from its German name wolfram"
  pos_x: 5
  pos_y: 5
- name: Rhenium
  symbol: Re
  atomic_number: 75
  atomic_weigth: "186.207"
  density: "21.02"
  melting_point: 3453-3459
  boiling_point: 5869-5900
  atomic_radius: "137"
  covalent_radius: "128"
  ionic_radius: (+7e) 53 (+4e) 72
  atomic_volume: "8.85"
  specific_heat: "0.138"
  fusion_heat: "34"
  evaporation_heat: "704"
  thermal_conductivity: "48.0"
  debye_temperature: "416.00"
  pauling_negativity_number: "1.9"
  first_ionizing_energy: "759.1"
  oxidation_states: "7, 6, 5, 4, 3, 2, -1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8309; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.760"
  lattice_ca_ratio: "1.615"
  appearance: "Dense, silvery-white metal"
  discovery_date: 1925 (Germany)
  discovered_by: "Walter Noddack, Ida Tacke, Otto Berg"
  named_after: "Latin: Rhenus, the Rhine River"
  pos_x: 6
  pos_y: 5
- name: Osmium
  symbol: Os
  atomic_number: 76
  atomic_weigth: "190.23"
  density: 22.57-22.61
  melting_point: 3306-3327
  boiling_point: 5285-5300
  atomic_radius: "135"
  covalent_radius: "126"
  ionic_radius: (+6e) 69 (+4e) 88
  atomic_volume: "8.43"
  specific_heat: "0.131"
  fusion_heat: "31.7"
  evaporation_heat: "738"
  thermal_conductivity: (87.6)
  debye_temperature: n/a
  pauling_negativity_number: "2.2"
  first_ionizing_energy: "819.8"
  oxidation_states: "8, 6, 4, 3, 2, 0, -2"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8310; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.740"
  lattice_ca_ratio: "1.579"
  appearance: "Blue-white, lustrous, hard metal"
  discovery_date: 1804 (England)
  discovered_by: Smithson Tenant
  named_after: "Greek: osme (odor)"
  pos_x: 7
  pos_y: 5
- name: Iridium
  symbol: Ir
  atomic_number: 77
  atomic_weigth: "192.217"
  density: 22.42-22.65
  melting_point: 2683-2739
  boiling_point: 4403-4701
  atomic_radius: "136"
  covalent_radius: "127"
  ionic_radius: (+4e) 68
  atomic_volume: "8.54"
  specific_heat: "0.133"
  fusion_heat: 26.0-27.61
  evaporation_heat: 604-610
  thermal_conductivity: "147"
  debye_temperature: "430.00"
  pauling_negativity_number: "2.20"
  first_ionizing_energy: "868.1"
  oxidation_states: "6, 4, 3, 2, 1, 0, -1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8311; 6s&#178;"
  lattice_structure: FCC
  lattice_constant: "3.840"
  lattice_ca_ratio: n/a
  appearance: "White, brittle metal"
  discovery_date: 1804 (England/France)
  discovered_by: "S.Tenant, A.F.Fourcroy, L.N.Vauquelin, H.V.Collet-Descoltils"
  named_after: "Greek: iris (rainbow)"
  pos_x: 8
  pos_y: 5
- name: Platinum
  symbol: Pt
  atomic_number: 78
  atomic_weigth: "195.08"
  density: 21.09-21.45
  melting_point: 2041.4-2045
  boiling_point: 4098-4100
  atomic_radius: "139"
  covalent_radius: "130"
  ionic_radius: (+4e) 65 (+2e) 80
  atomic_volume: "9.10"
  specific_heat: "0.133"
  fusion_heat: "21.76"
  evaporation_heat: ~470
  thermal_conductivity: "71.6"
  debye_temperature: "230.00"
  pauling_negativity_number: "2.28"
  first_ionizing_energy: "868.1"
  oxidation_states: "4, 2, 0"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8313; 6s&#185;"
  lattice_structure: FCC
  lattice_constant: "3.920"
  lattice_ca_ratio: n/a
  appearance: "Very heavy, soft, silvery-white metal"
  discovery_date: "1557 (Italy), but Incas use before"
  discovered_by: Julius Scaliger
  named_after: "Spanish: platina (little silver)"
  pos_x: 9
  pos_y: 5
- name: Gold
  symbol: Au
  atomic_number: 79
  atomic_weigth: "196.96657"
  density: "19.3"
  melting_point: 1337.33-1337.58
  boiling_point: 3080-3129
  atomic_radius: 144-146
  covalent_radius: "134"
  ionic_radius: (-3e) 185 (+1e) 137 or (+3e) 85 (+1e) 137
  atomic_volume: "10.2"
  specific_heat: "0.129"
  fusion_heat: "12.68"
  evaporation_heat: ~340
  thermal_conductivity: "318"
  debye_temperature: "170.00"
  pauling_negativity_number: 2.54-2.64
  first_ionizing_energy: "889.3"
  oxidation_states: "-1, 1, 3, 5"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#185;/[Xe] 5p&#8310; 5d&#185;&#8304; 6s&#185;"
  lattice_structure: FCC
  lattice_constant: "4.080"
  lattice_ca_ratio: n/a
  appearance: "Soft, malleable, yellow metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: geolo (yellow); symbol from Latin: aurum (shining dawn)"
  pos_x: 10
  pos_y: 5
- name: Mercury
  symbol: Hg
  atomic_number: 80
  atomic_weigth: "200.59"
  density: 13.546 (@ +20 °C)
  melting_point: 234.28-234.32
  boiling_point: 629.73-629.88
  atomic_radius: "157"
  covalent_radius: "149"
  ionic_radius: (+2e) 110 (+1e) 127
  atomic_volume: "14.8"
  specific_heat: "0.138"
  fusion_heat: "2.295"
  evaporation_heat: "58.5"
  thermal_conductivity: "8.3"
  debye_temperature: "100.00"
  pauling_negativity_number: "2.00"
  first_ionizing_energy: "1006.0"
  oxidation_states: "2, 1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178;"
  lattice_structure: RHL
  lattice_constant: "2.990"
  lattice_ca_ratio: "1.94"
  appearance: "Heavy, silver-white metal that is in its liquid state at"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "The Roman god Mercury; symbol from Latin: hydrargyrum (liquid silver)"
  pos_x: 11
  pos_y: 5
- name: Thallium
  symbol: Tl
  atomic_number: 81
  atomic_weigth: "204.3833"
  density: "11.85"
  melting_point: 576.6-577
  boiling_point: 1730-1746
  atomic_radius: "171"
  covalent_radius: "148"
  ionic_radius: (+3e) 95 (+1e) 147
  atomic_volume: "17.2"
  specific_heat: "0.128"
  fusion_heat: "4.31"
  evaporation_heat: "162.4"
  thermal_conductivity: "46.1"
  debye_temperature: "96.00"
  pauling_negativity_number: "1.62"
  first_ionizing_energy: "588.9"
  oxidation_states: "3, 1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#185;"
  lattice_structure: HEX
  lattice_constant: "3.460"
  lattice_ca_ratio: "1.599"
  appearance: "Soft, gray metal"
  discovery_date: 1861 (England)
  discovered_by: Sir William Crookes
  named_after: "Greek: thallos (green twig), for a bright green line in its spectrum"
  pos_x: 12
  pos_y: 5
- name: Lead
  symbol: Pb
  atomic_number: 82
  atomic_weigth: "207.2"
  density: 11.34-11.35
  melting_point: 600.61-600.65
  boiling_point: 2013-2022
  atomic_radius: "175"
  covalent_radius: "147"
  ionic_radius: (+4e) 84 (+2e) 120
  atomic_volume: "18.3"
  specific_heat: "0.159"
  fusion_heat: "4.77"
  evaporation_heat: "177.8"
  thermal_conductivity: "35.3"
  debye_temperature: "88.00"
  pauling_negativity_number: "1.8"
  first_ionizing_energy: "715.2"
  oxidation_states: "4, 2, 0"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#178;"
  lattice_structure: FCC
  lattice_constant: "4.950"
  lattice_ca_ratio: n/a
  appearance: "Very soft, highly malleable and ductile, blue-white shiny metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: lead; symbol from Latin: plumbum"
  pos_x: 13
  pos_y: 5
- name: Bismuth
  symbol: Bi
  atomic_number: 83
  atomic_weigth: "208.9804"
  density: 9.747-9.79
  melting_point: "544.5"
  boiling_point: 1564-1883
  atomic_radius: "170"
  covalent_radius: "146"
  ionic_radius: (+5e) 74 (+3e) 96
  atomic_volume: "21.3"
  specific_heat: "0.124"
  fusion_heat: 11.00-11.30
  evaporation_heat: "172.0"
  thermal_conductivity: "7.9"
  debye_temperature: "120.00"
  pauling_negativity_number: "2.02"
  first_ionizing_energy: "702.9"
  oxidation_states: "5, 3"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#179;"
  lattice_structure: RHL
  lattice_constant: "4.750"
  lattice_ca_ratio: n/a
  appearance: "Hard, brittle, steel-gray metal with a pinkish tinge"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "German: bisemutum (white mass); now spelled wismut"
  pos_x: 14
  pos_y: 5
- name: Polonium
  symbol: Po
  atomic_number: 84
  atomic_weigth: "208.9824"
  density: 9.196-9.32
  melting_point: "527"
  boiling_point: "1235"
  atomic_radius: "176"
  covalent_radius: "146"
  ionic_radius: (+6e) 67
  atomic_volume: "22.7"
  specific_heat: "0.125"
  fusion_heat: "10"
  evaporation_heat: "102.9"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: 2.0-2.3
  first_ionizing_energy: "813.1"
  oxidation_states: "-2, 2, 4, 6"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#8308;"
  lattice_structure: CUB
  lattice_constant: "3.350"
  lattice_ca_ratio: n/a
  appearance: Silvery-gray metal
  discovery_date: 1898 (France/Poland)
  discovered_by: Pierre and Marie Curie-Sklodowska
  named_after: "Named for Poland, native country of Marie Curie"
  pos_x: 15
  pos_y: 5
- name: Astatine
  symbol: At
  atomic_number: 85
  atomic_weigth: "209.9871"
  density: (6.4)
  melting_point: 503-575
  boiling_point: 575-610
  atomic_radius: "145"
  covalent_radius: (145)
  ionic_radius: (+7e) 62
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: (195)
  pauling_negativity_number: 2.2-2.5
  first_ionizing_energy: "916.3"
  oxidation_states: "7, 5, 3, 1, -1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#8309;"
  lattice_structure: FCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Unstable, radioactive halogen"
  discovery_date: 1940 (United States)
  discovered_by: "D.R.Corson, K.R.MacKenzie, E. Segre"
  named_after: "Greek: astatos (unstable)"
  pos_x: 16
  pos_y: 5
- name: Radon
  symbol: Rn
  atomic_number: 86
  atomic_weigth: "222.0176"
  density: 4.4 (@ -62 °C)
  melting_point: "202"
  boiling_point: "211.4"
  atomic_radius: "214"
  covalent_radius: 140-150
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: "0.094"
  fusion_heat: "2.7"
  evaporation_heat: "18.1"
  thermal_conductivity: "0.0036"
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: "1036.5"
  oxidation_states: "2, 4, 6, 8"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#8310;"
  lattice_structure: FCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: Heavy radioactive gas
  discovery_date: 1898 (Germany/England)
  discovered_by: "Fredrich Ernst Dorn, Ernest Rutherford"
  named_after: "Variation of the name of another element, radium"
  pos_x: 17
  pos_y: 5
- name: Francium
  symbol: Fr
  atomic_number: 87
  atomic_weigth: "223.0197"
  density: "1.87"
  melting_point: 291-300
  boiling_point: 913-950
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: (+1e) 180
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: ~2-15
  evaporation_heat: ~65
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "0.7"
  first_ionizing_energy: "380"
  oxidation_states: "1, 2"
  eletronic_configuration: "[Rn] 7s&#185;"
  lattice_structure: BCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1939 (France)
  discovered_by: Marguerite Perey
  named_after: "Named for France, the nation of its discovery"
  pos_x: 0
  pos_y: 6
- name: Radium
  symbol: Ra
  atomic_number: 88
  atomic_weigth: "226.0254"
  density: 5.5 (@ +20 °C)
  melting_point: "973"
  boiling_point: 1413-2010
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: (+2e) 143
  atomic_volume: "45.0"
  specific_heat: "0.120"
  fusion_heat: 8.5-(9.6)
  evaporation_heat: "113"
  thermal_conductivity: (18.6)
  debye_temperature: n/a
  pauling_negativity_number: "0.9"
  first_ionizing_energy: 509.3; 979.0
  oxidation_states: "2"
  eletronic_configuration: "[Rn] 7s&#178;"
  lattice_structure: BCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery white, radioactive element"
  discovery_date: 1898 (France/Poland)
  discovered_by: Pierre and Marie Curie-Sklodowska
  named_after: "Latin: radius (beam, ray)"
  pos_x: 1
  pos_y: 6
- name: Actinium
  symbol: Ac
  atomic_number: 89
  atomic_weigth: "227.0278"
  density: "10.07"
  melting_point: "1320"
  boiling_point: "3470"
  atomic_radius: "188"
  covalent_radius: n/a
  ionic_radius: (+3e) 118
  atomic_volume: "22.54"
  specific_heat: n/a
  fusion_heat: (10.5)
  evaporation_heat: (292.9)
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.1"
  first_ionizing_energy: "665.5"
  oxidation_states: "3"
  eletronic_configuration: "[Rn] 6d&#185; 7s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.310"
  lattice_ca_ratio: n/a
  appearance: "Heavy, Silvery-white metal that is very radioactive"
  discovery_date: 1899 (France)
  discovered_by: Andre-Louis Debierne
  named_after: "Greek: akis, aktis, aktinos (beam, ray)"
  pos_x: 3
  pos_y: 8
- name: Thorium
  symbol: Th
  atomic_number: 90
  atomic_weigth: "232.0381"
  density: "11.78"
  melting_point: "2028"
  boiling_point: "5060"
  atomic_radius: "180"
  covalent_radius: "165"
  ionic_radius: (+4e) 102
  atomic_volume: "19.8"
  specific_heat: "0.113"
  fusion_heat: "16.11"
  evaporation_heat: "513.7"
  thermal_conductivity: (54.0)
  debye_temperature: "100.00"
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "670.4"
  oxidation_states: "4"
  eletronic_configuration: "[Rn] 6d&#178; 7s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.080"
  lattice_ca_ratio: n/a
  appearance: "Gray, soft, malleable, ductile, radioactive metal"
  discovery_date: 1828 (Sweden)
  discovered_by: Jons Jacob Berzelius
  named_after: "Named for Thor, Norse god of thunder"
  pos_x: 4
  pos_y: 8
- name: Protactinium
  symbol: Pa
  atomic_number: 91
  atomic_weigth: "231.03588"
  density: "15.37"
  melting_point: "2113"
  boiling_point: "4300"
  atomic_radius: "161"
  covalent_radius: n/a
  ionic_radius: (+5e) 89 (+3e) 113
  atomic_volume: "15.0"
  specific_heat: "0.121"
  fusion_heat: "16.7"
  evaporation_heat: "481.2"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.5"
  first_ionizing_energy: "0.0"
  oxidation_states: "5, 4"
  eletronic_configuration: "[Rn] 5f&#178; 6d&#185; 7s&#178;"
  lattice_structure: TET
  lattice_constant: "3.920"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, radioactive metal"
  discovery_date: 1918 (England/France)
  discovered_by: "Fredrich Soddy, John Cranston, Otto Hahn, Lise Meitner"
  named_after: "Greek: proto and actinium (parent of actinium); it forms actinium when it radioactively decays"
  pos_x: 5
  pos_y: 8
- name: Uranium
  symbol: U
  atomic_number: 92
  atomic_weigth: "238.0289"
  density: "19.05"
  melting_point: "1405.5"
  boiling_point: "4018"
  atomic_radius: "138"
  covalent_radius: "142"
  ionic_radius: (+6e) 80 (+4e) 97
  atomic_volume: "12.5"
  specific_heat: "0.115"
  fusion_heat: "12.6"
  evaporation_heat: "417"
  thermal_conductivity: "27.5"
  debye_temperature: n/a
  pauling_negativity_number: "1.38"
  first_ionizing_energy: "686.4"
  oxidation_states: "6, 5, 4, 3"
  eletronic_configuration: "[Rn] 5f&#179; 6d&#185; 7s&#178;"
  lattice_structure: ORC
  lattice_constant: "2.850"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, dense, ductile and malleable, radioactive metal"
  discovery_date: 1789 (Germany)
  discovered_by: Martin Klaproth
  named_after: Named for the planet Uranus
  pos_x: 6
  pos_y: 8
- name: Neptunium
  symbol: Np
  atomic_number: 93
  atomic_weigth: "237.048"
  density: "20.25"
  melting_point: "913"
  boiling_point: "4175"
  atomic_radius: "130"
  covalent_radius: n/a
  ionic_radius: (+4e) 95 (+3e) 110
  atomic_volume: "21.1"
  specific_heat: n/a
  fusion_heat: (9.6)
  evaporation_heat: "336"
  thermal_conductivity: (6.3)
  debye_temperature: n/a
  pauling_negativity_number: "1.36"
  first_ionizing_energy: "0.0"
  oxidation_states: "7, 6, 5, 4, 3"
  eletronic_configuration: "[Rn] 5f&#8308; 6d&#185; 7s&#178;"
  lattice_structure: ORC
  lattice_constant: "4.720"
  lattice_ca_ratio: n/a
  appearance: Silvery metal
  discovery_date: 1940 (United States)
  discovered_by: "E.M. McMillan, P.H. Abelson"
  named_after: Named for the planet Neptune
  pos_x: 7
  pos_y: 8
- name: Plutonium
  symbol: Pu
  atomic_number: 94
  atomic_weigth: "244.0642"
  density: "19.84"
  melting_point: 641-912.85
  boiling_point: 3505-3508.15
  atomic_radius: 151-162
  covalent_radius: n/a
  ionic_radius: 71-100 or (+4e) 93 (+3e) 108
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: "2.8"
  evaporation_heat: "343.5"
  thermal_conductivity: (6.7)
  debye_temperature: n/a
  pauling_negativity_number: "1.28"
  first_ionizing_energy: 491.9-584.7
  oxidation_states: "2, 3, 4, 5, 6, 7"
  eletronic_configuration: "[Rn] 5f&#8310; 7s&#178;"
  lattice_structure: MCL
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, radioactive metal"
  discovery_date: 1940 (United States)
  discovered_by: "G.T.Seaborg, J.W.Kennedy, E.M.McMillan, A.C.Wohl"
  named_after: Named for the planet Pluto
  pos_x: 8
  pos_y: 8
- name: Americium
  symbol: Am
  atomic_number: 95
  atomic_weigth: "243.0614"
  density: "13.67"
  melting_point: 1267-1448
  boiling_point: 2880-2880.15
  atomic_radius: "173"
  covalent_radius: n/a
  ionic_radius: (+4e) 92 (+3e) 107
  atomic_volume: "20.8"
  specific_heat: n/a
  fusion_heat: (10.0)
  evaporation_heat: "238.5"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "0.0"
  oxidation_states: "6, 5, 4, 3"
  eletronic_configuration: "[Rn] 5f&#8311; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, radioactive metal"
  discovery_date: 1944 (United States)
  discovered_by: "G.T.Seaborg, R.A.James, L.O.Morgan, A.Ghiorso"
  named_after: "Named for the American continent, by analogy with europium"
  pos_x: 9
  pos_y: 8
- name: Curium
  symbol: Cm
  atomic_number: 96
  atomic_weigth: "247.0703"
  density: "13.51"
  melting_point: 1340-1613
  boiling_point: "3383"
  atomic_radius: "299"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: "18.28"
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "581"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Rn] 5f&#8311; 6d&#185; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery, malleable, synthetic radioactive metal"
  discovery_date: 1944 (United States)
  discovered_by: "G.T.Seaborg, R.A.James, A.Ghiorso"
  named_after: Named in honor of Pierre and Marie Curie
  pos_x: 10
  pos_y: 8
- name: Berkelium
  symbol: Bk
  atomic_number: 97
  atomic_weigth: "247.0703"
  density: "13.25"
  melting_point: "1259"
  boiling_point: "2900"
  atomic_radius: "297"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: (600)
  oxidation_states: "4, 3"
  eletronic_configuration: "[Rn] 5f&#8313; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: Radioactive synthetic metal
  discovery_date: 1949 (United States)
  discovered_by: "G.T.Seaborg, S.G.Tompson, A.Ghiorso"
  named_after: "Named after Berkeley, California the city of its discovery"
  pos_x: 11
  pos_y: 8
- name: Californium
  symbol: Cf
  atomic_number: 98
  atomic_weigth: "251.0796"
  density: "15.1"
  melting_point: 900-1173.15
  boiling_point: (1743)
  atomic_radius: "295"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: (610)
  oxidation_states: "2, 3, 4"
  eletronic_configuration: "[Rn] 5f&#185;&#8304; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: Powerful neutron emitter
  discovery_date: 1950 (United States)
  discovered_by: "G.T.Seaborg, S.G.Tompson, A.Ghiorso, K.Street Jr."
  named_after: Named after the US-state and University of California
  pos_x: 12
  pos_y: 8
- name: Einsteinium
  symbol: Es
  atomic_number: 99
  atomic_weigth: "252.083"
  density: "13.5"
  melting_point: "860"
  boiling_point: "1130"
  atomic_radius: "292"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "619"
  oxidation_states: "2, 3, 4"
  eletronic_configuration: "[Rn] 5f&#185;&#185; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1952 (United States)
  discovered_by: "Argonne, Los Alamos, University of California"
  named_after: Named in honor of the scientist Albert Einstein
  pos_x: 13
  pos_y: 8
- name: Fermium
  symbol: Fm
  atomic_number: 100
  atomic_weigth: "257.0951"
  density: n/a
  melting_point: "1800"
  boiling_point: n/a
  atomic_radius: "290"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: 627-630
  oxidation_states: "3"
  eletronic_configuration: "[Rn] 5f&#185;&#178; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1952 (United States)
  discovered_by: "Argonne, Los Alamos, University of California"
  named_after: Named in honor of the scientist Enrico Fermi
  pos_x: 14
  pos_y: 8
- name: Mendelevium
  symbol: Md
  atomic_number: 101
  atomic_weigth: "258.1"
  density: n/a
  melting_point: "1100"
  boiling_point: n/a
  atomic_radius: "287"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "635"
  oxidation_states: "1, 2, 3"
  eletronic_configuration: "[Rn] 5f&#185;&#179; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1955 (United States)
  discovered_by: "G.T.Seaborg, S.G.Tompson, A.Ghiorso, K.Street Jr."
  named_after: "Named in honor of the scientist Dmitry Ivanovich Mendeleev, who devised the periodic table"
  pos_x: 15
  pos_y: 8
- name: Nobelium
  symbol: "No"
  atomic_number: 102
  atomic_weigth: "259.1009"
  density: n/a
  melting_point: "1100"
  boiling_point: n/a
  atomic_radius: "285"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "640"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1963 (USSR)
  discovered_by: Dubna
  named_after: "Named in honor of Alfred Nobel, who invented dynamite and founded Nobel prize"
  pos_x: 16
  pos_y: 8
- name: Lawrencium
  symbol: Lr
  atomic_number: 103
  atomic_weigth: "262.11"
  density: n/a
  melting_point: "1900"
  boiling_point: n/a
  atomic_radius: "282"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "470"
  oxidation_states: "3"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1961 (USSR/United States)
  discovered_by: Soviet Nuclear Research/University of California at Berkeley
  named_after: "Named in honor of Ernest Orlando Lawrence, inventor of the cyclotron"
  pos_x: 17
  pos_y: 8
- name: Rutherfordium
  symbol: Rf
  atomic_number: 104
  atomic_weigth: "261"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "4"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#178; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1964 (USSR)
  discovered_by: Dubna
  named_after: Named in honor of Ernest Rutherford
  pos_x: 3
  pos_y: 6
- name: Dubnium
  symbol: Db
  atomic_number: 105
  atomic_weigth: "268"
  density: "26.1"
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: "139"
  covalent_radius: "66"
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "2, 3, 4, 5"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#179; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1970 (USSR/United States)
  discovered_by: Soviet Nuclear Research/University of California at Berkeley
  named_after: Named after the science-town Dubna in Russia
  pos_x: 4
  pos_y: 6
- name: Seaborgium
  symbol: Sg
  atomic_number: 106
  atomic_weigth: "[271]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8308; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1974 (USSR/United States)
  discovered_by: Soviet Nuclear Research/University of California at Berkeley
  named_after: "Named in honor of Glenn Theodore Seaborg, American physical chemist known for research on transuranium elements"
  pos_x: 5
  pos_y: 6
- name: Bohrium
  symbol: Bh
  atomic_number: 107
  atomic_weigth: "[267]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: (128)
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: (660)
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8309; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1976 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of Niels Bohr
  pos_x: 6
  pos_y: 6
- name: Hassium
  symbol: Hs
  atomic_number: 108
  atomic_weigth: "[269]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8310; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1984 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: "Named in honor of Hessen county, Germany (latin: Hassia)"
  pos_x: 7
  pos_y: 6
- name: Meitnerium
  symbol: Mt
  atomic_number: 109
  atomic_weigth: "[278]"
  density: "37.4"
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "9, 8, 6, 4, 3, 1"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8311; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1982 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of Lise Meitner
  pos_x: 8
  pos_y: 6
- name: Darmstadtium
  symbol: Ds
  atomic_number: 110
  atomic_weigth: "[281]"
  density: (34.8)
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8313; 7s&#185;/[Rn] 5f&#185;&#8308; 6d&#8312; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1994 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: "Named after Darmstadt, Germany, the city of its discovery"
  pos_x: 9
  pos_y: 6
- name: Roentgenium
  symbol: Rg
  atomic_number: 111
  atomic_weigth: "[281]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#185;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1994 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of physicist Wilhelm Conrad Roentgen
  pos_x: 10
  pos_y: 6
- name: Copernicium
  symbol: Cn
  atomic_number: 112
  atomic_weigth: "285"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1996 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of Nicolaus Copernicus
  pos_x: 11
  pos_y: 6
- name: Ununtrium
  symbol: Uut
  atomic_number: 113
  atomic_weigth: (286)
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#185;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2004 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) trium (three)
  pos_x: 12
  pos_y: 6
- name: Flerovium
  symbol: Fl
  atomic_number: 114
  atomic_weigth: "[289]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "4, 2"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1998 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Named in honor of Soviet physicist Flerov
  pos_x: 13
  pos_y: 6
- name: Ununpentium
  symbol: Uup
  atomic_number: 115
  atomic_weigth: "288"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#179;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2003 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) pentium (five)
  pos_x: 14
  pos_y: 6
- name: Livermorium
  symbol: Lv
  atomic_number: 116
  atomic_weigth: "[193]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "-2, 2, 4, 6"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#8308;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2000 (Russia/United States)
  discovered_by: "Dubna, Dimitrovgrad, Lesnoy/Livermore"
  named_after: Named in honor of Livermore National Laboratory (USA)
  pos_x: 15
  pos_y: 6
- name: Ununseptium
  symbol: Uus
  atomic_number: 117
  atomic_weigth: "[294]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#8309;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2009 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) septium (seven)
  pos_x: 16
  pos_y: 6
- name: Ununoctium
  symbol: Uuo
  atomic_number: 118
  atomic_weigth: "[294]"
  density: "13.65"
  melting_point: n/a
  boiling_point: "350&#177;30"
  atomic_radius: "152"
  covalent_radius: "230"
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: "23.5"
  evaporation_heat: "19.4"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: "975&#177;155"
  oxidation_states: "-1, 0, 1, 2, 4, 6, 8"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#8310;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2006 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) octium (eight)
  pos_x: 17
  pos_y: 6
- name: Hydrogen
  symbol: H
  atomic_number: 1
  atomic_weigth: "1.00794"
  density: 0.0708 (@ -253 °C)
  melting_point: "14.01"
  boiling_point: "20.28"
  atomic_radius: 53-79
  covalent_radius: "32"
  ionic_radius: 54 (-1e)
  atomic_volume: "14.1"
  specific_heat: 14.267 (H-H)
  fusion_heat: 0.117 (H-H)
  evaporation_heat: 0.904 (H-H)
  thermal_conductivity: "0.1815"
  debye_temperature: "110.00"
  pauling_negativity_number: "2.20"
  first_ionizing_energy: "1311.3"
  oxidation_states: "1, 0, -1"
  eletronic_configuration: "1s&#185;"
  lattice_structure: HEX
  lattice_constant: "3.750"
  lattice_ca_ratio: "1.731"
  appearance: "Colorless, odorless, tasteless gas"
  discovery_date: 1766 (England)
  discovered_by: Henry Cavendish
  named_after: "Greek: hydro (water) and genes (generate)"
  pos_x: 0
  pos_y: 0
- name: Helium
  symbol: He
  atomic_number: 2
  atomic_weigth: "4.002602"
  density: 0.147 (@ -270 °C)
  melting_point: "0.95"
  boiling_point: "4.216"
  atomic_radius: 28-31
  covalent_radius: 28-140
  ionic_radius: "93"
  atomic_volume: "31.8"
  specific_heat: "5.188"
  fusion_heat: n/a
  evaporation_heat: "0.08"
  thermal_conductivity: "0.152"
  debye_temperature: n/a
  pauling_negativity_number: "4.5"
  first_ionizing_energy: "2361.3"
  oxidation_states: "0"
  eletronic_configuration: "1s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.570"
  lattice_ca_ratio: "1.633"
  appearance: "Inert, colorless, odorless, tasteless gas"
  discovery_date: 1895 (Scotland/Sweden)
  discovered_by: "Sir William Ramsey, Nils Langet, P.T.Cleve"
  named_after: "Greek: helios (sun)"
  pos_x: 17
  pos_y: 0
- name: Lithium
  symbol: Li
  atomic_number: 3
  atomic_weigth: "6.941"
  density: "0.534"
  melting_point: 453.69-553.69
  boiling_point: 1118.5-1613
  atomic_radius: 145-155
  covalent_radius: 134-163
  ionic_radius: 68 (+1e) or 76 (+1e)
  atomic_volume: "13.1"
  specific_heat: "3.489"
  fusion_heat: "2.89"
  evaporation_heat: "148"
  thermal_conductivity: "84.8"
  debye_temperature: "400.00"
  pauling_negativity_number: "0.98"
  first_ionizing_energy: "519.9"
  oxidation_states: "1"
  eletronic_configuration: "[He] 2s&#185;"
  lattice_structure: BCC
  lattice_constant: "3.490"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white metal"
  discovery_date: 1817 (Sweden)
  discovered_by: Johann Arfwedson
  named_after: "Greek: lithos (stone)"
  pos_x: 0
  pos_y: 1
- name: Beryllium
  symbol: Be
  atomic_number: 4
  atomic_weigth: "9.01218"
  density: "1.848"
  melting_point: "1551"
  boiling_point: "3243"
  atomic_radius: "112"
  covalent_radius: "90"
  ionic_radius: 35 (+2e)
  atomic_volume: "5.0"
  specific_heat: "1.824"
  fusion_heat: "12.21"
  evaporation_heat: "309"
  thermal_conductivity: "201"
  debye_temperature: "1000.00"
  pauling_negativity_number: "1.57"
  first_ionizing_energy: "898.8"
  oxidation_states: "2, 1"
  eletronic_configuration: "[He] 2s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.290"
  lattice_ca_ratio: "1.567"
  appearance: "Hard, brittle, steel-gray metal"
  discovery_date: 1798 (France)
  discovered_by: Louis-Nicolas Vauquelin
  named_after: "Greek: beryllos, 'beryl' (a mineral)"
  pos_x: 1
  pos_y: 1
- name: Boron
  symbol: B
  atomic_number: 5
  atomic_weigth: "10.811"
  density: "2.34"
  melting_point: "2573"
  boiling_point: "3931"
  atomic_radius: "98"
  covalent_radius: "82"
  ionic_radius: 23 (+3e)
  atomic_volume: "4.6"
  specific_heat: "1.025"
  fusion_heat: "23.60"
  evaporation_heat: "504.5"
  thermal_conductivity: "27.4"
  debye_temperature: "1250.00"
  pauling_negativity_number: "2.04"
  first_ionizing_energy: "800.2"
  oxidation_states: "3"
  eletronic_configuration: "[He] 2s&#178; 2p&#185;"
  lattice_structure: RHL
  lattice_constant: "8.730"
  lattice_ca_ratio: "0.576"
  appearance: "Hard, brittle, lustrous black semimetal"
  discovery_date: 1808 (England/France)
  discovered_by: "Sir H. Davy, J.L. Gay-Lussac, L.J. Thenard"
  named_after: The Arabic and Persian words for borax
  pos_x: 12
  pos_y: 1
- name: Carbon
  symbol: C
  atomic_number: 6
  atomic_weigth: "12.011"
  density: 2.25 (graphite)
  melting_point: "3820"
  boiling_point: "5100"
  atomic_radius: "91"
  covalent_radius: "77"
  ionic_radius: 16 (+4e) 260 (-4e)
  atomic_volume: "5.3"
  specific_heat: "0.711"
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: "1.59"
  debye_temperature: 1860.00 (diamond)
  pauling_negativity_number: "2.55"
  first_ionizing_energy: "1085.7"
  oxidation_states: "4, 3, 2, 1, 0, -1, -2, -3, -4"
  eletronic_configuration: "[He] 2s&#178; 2p&#178;"
  lattice_structure: HEX (graphite) DIA (diamond)
  lattice_constant: "3.570"
  lattice_ca_ratio: n/a
  appearance: "Dense, Black (graphite)"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Latin: carbo (charcoal)"
  pos_x: 13
  pos_y: 1
- name: Nitrogen
  symbol: N
  atomic_number: 7
  atomic_weigth: "14.00674"
  density: 0.808 (@ -195.8 °C)
  melting_point: "63.29"
  boiling_point: "77.4"
  atomic_radius: "92"
  covalent_radius: "75"
  ionic_radius: 13 (+5e) 171 (-3e)
  atomic_volume: "17.3"
  specific_heat: 1.042 (N-N)
  fusion_heat: 0.720 (N-N)
  evaporation_heat: 5.57 (N-N)
  thermal_conductivity: "0.026"
  debye_temperature: n/a
  pauling_negativity_number: "3.04"
  first_ionizing_energy: "1401.5"
  oxidation_states: "5, 4, 3, 2, 1, 0, -1, -2, -3"
  eletronic_configuration: "[He] 2s&#178; 2p&#179;"
  lattice_structure: HEX or CUB
  lattice_constant: 4.039 (HEX)
  lattice_ca_ratio: 1.651 (HEX)
  appearance: "Colorless, odorless, tasteless, and generally inert gas"
  discovery_date: 1772 (Scotland)
  discovered_by: Daniel Rutherford
  named_after: "Greek: nitron and genes (soda forming)"
  pos_x: 14
  pos_y: 1
- name: Oxygen
  symbol: O
  atomic_number: 8
  atomic_weigth: "15.9994"
  density: 1.149 (@ -183 °C)
  melting_point: "54.8"
  boiling_point: "90.19"
  atomic_radius: 60 (48)
  covalent_radius: "73"
  ionic_radius: 132 (-2e)
  atomic_volume: "14.0"
  specific_heat: 0.916 (O-O)
  fusion_heat: "0.444"
  evaporation_heat: "3.4099"
  thermal_conductivity: "0.027"
  debye_temperature: "155"
  pauling_negativity_number: "3.44"
  first_ionizing_energy: "1313.1"
  oxidation_states: "-2, -1, -1/2, -1/3, 0, 1/2, 1, 2"
  eletronic_configuration: "[He] 2s&#178; 2p&#8308;"
  lattice_structure: MCL or CUB
  lattice_constant: 6.830 (CUB)
  lattice_ca_ratio: n/a
  appearance: "Colorless, odorless, tasteless gas; pale blue liquid"
  discovery_date: 1774 (England/Sweden)
  discovered_by: "Joseph Priestly, Carl Wilhelm Scheele"
  named_after: "Greek: oxys and genes (acid former)"
  pos_x: 15
  pos_y: 1
- name: Fluorine
  symbol: F
  atomic_number: 9
  atomic_weigth: "18.998403"
  density: 1.108 (@ -189 °C)
  melting_point: "53.53"
  boiling_point: "85.03"
  atomic_radius: "71"
  covalent_radius: "72"
  ionic_radius: (-1e) 133
  atomic_volume: "17.1"
  specific_heat: 0.824 (F-F)
  fusion_heat: 0.51 (F-F)
  evaporation_heat: 6.54 (F-F)
  thermal_conductivity: "0.028"
  debye_temperature: n/a
  pauling_negativity_number: "3.98"
  first_ionizing_energy: "1680.0"
  oxidation_states: "-1, 0"
  eletronic_configuration: "[He] 2s&#178; 2p&#8309;"
  lattice_structure: MCL
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Greenish-yellow, pungent, corrosive gas"
  discovery_date: 1886 (France)
  discovered_by: Henri Moissan
  named_after: "Latin: fluere (flow)"
  pos_x: 16
  pos_y: 1
- name: Neon
  symbol: Ne
  atomic_number: 10
  atomic_weigth: "20.1797"
  density: 1.204 (@ -246 °C)
  melting_point: "24.55"
  boiling_point: "27.1"
  atomic_radius: (38)
  covalent_radius: 58-71
  ionic_radius: "112"
  atomic_volume: "16.8"
  specific_heat: "1.029"
  fusion_heat: n/a
  evaporation_heat: "1.74"
  thermal_conductivity: (0.0493)
  debye_temperature: "63.00"
  pauling_negativity_number: "4.4"
  first_ionizing_energy: "2079.4"
  oxidation_states: n/a
  eletronic_configuration: "[He] 2s&#178; 2p&#8310;"
  lattice_structure: FCC
  lattice_constant: "4.430"
  lattice_ca_ratio: n/a
  appearance: "Colorless, odorless, tasteless gas"
  discovery_date: 1898 (England)
  discovered_by: "Sir William Ramsey, M.W. Travers"
  named_after: "Greek: neos (new)"
  pos_x: 17
  pos_y: 1
- name: Sodium
  symbol: Na
  atomic_number: 11
  atomic_weigth: "22.989769"
  density: "0.971"
  melting_point: "370.96"
  boiling_point: "1156.1"
  atomic_radius: "190"
  covalent_radius: "154"
  ionic_radius: 97 (+1e)
  atomic_volume: "23.7"
  specific_heat: "1.222"
  fusion_heat: "2.64"
  evaporation_heat: "97.9"
  thermal_conductivity: "142.0"
  debye_temperature: "150.00"
  pauling_negativity_number: "0.93"
  first_ionizing_energy: "495.6"
  oxidation_states: "1"
  eletronic_configuration: "[Ne] 3s&#185;"
  lattice_structure: BCC
  lattice_constant: "4.230"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white metal"
  discovery_date: 1807 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Medieval Latin: sodanum (headache remedy); symbol from Latin natrium (sodium carbonate)"
  pos_x: 0
  pos_y: 2
- name: Magnesium
  symbol: Mg
  atomic_number: 12
  atomic_weigth: "24.305"
  density: "1.738"
  melting_point: "923"
  boiling_point: "1363"
  atomic_radius: "160"
  covalent_radius: "136"
  ionic_radius: 66 (+2e)
  atomic_volume: "14.0"
  specific_heat: "1.025"
  fusion_heat: "9.20"
  evaporation_heat: "131.8"
  thermal_conductivity: "156"
  debye_temperature: "318.00"
  pauling_negativity_number: "1.31"
  first_ionizing_energy: "737.3"
  oxidation_states: "2"
  eletronic_configuration: "[Ne] 3s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.210"
  lattice_ca_ratio: "1.624"
  appearance: "Lightweight, malleable, silvery-white metal"
  discovery_date: 1808 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Magnesia, ancient city in district of Thessaly, Greece"
  pos_x: 1
  pos_y: 2
- name: Aluminum
  symbol: Al
  atomic_number: 13
  atomic_weigth: "26.981539"
  density: "2.6989"
  melting_point: "933.5"
  boiling_point: 2740-2792
  atomic_radius: "143"
  covalent_radius: "121&#177;4"
  ionic_radius: 51 (+3e)
  atomic_volume: "10.0"
  specific_heat: "0.900"
  fusion_heat: "10.75"
  evaporation_heat: "284.1"
  thermal_conductivity: "237"
  debye_temperature: "394.00"
  pauling_negativity_number: "1.61"
  first_ionizing_energy: 577.5; 1816.7
  oxidation_states: "3"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#185;"
  lattice_structure: FCC
  lattice_constant: "4.050"
  lattice_ca_ratio: n/a
  appearance: "Soft, lightweight, silvery-white metal"
  discovery_date: 1825 (Denmark)
  discovered_by: Hans Christian Oersted
  named_after: "Latin: alumen, aluminis (alum)"
  pos_x: 12
  pos_y: 2
- name: Silicon
  symbol: Si
  atomic_number: 14
  atomic_weigth: "28.0855"
  density: "2.33"
  melting_point: "1688"
  boiling_point: "2623"
  atomic_radius: "132"
  covalent_radius: "111"
  ionic_radius: 42 (+4e) 271  (-4e)
  atomic_volume: "12.1"
  specific_heat: "0.703"
  fusion_heat: "50.6"
  evaporation_heat: "383"
  thermal_conductivity: "149"
  debye_temperature: "625.00"
  pauling_negativity_number: "1.90"
  first_ionizing_energy: "786.0"
  oxidation_states: "4, 2, 0, -4"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#178;, [Ne] 3s 3p&#179;"
  lattice_structure: DIA
  lattice_constant: "5.430"
  lattice_ca_ratio: n/a
  appearance: Amorphous form is brown powder; crystalline form has a gray
  discovery_date: 1825 (Sweden)
  discovered_by: Jons Jacob Berzelius
  named_after: "Latin: silex, silicus (flint)"
  pos_x: 13
  pos_y: 2
- name: Phosphorus
  symbol: P
  atomic_number: 15
  atomic_weigth: "30.973762"
  density: 1.82 (white phosphorus)
  melting_point: "317.3"
  boiling_point: "553"
  atomic_radius: "128"
  covalent_radius: "106"
  ionic_radius: 35 (+5e) 212 (-3e)
  atomic_volume: "17.0"
  specific_heat: "0.757"
  fusion_heat: "2.51"
  evaporation_heat: "49.8"
  thermal_conductivity: (0.236)
  debye_temperature: n/a
  pauling_negativity_number: "2.19"
  first_ionizing_energy: "1011.2"
  oxidation_states: "5, 3, 1, 0, -1, -3"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#179;"
  lattice_structure: BCC
  lattice_constant: "7.170"
  lattice_ca_ratio: n/a
  appearance: "The most common white form is a waxy, phosphorescent solid"
  discovery_date: 1669 (Germany)
  discovered_by: Hennig Brand
  named_after: "Greek: phosphoros (bringer of light)"
  pos_x: 14
  pos_y: 2
- name: Sulfur
  symbol: S
  atomic_number: 16
  atomic_weigth: "32.066"
  density: "2.070"
  melting_point: "386"
  boiling_point: "717.824"
  atomic_radius: "127"
  covalent_radius: "102"
  ionic_radius: 30 (+6e) 184 (-2e)
  atomic_volume: "15.5"
  specific_heat: "0.732"
  fusion_heat: "1.23"
  evaporation_heat: "10.5"
  thermal_conductivity: "0.27"
  debye_temperature: n/a
  pauling_negativity_number: "2.58"
  first_ionizing_energy: "999.0"
  oxidation_states: "6, 4, 2, 1, 0, -1, -2"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#8308;"
  lattice_structure: ORC
  lattice_constant: "10.470"
  lattice_ca_ratio: n/a
  appearance: "Tasteless, odorless, light-yellow, brittle solid"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Latin: sulphur (brimstone)"
  pos_x: 15
  pos_y: 2
- name: Chlorine
  symbol: Cl
  atomic_number: 17
  atomic_weigth: "35.4527"
  density: 1.56 (@ -33.6 °C)
  melting_point: "172.2"
  boiling_point: "238.6"
  atomic_radius: "100"
  covalent_radius: "99"
  ionic_radius: (+7e)27 (-1e)181
  atomic_volume: "18.7"
  specific_heat: 0.477 (Cl-Cl)
  fusion_heat: 6.41 (Cl-Cl)
  evaporation_heat: 20.41 (Cl-Cl)
  thermal_conductivity: "0.009"
  debye_temperature: n/a
  pauling_negativity_number: "3.16"
  first_ionizing_energy: "1254.9"
  oxidation_states: "7, 6, 5, 4, 3, 1, 0, -1"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#8309;"
  lattice_structure: ORC
  lattice_constant: "6.240"
  lattice_ca_ratio: n/a
  appearance: "Greenish-yellow, disagreeable gas"
  discovery_date: 1774 (Sweden)
  discovered_by: Carl Wilhelm Scheele
  named_after: "Greek: chloros (greenish yellow)"
  pos_x: 16
  pos_y: 2
- name: Argon
  symbol: Ar
  atomic_number: 18
  atomic_weigth: "39.948"
  density: "1.40 (@ -189,35 °C)"
  melting_point: "83.8"
  boiling_point: "87.3"
  atomic_radius: (71)
  covalent_radius: 98-106
  ionic_radius: "154"
  atomic_volume: "24.2"
  specific_heat: "0.138"
  fusion_heat: "7.05"
  evaporation_heat: 6.45-6.52
  thermal_conductivity: "0.0177"
  debye_temperature: "85.00"
  pauling_negativity_number: 0.0-4.3
  first_ionizing_energy: "1519.6"
  oxidation_states: "0"
  eletronic_configuration: "[Ne] 3s&#178; 3p&#8310;"
  lattice_structure: FCC
  lattice_constant: "5.260"
  lattice_ca_ratio: n/a
  appearance: "Colorless, tasteless, odorless noble gas"
  discovery_date: 1894 (Scotland)
  discovered_by: "Sir William Ramsey, Baron Rayleigh"
  named_after: "Greek: argos (inactive)"
  pos_x: 17
  pos_y: 2
- name: Potassium
  symbol: K
  atomic_number: 19
  atomic_weigth: "39.0983"
  density: "0.856"
  melting_point: 336.53-336.8
  boiling_point: "1047"
  atomic_radius: "235"
  covalent_radius: "203"
  ionic_radius: (+1e)133
  atomic_volume: "45.3"
  specific_heat: "0.753"
  fusion_heat: "2.33"
  evaporation_heat: "76.9"
  thermal_conductivity: "79.0"
  debye_temperature: "100.00"
  pauling_negativity_number: "0.82"
  first_ionizing_energy: "418.5"
  oxidation_states: "1"
  eletronic_configuration: "[Ar] 4s&#185;"
  lattice_structure: BCC
  lattice_constant: "5.230"
  lattice_ca_ratio: n/a
  appearance: "Soft, waxy, silvery-white metal"
  discovery_date: 1807 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "English: pot ash; symbol from Latin: kalium (alkali)"
  pos_x: 0
  pos_y: 3
- name: Calcium
  symbol: Ca
  atomic_number: 20
  atomic_weigth: "40.078"
  density: "1.55"
  melting_point: "1112"
  boiling_point: "1757"
  atomic_radius: "197"
  covalent_radius: "174"
  ionic_radius: (+2e)99
  atomic_volume: "29.9"
  specific_heat: "0.653"
  fusion_heat: "9.20"
  evaporation_heat: "153.6"
  thermal_conductivity: (201)
  debye_temperature: "230.00"
  pauling_negativity_number: "1.00"
  first_ionizing_energy: "589.4"
  oxidation_states: "2"
  eletronic_configuration: "[Ar] 4s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.580"
  lattice_ca_ratio: n/a
  appearance: "Fairly hard, silvery-white metal"
  discovery_date: 1808 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Latin: calx, calcis (lime)"
  pos_x: 1
  pos_y: 3
- name: Scandium
  symbol: Sc
  atomic_number: 21
  atomic_weigth: "44.95591"
  density: "2.99"
  melting_point: "1814"
  boiling_point: 3104-3110
  atomic_radius: "162"
  covalent_radius: "144"
  ionic_radius: (+3e)72.3
  atomic_volume: "15.0"
  specific_heat: "0.556"
  fusion_heat: "15.8"
  evaporation_heat: "332.7"
  thermal_conductivity: "15.8"
  debye_temperature: n/a
  pauling_negativity_number: "1.36"
  first_ionizing_energy: "630.8"
  oxidation_states: "3"
  eletronic_configuration: "[Ar] 3d&#185; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.310"
  lattice_ca_ratio: "1.594"
  appearance: "Fairly soft, silvery-white metal"
  discovery_date: 1879 (Sweden)
  discovered_by: Lars Nilson
  named_after: "Latin: Scandia (Scandinavia)"
  pos_x: 2
  pos_y: 3
- name: Titanium
  symbol: Ti
  atomic_number: 22
  atomic_weigth: "47.867"
  density: "4.54"
  melting_point: "1933&#177;20"
  boiling_point: "3560"
  atomic_radius: "147"
  covalent_radius: "132"
  ionic_radius: (+4e)68 (+2e)94
  atomic_volume: "10.6"
  specific_heat: "0.523"
  fusion_heat: "18.8"
  evaporation_heat: "422.6"
  thermal_conductivity: "21.9"
  debye_temperature: "380.00"
  pauling_negativity_number: "1.54"
  first_ionizing_energy: "657.8"
  oxidation_states: "2, 3, 4"
  eletronic_configuration: "[Ar] 3d&#178; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.950"
  lattice_ca_ratio: n/a
  appearance: "Shiny, dark-gray metal"
  discovery_date: 1791 (England)
  discovered_by: William Gregor
  named_after: "Greek: titanos (Titans)"
  pos_x: 3
  pos_y: 3
- name: Vanadium
  symbol: V
  atomic_number: 23
  atomic_weigth: "50.9415"
  density: "6.11"
  melting_point: "2160"
  boiling_point: "3650"
  atomic_radius: "134"
  covalent_radius: "122"
  ionic_radius: (+5e)59 (+3e)7
  atomic_volume: "8.35"
  specific_heat: "0.485"
  fusion_heat: "17.5"
  evaporation_heat: "460"
  thermal_conductivity: "30.7"
  debye_temperature: "390.00"
  pauling_negativity_number: "1.63"
  first_ionizing_energy: "650.1"
  oxidation_states: "5, 4, 3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#179; 4s&#178;"
  lattice_structure: BCC
  lattice_constant: "3.020"
  lattice_ca_ratio: n/a
  appearance: "Soft, ductile, silvery-white metal"
  discovery_date: 1830 (Sweden)
  discovered_by: Nils Gabriel Sefstrom
  named_after: "The scandinavian goddess, Vanadis"
  pos_x: 4
  pos_y: 3
- name: Chromium
  symbol: Cr
  atomic_number: 24
  atomic_weigth: "51.9961"
  density: "7,18-7.19"
  melting_point: "2130"
  boiling_point: "2945"
  atomic_radius: "130"
  covalent_radius: "118"
  ionic_radius: (+6e)52 (+3e)63
  atomic_volume: "7.23"
  specific_heat: "0.488"
  fusion_heat: "21"
  evaporation_heat: "342"
  thermal_conductivity: "93.9"
  debye_temperature: "460.00"
  pauling_negativity_number: "1.66"
  first_ionizing_energy: "652.4"
  oxidation_states: "6, 3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#8309; 4s&#185;"
  lattice_structure: BCC
  lattice_constant: "2.880"
  lattice_ca_ratio: n/a
  appearance: "Very hard, crystalline, steel-gray metal"
  discovery_date: 1797 (France)
  discovered_by: Louis Vauquelin
  named_after: "Greek: chroma (color)"
  pos_x: 5
  pos_y: 3
- name: Manganese
  symbol: Mn
  atomic_number: 25
  atomic_weigth: "54.93805"
  density: "7.21"
  melting_point: "1517"
  boiling_point: "2235"
  atomic_radius: "135"
  covalent_radius: "117"
  ionic_radius: (+7e) 46 (+2e) 80
  atomic_volume: "7.39"
  specific_heat: "0.477"
  fusion_heat: "13.4"
  evaporation_heat: "221"
  thermal_conductivity: (7.8)
  debye_temperature: "400.00"
  pauling_negativity_number: "1.55"
  first_ionizing_energy: "716.8"
  oxidation_states: "7, 6, 5, 4, 3, 2, 0, 1, -1"
  eletronic_configuration: "[Ar] 3d&#8309; 4s&#178;"
  lattice_structure: CUB
  lattice_constant: "8.890"
  lattice_ca_ratio: n/a
  appearance: "Hard, brittle, gray-white metal"
  discovery_date: 1774 (Sweden)
  discovered_by: Johann Gahn
  named_after: "Latin: magnes (magnet); Italian: manganese"
  pos_x: 6
  pos_y: 3
- name: Iron
  symbol: Fe
  atomic_number: 26
  atomic_weigth: "55.847"
  density: "7.874"
  melting_point: 1808-1812
  boiling_point: 3023-3134
  atomic_radius: "126"
  covalent_radius: "117"
  ionic_radius: (+3e) 64 (+2e) 74
  atomic_volume: "7.1"
  specific_heat: "25.14"
  fusion_heat: "13.8"
  evaporation_heat: ~340
  thermal_conductivity: "80.4"
  debye_temperature: "460.00"
  pauling_negativity_number: "1.83"
  first_ionizing_energy: "759.1"
  oxidation_states: "6, 3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#8310; 4s&#178;"
  lattice_structure: BCC
  lattice_constant: "2.870"
  lattice_ca_ratio: n/a
  appearance: "Malleable, ductile, silvery-white metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: iron; symbol from Latin: ferrum (iron)"
  pos_x: 7
  pos_y: 3
- name: Cobalt
  symbol: Co
  atomic_number: 27
  atomic_weigth: "58.9332"
  density: "8.9"
  melting_point: "1768"
  boiling_point: "3143"
  atomic_radius: "125"
  covalent_radius: "116"
  ionic_radius: (+3e) 63 (+2e) 72
  atomic_volume: "6.7"
  specific_heat: "0.456"
  fusion_heat: "15.48"
  evaporation_heat: "389.1"
  thermal_conductivity: "100"
  debye_temperature: "385.00"
  pauling_negativity_number: "1.88"
  first_ionizing_energy: "758.1"
  oxidation_states: "3, 2, 0, -1"
  eletronic_configuration: "[Ar] 3d&#8311; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.510"
  lattice_ca_ratio: n/a
  appearance: "Hard, ductile, lustrous bluish-gray metal"
  discovery_date: 1735 (Sweden)
  discovered_by: George Brandt
  named_after: "German: kobold (goblin)"
  pos_x: 8
  pos_y: 3
- name: Nickel
  symbol: Ni
  atomic_number: 28
  atomic_weigth: "58.6934"
  density: "8.902"
  melting_point: "1726"
  boiling_point: "3005"
  atomic_radius: "124"
  covalent_radius: "115"
  ionic_radius: (+2e) 69
  atomic_volume: "6.6"
  specific_heat: "0.443"
  fusion_heat: "17.61"
  evaporation_heat: "378.6"
  thermal_conductivity: "90.9"
  debye_temperature: "375.00"
  pauling_negativity_number: "1.91"
  first_ionizing_energy: "736.2"
  oxidation_states: "3, 2, 0"
  eletronic_configuration: "[Ar] 3d&#8312; 4s&#178;"
  lattice_structure: FCC
  lattice_constant: "3.520"
  lattice_ca_ratio: n/a
  appearance: "Hard, malleable, silvery-white metal"
  discovery_date: 1751 (Sweden)
  discovered_by: Axel Cronstedt
  named_after: "German: kupfernickel (false coppernickel (goblin)"
  pos_x: 9
  pos_y: 3
- name: Copper
  symbol: Cu
  atomic_number: 29
  atomic_weigth: "63.546"
  density: 8.92-8.96
  melting_point: 1356.55-1356.6
  boiling_point: 2840-2840.15
  atomic_radius: "128"
  covalent_radius: "117"
  ionic_radius: (+2e) 72 (+1e) 96
  atomic_volume: "7.1"
  specific_heat: "0.385"
  fusion_heat: "13.01"
  evaporation_heat: "304.6"
  thermal_conductivity: "401"
  debye_temperature: "315.00"
  pauling_negativity_number: "1.90"
  first_ionizing_energy: "745.0"
  oxidation_states: "3, 2, 1, 0"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#185;"
  lattice_structure: FCC
  lattice_constant: "3.610"
  lattice_ca_ratio: n/a
  appearance: "Malleable, ductile, reddish-brown metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Symbol from Latin: cuprum (island of Cyprus famed for its copper mines)"
  pos_x: 10
  pos_y: 3
- name: Zinc
  symbol: Zn
  atomic_number: 30
  atomic_weigth: "65.38"
  density: "7.133"
  melting_point: 692.73-692.75
  boiling_point: 1179.35-1180
  atomic_radius: "138"
  covalent_radius: "125"
  ionic_radius: (+2e) 74
  atomic_volume: "9.2"
  specific_heat: "0.388"
  fusion_heat: "7.28"
  evaporation_heat: "114.8"
  thermal_conductivity: "116"
  debye_temperature: "234.00"
  pauling_negativity_number: "1.65"
  first_ionizing_energy: "905.8"
  oxidation_states: "2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.660"
  lattice_ca_ratio: n/a
  appearance: "Bluish-silver, ductile metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "German: Zink (German for zinc)"
  pos_x: 11
  pos_y: 3
- name: Gallium
  symbol: Ga
  atomic_number: 31
  atomic_weigth: "69.723"
  density: 5.904-5.91
  melting_point: "302.93"
  boiling_point: "2477"
  atomic_radius: 130-141
  covalent_radius: 122-126
  ionic_radius: (+3e) 62 (+1e) 81
  atomic_volume: "11.8"
  specific_heat: "0.372"
  fusion_heat: "5.59"
  evaporation_heat: "270.3"
  thermal_conductivity: "28.1"
  debye_temperature: "240.00"
  pauling_negativity_number: "1.81"
  first_ionizing_energy: "578.7"
  oxidation_states: "3"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#185;"
  lattice_structure: ORC
  lattice_constant: "4.510"
  lattice_ca_ratio: n/a
  appearance: "Soft, blue-white metal"
  discovery_date: 1875 (France)
  discovered_by: Paul-Emile Lecoq de Boisbaudran
  named_after: "Latin: Gallia (France)"
  pos_x: 12
  pos_y: 3
- name: Germanium
  symbol: Ge
  atomic_number: 32
  atomic_weigth: "72.630"
  density: "5.323"
  melting_point: "1210.6"
  boiling_point: "3103"
  atomic_radius: 122.5-137
  covalent_radius: "122"
  ionic_radius: (+4e)53 (+2e)73
  atomic_volume: "13.6"
  specific_heat: "0.322"
  fusion_heat: "36.8"
  evaporation_heat: "328"
  thermal_conductivity: "60.2"
  debye_temperature: "360.00"
  pauling_negativity_number: "2.01"
  first_ionizing_energy: "760.0"
  oxidation_states: "4, 2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#178;"
  lattice_structure: DIA
  lattice_constant: "5.660"
  lattice_ca_ratio: n/a
  appearance: Grayish-white metal
  discovery_date: 1886 (Germany)
  discovered_by: Clemens Winkler
  named_after: "Latin: Germania (Germany)"
  pos_x: 13
  pos_y: 3
- name: Arsenic
  symbol: As
  atomic_number: 33
  atomic_weigth: "74.92160"
  density: 5.73 (grey arsenic)
  melting_point: "1090"
  boiling_point: 876-886
  atomic_radius: "139"
  covalent_radius: "120"
  ionic_radius: (+5e)46 (-3e)222
  atomic_volume: "13.1"
  specific_heat: "0.328"
  fusion_heat: 24.44 (grey arsenic)
  evaporation_heat: "32.4"
  thermal_conductivity: (50.2)
  debye_temperature: "285.00"
  pauling_negativity_number: "2.18"
  first_ionizing_energy: "946.2"
  oxidation_states: "5, 3, -3"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#179;"
  lattice_structure: TRG
  lattice_constant: "4.130"
  lattice_ca_ratio: n/a
  appearance: "Steel gray, brittle semimetal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Greek: arsenikon; Latin: arsenicum (both names for yellow pigment)"
  pos_x: 14
  pos_y: 3
- name: Selenium
  symbol: Se
  atomic_number: 34
  atomic_weigth: "78.96"
  density: "4.79"
  melting_point: "490"
  boiling_point: "958.1"
  atomic_radius: "140"
  covalent_radius: "116"
  ionic_radius: (+6e)42 (-2e)191
  atomic_volume: "16.5"
  specific_heat: 0.321 (Se-Se)
  fusion_heat: "5.23"
  evaporation_heat: "59.7"
  thermal_conductivity: "0.52"
  debye_temperature: "90"
  pauling_negativity_number: "2.55"
  first_ionizing_energy: "940.4"
  oxidation_states: "6, 4, -2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#8308;"
  lattice_structure: HEX
  lattice_constant: "4.360"
  lattice_ca_ratio: n/a
  appearance: A soft metalloid similar to sulfur
  discovery_date: 1817 (Sweden)
  discovered_by: Jons Jacob Berzelius
  named_after: "Greek: selena (moon)"
  pos_x: 15
  pos_y: 3
- name: Bromine
  symbol: Br
  atomic_number: 35
  atomic_weigth: "79.904"
  density: 3.102-3.12
  melting_point: "265.9"
  boiling_point: "331.9"
  atomic_radius: n/a
  covalent_radius: "114"
  ionic_radius: (+5e)47 (-1e)196
  atomic_volume: "23.5"
  specific_heat: 0.473 (Br-Br)
  fusion_heat: 10.57 (Br-Br)
  evaporation_heat: 29.56 (Br-Br)
  thermal_conductivity: "0.005"
  debye_temperature: n/a
  pauling_negativity_number: "2.96"
  first_ionizing_energy: "1142.0"
  oxidation_states: "7, 5, 3, 1, 0, -1"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#8309;"
  lattice_structure: ORC
  lattice_constant: "6.670"
  lattice_ca_ratio: n/a
  appearance: Reddish-brown liquid
  discovery_date: 1826 (France)
  discovered_by: Antoine J. Balard
  named_after: "Greek: bromos (stench)"
  pos_x: 16
  pos_y: 3
- name: Krypton
  symbol: Kr
  atomic_number: 36
  atomic_weigth: "83.798"
  density: 2.155 (@ -153 °C)
  melting_point: "116.6"
  boiling_point: "120.85"
  atomic_radius: (88)
  covalent_radius: 112-116
  ionic_radius: "169"
  atomic_volume: "32.2"
  specific_heat: "0.247"
  fusion_heat: n/a
  evaporation_heat: "9.05"
  thermal_conductivity: "0.0095"
  debye_temperature: "72"
  pauling_negativity_number: 0.0-3.0
  first_ionizing_energy: "1350.0"
  oxidation_states: "2"
  eletronic_configuration: "[Ar] 3d&#185;&#8304; 4s&#178; 4p&#8310;"
  lattice_structure: FCC
  lattice_constant: "5.720"
  lattice_ca_ratio: n/a
  appearance: "Dense, colorless, odorless, and tasteless gas"
  discovery_date: 1898 (Great Britain)
  discovered_by: "Sir William Ramsey, M.W. Travers"
  named_after: "Greek: kryptos (hidden)"
  pos_x: 17
  pos_y: 3
- name: Rubidium
  symbol: Rb
  atomic_number: 37
  atomic_weigth: "85.4678"
  density: "1.532"
  melting_point: "312.2"
  boiling_point: 961-961.15
  atomic_radius: "248"
  covalent_radius: "216"
  ionic_radius: (+1e)147
  atomic_volume: "55.9"
  specific_heat: "0.360"
  fusion_heat: "2.20"
  evaporation_heat: "75.8"
  thermal_conductivity: "58.2"
  debye_temperature: "56"
  pauling_negativity_number: "0.82"
  first_ionizing_energy: "402.8"
  oxidation_states: "1"
  eletronic_configuration: "[Kr] 5s&#185;"
  lattice_structure: BCC
  lattice_constant: "5.590"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white, highly reactive metal"
  discovery_date: 1861 (Germany)
  discovered_by: "Gustov Kirchoff, Robert Bunsen"
  named_after: "Latin: rubidus (deep red); the color its salts impart to flames"
  pos_x: 0
  pos_y: 4
- name: Strontium
  symbol: Sr
  atomic_number: 38
  atomic_weigth: "87.62"
  density: "2.54"
  melting_point: "1042"
  boiling_point: "1657"
  atomic_radius: "215"
  covalent_radius: "191"
  ionic_radius: (+2e) 112
  atomic_volume: "33.7"
  specific_heat: "0.301"
  fusion_heat: "9.20"
  evaporation_heat: "144"
  thermal_conductivity: (35.4)
  debye_temperature: "147"
  pauling_negativity_number: "0.95"
  first_ionizing_energy: "549.0"
  oxidation_states: "2"
  eletronic_configuration: "[Kr] 5s&#178;"
  lattice_structure: FCC
  lattice_constant: "6.080"
  lattice_ca_ratio: n/a
  appearance: "Silvery, malleable metal"
  discovery_date: 1790 (Scotland)
  discovered_by: A. Crawford
  named_after: "The Scottish town, Strontian"
  pos_x: 1
  pos_y: 4
- name: Yttrium
  symbol: Y
  atomic_number: 39
  atomic_weigth: "88.90585"
  density: "4.47"
  melting_point: "1795"
  boiling_point: "3611"
  atomic_radius: "178"
  covalent_radius: "162"
  ionic_radius: (+3e) 89.3
  atomic_volume: "19.8"
  specific_heat: "0.284"
  fusion_heat: "11.5"
  evaporation_heat: "367"
  thermal_conductivity: (17.2)
  debye_temperature: "280"
  pauling_negativity_number: "1.22"
  first_ionizing_energy: "615.4"
  oxidation_states: "3"
  eletronic_configuration: "[Kr] 4d&#185; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.650"
  lattice_ca_ratio: "1.571"
  appearance: "Silvery, ductile, fairly reactive metal"
  discovery_date: 1794 (Finland)
  discovered_by: Johann Gadolin
  named_after: "Named after the Swedish town, Ytterby, where one of its minerals was first found"
  pos_x: 2
  pos_y: 4
- name: Zirconium
  symbol: Zr
  atomic_number: 40
  atomic_weigth: "91.224"
  density: "6.506"
  melting_point: "2125"
  boiling_point: "4650"
  atomic_radius: "160"
  covalent_radius: "145"
  ionic_radius: (+4e)79
  atomic_volume: "14.1"
  specific_heat: "0.281"
  fusion_heat: "19.2"
  evaporation_heat: "567"
  thermal_conductivity: "22.7"
  debye_temperature: 250-291
  pauling_negativity_number: "1.33"
  first_ionizing_energy: "659.7"
  oxidation_states: "0, 1, 2, 3, 4"
  eletronic_configuration: "[Kr] 4d&#178; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.230"
  lattice_ca_ratio: "1.593"
  appearance: "Gray-white, lustrous, corrosion-resistant metal"
  discovery_date: 1789 (Germany)
  discovered_by: Martin Klaproth
  named_after: "The mineral, zircon"
  pos_x: 3
  pos_y: 4
- name: Niobium
  symbol: Nb
  atomic_number: 41
  atomic_weigth: "92.90638"
  density: "8.57"
  melting_point: "2741"
  boiling_point: "5015"
  atomic_radius: "146"
  covalent_radius: 134-164
  ionic_radius: (+5e)69
  atomic_volume: "10.8"
  specific_heat: "0.268"
  fusion_heat: "26.8"
  evaporation_heat: "680"
  thermal_conductivity: "53.7"
  debye_temperature: "275.00"
  pauling_negativity_number: "1.6"
  first_ionizing_energy: "663.6"
  oxidation_states: "5, 4, 3, 2, 1"
  eletronic_configuration: "[Kr] 4d&#8308; 5s&#185;"
  lattice_structure: BCC
  lattice_constant: "3.300"
  lattice_ca_ratio: n/a
  appearance: "Shiny white, soft, ductile metal"
  discovery_date: 1801 (England)
  discovered_by: Charles Hatchet
  named_after: Niobe; daughter of the mythical Greek king Tantalus
  pos_x: 4
  pos_y: 4
- name: Molybdenum
  symbol: Mo
  atomic_number: 42
  atomic_weigth: "95.96"
  density: "10.22"
  melting_point: "2890"
  boiling_point: "4885"
  atomic_radius: "139"
  covalent_radius: "130"
  ionic_radius: " (+6e) 62 (+4e) 70"
  atomic_volume: "9.4"
  specific_heat: "0.251"
  fusion_heat: "28"
  evaporation_heat: ~590
  thermal_conductivity: (138)
  debye_temperature: 380.00-450.00
  pauling_negativity_number: "2.16"
  first_ionizing_energy: "684.8"
  oxidation_states: "6, 5, 4, 3, 2, 0"
  eletronic_configuration: "[Kr] 4d&#8309; 5s&#185;"
  lattice_structure: BCC
  lattice_constant: "3.150"
  lattice_ca_ratio: n/a
  appearance: "Silvery white, hard metal"
  discovery_date: 1778 (Sweden)
  discovered_by: Carl Wilhelm Scheele
  named_after: "Greek: molybdos (lead)"
  pos_x: 5
  pos_y: 4
- name: Technetium
  symbol: Tc
  atomic_number: 43
  atomic_weigth: "97.9072"
  density: "11.5"
  melting_point: 2430-2445
  boiling_point: 4538-5150
  atomic_radius: "136"
  covalent_radius: "127"
  ionic_radius: (+7e)56
  atomic_volume: "8.5"
  specific_heat: "0.243"
  fusion_heat: "23.8"
  evaporation_heat: "585"
  thermal_conductivity: "50.6"
  debye_temperature: "453"
  pauling_negativity_number: "1.9"
  first_ionizing_energy: "702.2"
  oxidation_states: "-1, 0, 1, 2, 3, 4, 5, 6, 7"
  eletronic_configuration: "[Kr] 4d&#8309; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.740"
  lattice_ca_ratio: "1.604"
  appearance: Silvery-gray metal
  discovery_date: 1937 (Italy)
  discovered_by: "Carlo Perrier, Emilio Segre"
  named_after: "Greek: technetos (artificial)"
  pos_x: 6
  pos_y: 4
- name: Ruthenium
  symbol: Ru
  atomic_number: 44
  atomic_weigth: "101.07"
  density: "12.41"
  melting_point: 2583-2607.15
  boiling_point: 4173-4350.15
  atomic_radius: "134"
  covalent_radius: "125"
  ionic_radius: (+4e) 67
  atomic_volume: "8.3"
  specific_heat: "0.238"
  fusion_heat: (25.5)
  evaporation_heat: n/a
  thermal_conductivity: "117.0"
  debye_temperature: "600"
  pauling_negativity_number: "2.2"
  first_ionizing_energy: "710.3"
  oxidation_states: "2, 3, 4, 6, 8, 0, -2"
  eletronic_configuration: "[Kr] 4d&#8311; 5s&#185;"
  lattice_structure: HEX
  lattice_constant: "2.700"
  lattice_ca_ratio: "1.584"
  appearance: "Rare, silver-gray, extremely brittle metal"
  discovery_date: 1844 (Russia)
  discovered_by: Karl Klaus
  named_after: "Latin: Ruthenia (Russia)"
  pos_x: 7
  pos_y: 4
- name: Rhodium
  symbol: Rh
  atomic_number: 45
  atomic_weigth: "102.9055"
  density: "12.41"
  melting_point: "2239"
  boiling_point: "4000"
  atomic_radius: "134"
  covalent_radius: "125"
  ionic_radius: (+3e)68
  atomic_volume: "8.3"
  specific_heat: "0.244"
  fusion_heat: "21.8"
  evaporation_heat: "494"
  thermal_conductivity: "150"
  debye_temperature: "480"
  pauling_negativity_number: "2.28"
  first_ionizing_energy: "719.5"
  oxidation_states: "5, 4, 3,  2, 1, 0"
  eletronic_configuration: "[Kr] 4d&#8312; 5s&#185;"
  lattice_structure: FCC
  lattice_constant: "3.800"
  lattice_ca_ratio: n/a
  appearance: "Silvery white, hard metal"
  discovery_date: 1803 (England)
  discovered_by: William Wollaston
  named_after: "Greek: rhodon (rose); its salts give a rosy solution"
  pos_x: 8
  pos_y: 4
- name: Palladium
  symbol: Pd
  atomic_number: 46
  atomic_weigth: "106.42"
  density: "12.02"
  melting_point: 1825-1827
  boiling_point: 2940-3413
  atomic_radius: "137"
  covalent_radius: "128"
  ionic_radius: (+4e) 65 (+2e) 80
  atomic_volume: "8.9"
  specific_heat: "0.244"
  fusion_heat: "17.24"
  evaporation_heat: "372.4"
  thermal_conductivity: "71.8"
  debye_temperature: 274.00-275.00
  pauling_negativity_number: "2.20"
  first_ionizing_energy: "803.5"
  oxidation_states: "0, 1, 2, 3, 4, 5, 6"
  eletronic_configuration: "[Kr] 4d&#185;&#8304;"
  lattice_structure: FCC
  lattice_constant: "3.890"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, soft, malleable and ductile metal"
  discovery_date: 1803 (England)
  discovered_by: William Wollaston
  named_after: "Named after the asteroid, Pallas, discovered in 1803"
  pos_x: 9
  pos_y: 4
- name: Silver
  symbol: Ag
  atomic_number: 47
  atomic_weigth: "107.8682"
  density: "10.5"
  melting_point: "1235.1"
  boiling_point: "2485"
  atomic_radius: "144"
  covalent_radius: "134"
  ionic_radius: (+2e) 89 (+1e) 126
  atomic_volume: "10.3"
  specific_heat: "0.237"
  fusion_heat: "11.95"
  evaporation_heat: "254.1"
  thermal_conductivity: "429"
  debye_temperature: 215.00-225.00
  pauling_negativity_number: "1.93"
  first_ionizing_energy: 730.5; 2070; 3361
  oxidation_states: "2, 1"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#185;"
  lattice_structure: FCC
  lattice_constant: "4.090"
  lattice_ca_ratio: n/a
  appearance: "Silvery-ductile, and malleable metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: siolful (silver); symbol from Latin: argentum"
  pos_x: 10
  pos_y: 4
- name: Cadmium
  symbol: Cd
  atomic_number: 48
  atomic_weigth: "112.411"
  density: "8.65"
  melting_point: 594.1-594.22
  boiling_point: 1038-1040
  atomic_radius: 154-161
  covalent_radius: "148"
  ionic_radius: (+2e) 97
  atomic_volume: "13.1"
  specific_heat: "0.232"
  fusion_heat: "6.11"
  evaporation_heat: 59.1-100
  thermal_conductivity: "96.9"
  debye_temperature: "209.00"
  pauling_negativity_number: "1.69"
  first_ionizing_energy: 867.2-867.8
  oxidation_states: "2"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.980"
  lattice_ca_ratio: "1.886"
  appearance: "Soft, malleable, blue-white metal"
  discovery_date: 1817 (Germany)
  discovered_by: Fredrich Stromeyer
  named_after: "Greek: kadmia (ancient name for calamine (zinc oxide))"
  pos_x: 11
  pos_y: 4
- name: Indium
  symbol: In
  atomic_number: 49
  atomic_weigth: "114.818"
  density: "7.31"
  melting_point: "429.32"
  boiling_point: "2353"
  atomic_radius: "166"
  covalent_radius: "144"
  ionic_radius: (+3e) 81
  atomic_volume: "15.7"
  specific_heat: "0.234"
  fusion_heat: "3.24"
  evaporation_heat: "225.1"
  thermal_conductivity: "81.8"
  debye_temperature: "129.00"
  pauling_negativity_number: "1.78"
  first_ionizing_energy: "558.0"
  oxidation_states: "1, 3"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#185;"
  lattice_structure: TET
  lattice_constant: "4.590"
  lattice_ca_ratio: n/a
  appearance: "Very soft, silvery-white metal"
  discovery_date: 1863 (Germany)
  discovered_by: "Ferdinand Reich, T. Richter"
  named_after: "Latin: indicum (color indigo), the color it shows in a spectroscope"
  pos_x: 12
  pos_y: 4
- name: Tin
  symbol: Sn
  atomic_number: 50
  atomic_weigth: "118.71"
  density: "7.31"
  melting_point: 505.05-505.1
  boiling_point: 2543-2873
  atomic_radius: "162"
  covalent_radius: "141"
  ionic_radius: (+4e) 71 (+2) 93
  atomic_volume: "16.3"
  specific_heat: "0.222"
  fusion_heat: "7.07"
  evaporation_heat: "296"
  thermal_conductivity: "66.8"
  debye_temperature: "170.00"
  pauling_negativity_number: "1.96"
  first_ionizing_energy: "708.2"
  oxidation_states: "4, 2"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#178;"
  lattice_structure: TET
  lattice_constant: "5.820"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, soft, malleable and ductile metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Named after Etruscan god, Tinia; symbol from Latin: stannum (tin)"
  pos_x: 13
  pos_y: 4
- name: Antimony
  symbol: Sb
  atomic_number: 51
  atomic_weigth: "121.760"
  density: "6.691"
  melting_point: "903.9"
  boiling_point: "1908"
  atomic_radius: "159"
  covalent_radius: "140"
  ionic_radius: (+6e)62 (−3e)245
  atomic_volume: "18.4"
  specific_heat: "0.205"
  fusion_heat: "20.08"
  evaporation_heat: "195.2"
  thermal_conductivity: "24.43"
  debye_temperature: "200.00"
  pauling_negativity_number: "2.05"
  first_ionizing_energy: "833.3"
  oxidation_states: "5, 3, -3"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#179;"
  lattice_structure: TRG
  lattice_constant: "4.510"
  lattice_ca_ratio: n/a
  appearance: "Hard, silvery-white, brittle semimetal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Greek: anti and monos (not alone); symbol from mineral stibnite"
  pos_x: 14
  pos_y: 4
- name: Tellurium
  symbol: Te
  atomic_number: 52
  atomic_weigth: "127.6"
  density: "6.24"
  melting_point: "722.7"
  boiling_point: "1263"
  atomic_radius: "160"
  covalent_radius: "136"
  ionic_radius: (+6e) 56 211 (−2e)
  atomic_volume: "20.5"
  specific_heat: "0.201"
  fusion_heat: "17.91"
  evaporation_heat: "49.8"
  thermal_conductivity: "14.3"
  debye_temperature: n/a
  pauling_negativity_number: "2.1"
  first_ionizing_energy: "869.0"
  oxidation_states: "6, 4, 2, -2"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#8308;"
  lattice_structure: HEX
  lattice_constant: "4.450"
  lattice_ca_ratio: "1.330"
  appearance: "Silvery-white, brittle semimetal"
  discovery_date: 1782 (Romania)
  discovered_by: Franz Joseph Meller von Reichenstein
  named_after: "Latin: tellus, telluris (Planet Earth)"
  pos_x: 15
  pos_y: 4
- name: Iodine
  symbol: I
  atomic_number: 53
  atomic_weigth: "126.90447"
  density: "4.93"
  melting_point: 386.65-386.85
  boiling_point: 457.4-457.5
  atomic_radius: "136"
  covalent_radius: "133"
  ionic_radius: (+7e) 50 (-1e) 220
  atomic_volume: "25.7"
  specific_heat: 0.427 (I-I)
  fusion_heat: 15.52 (I-I)
  evaporation_heat: 41.95 (I-I)
  thermal_conductivity: (0.45)
  debye_temperature: n/a
  pauling_negativity_number: "2.66"
  first_ionizing_energy: "1008.3"
  oxidation_states: "7, 5, 3, 1, 0, -1"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#8309;"
  lattice_structure: ORC
  lattice_constant: "7.720"
  lattice_ca_ratio: n/a
  appearance: "Shiny, black nonmetallic solid"
  discovery_date: 1811 (France)
  discovered_by: Bernard Courtois
  named_after: "Greek: iodes (violet colored)"
  pos_x: 16
  pos_y: 4
- name: Xenon
  symbol: Xe
  atomic_number: 54
  atomic_weigth: "131.29"
  density: 3.52 (@ -107.05 °C)
  melting_point: "161.3"
  boiling_point: "166.1"
  atomic_radius: (108)
  covalent_radius: 131-140
  ionic_radius: "190"
  atomic_volume: "42.9"
  specific_heat: "0.158"
  fusion_heat: "2.27"
  evaporation_heat: "12.65"
  thermal_conductivity: "0.0057"
  debye_temperature: n/a
  pauling_negativity_number: 0.0-2.6
  first_ionizing_energy: "1170.0"
  oxidation_states: "0, 1, 2, 4, 6, 7, 8"
  eletronic_configuration: "[Kr] 4d&#185;&#8304; 5s&#178; 5p&#8310;"
  lattice_structure: FCC
  lattice_constant: "6.200"
  lattice_ca_ratio: n/a
  appearance: "Heavy, colorless, and odorless noble gas"
  discovery_date: 1898 (England)
  discovered_by: "Sir William Ramsey, M.W. Travers"
  named_after: "Greek: xenos (strange)"
  pos_x: 17
  pos_y: 4
- name: Cesium
  symbol: Cs
  atomic_number: 55
  atomic_weigth: "132.90545"
  density: "1.873"
  melting_point: 301.6-301.85
  boiling_point: 940.75-951.6
  atomic_radius: "267"
  covalent_radius: "235"
  ionic_radius: (+1e) 167
  atomic_volume: "70.0"
  specific_heat: "0.241"
  fusion_heat: "2.09"
  evaporation_heat: "68.3"
  thermal_conductivity: "35.9"
  debye_temperature: "39.2"
  pauling_negativity_number: "0.79"
  first_ionizing_energy: "375.5"
  oxidation_states: "1"
  eletronic_configuration: "[Xe] 6s&#185;"
  lattice_structure: BCC
  lattice_constant: "6.050"
  lattice_ca_ratio: n/a
  appearance: "Very soft, ductile, light gray metal"
  discovery_date: 1860 (Germany)
  discovered_by: "Gustov Kirchoff, Robert Bunsen"
  named_after: "Latin: caesius (sky blue); for the blue lines of its spectrum"
  pos_x: 0
  pos_y: 5
- name: Barium
  symbol: Ba
  atomic_number: 56
  atomic_weigth: "137.327"
  density: "3.5"
  melting_point: "1002"
  boiling_point: "1910"
  atomic_radius: "222"
  covalent_radius: "198"
  ionic_radius: (+2e) 134
  atomic_volume: "39.0"
  specific_heat: "0.192"
  fusion_heat: "7.66"
  evaporation_heat: "142.0"
  thermal_conductivity: (18.4)
  debye_temperature: n/a
  pauling_negativity_number: "0.89"
  first_ionizing_energy: "502.5"
  oxidation_states: "2"
  eletronic_configuration: "[Xe] 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "5.020"
  lattice_ca_ratio: n/a
  appearance: "Soft, slightly malleable, silver-white metal"
  discovery_date: 1808 (England)
  discovered_by: Sir Humphrey Davy
  named_after: "Greek: barys (heavy or dense)"
  pos_x: 1
  pos_y: 5
- name: Lanthanum
  symbol: La
  atomic_number: 57
  atomic_weigth: "138.9055"
  density: 6.15-6.18
  melting_point: 920-1194
  boiling_point: 3447-3730
  atomic_radius: "187"
  covalent_radius: "169"
  ionic_radius: 101.(+3e) 6
  atomic_volume: "22.5"
  specific_heat: "0.197"
  fusion_heat: "8.5"
  evaporation_heat: "402"
  thermal_conductivity: "13.4"
  debye_temperature: "132.00"
  pauling_negativity_number: "1.10"
  first_ionizing_energy: "541.1"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 5d&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.750"
  lattice_ca_ratio: "1.619"
  appearance: "Silvery-white, soft, malleable, and ductile metal"
  discovery_date: 1839 (Sweden)
  discovered_by: Carl Mosander
  named_after: "Greek: lanthanein (to be hidden)"
  pos_x: 3
  pos_y: 7
- name: Cerium
  symbol: Ce
  atomic_number: 58
  atomic_weigth: "140.116"
  density: "6.757"
  melting_point: "1072"
  boiling_point: "3699"
  atomic_radius: "181"
  covalent_radius: "165"
  ionic_radius: (+4e) 92 103.(+3e) 4
  atomic_volume: "21.0"
  specific_heat: "0.205"
  fusion_heat: "5.2"
  evaporation_heat: "398"
  thermal_conductivity: "11.3"
  debye_temperature: n/a
  pauling_negativity_number: "1.12"
  first_ionizing_energy: "540.1"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Xe] 4f&#185; 5d&#185; 6s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.160"
  lattice_ca_ratio: n/a
  appearance: "Malleable, ductile, iron-gray metal"
  discovery_date: 1803 (Sweden/Germany)
  discovered_by: "Wilhelm von Hisinger, Jons Jacob Berzelius, Martin Klaproth"
  named_after: "Named after the asteroid, Ceres, discovered two years before the element"
  pos_x: 4
  pos_y: 7
- name: Praseodymium
  symbol: Pr
  atomic_number: 59
  atomic_weigth: "140.90765"
  density: "6.773"
  melting_point: "1204"
  boiling_point: "3785"
  atomic_radius: "182"
  covalent_radius: "165"
  ionic_radius: (+4e) 90 101.(+3e) 3
  atomic_volume: "20.8"
  specific_heat: "0.192"
  fusion_heat: "11.3"
  evaporation_heat: "331"
  thermal_conductivity: "12.5"
  debye_temperature: n/a
  pauling_negativity_number: "1.13"
  first_ionizing_energy: "526.6"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Xe] 4f&#179; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.670"
  lattice_ca_ratio: "3.22"
  appearance: "Silvery white, moderately soft, malleable, and ductile metal"
  discovery_date: 1885 (Austria)
  discovered_by: C.F. Aver von Welsbach
  named_after: "Greek: prasios and didymos (green twin); from its green salts"
  pos_x: 5
  pos_y: 7
- name: Neodymium
  symbol: Nd
  atomic_number: 60
  atomic_weigth: "144.242"
  density: "7.007"
  melting_point: "1294"
  boiling_point: "3341"
  atomic_radius: "182"
  covalent_radius: "184"
  ionic_radius: 99.(+3e) 5
  atomic_volume: "20.6"
  specific_heat: "0.205"
  fusion_heat: "7.1"
  evaporation_heat: "289"
  thermal_conductivity: (16.5)
  debye_temperature: n/a
  pauling_negativity_number: "1.14"
  first_ionizing_energy: "531.5"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#8308; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.660"
  lattice_ca_ratio: "1.614"
  appearance: "Silvery-white, rare-earth metal that oxidizes easily in air"
  discovery_date: 1925 (Austria)
  discovered_by: C.F. Aver von Welsbach
  named_after: "Greek: neos and didymos (new twin)"
  pos_x: 6
  pos_y: 7
- name: Promethium
  symbol: Pm
  atomic_number: 61
  atomic_weigth: "144.9127"
  density: 7.2-7.26
  melting_point: "1441"
  boiling_point: 3000-~3273
  atomic_radius: 183-185(205)
  covalent_radius: "199"
  ionic_radius: (+3e)97.9 or (+3e)111
  atomic_volume: "19.96"
  specific_heat: "0.185"
  fusion_heat: "7.13"
  evaporation_heat: "330.5"
  thermal_conductivity: "17.9"
  debye_temperature: n/a
  pauling_negativity_number: 1.1-1.13
  first_ionizing_energy: "536"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#8309; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Light-grey, radioactive element"
  discovery_date: 1945 (United States)
  discovered_by: "J.A. Marinsky, L.E. Glendenin, C.D. Coryell"
  named_after: "Named for the Greek god, Prometheus"
  pos_x: 7
  pos_y: 7
- name: Samarium
  symbol: Sm
  atomic_number: 62
  atomic_weigth: "150.36"
  density: "7.520"
  melting_point: "1350"
  boiling_point: "2064"
  atomic_radius: "181"
  covalent_radius: "162"
  ionic_radius: (+3e) 96.4
  atomic_volume: "19.9"
  specific_heat: "0.180"
  fusion_heat: "8.9"
  evaporation_heat: "165"
  thermal_conductivity: (13.3)
  debye_temperature: "166.00"
  pauling_negativity_number: "1.17"
  first_ionizing_energy: "540.1"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#8310; 6s&#178;"
  lattice_structure: RHL
  lattice_constant: "9.000"
  lattice_ca_ratio: n/a
  appearance: Silvery rare-earth metal
  discovery_date: 1880 (France)
  discovered_by: Jean Charles Galissard de Marignac
  named_after: Named after the mineral samarskite
  pos_x: 8
  pos_y: 7
- name: Europium
  symbol: Eu
  atomic_number: 63
  atomic_weigth: "151.964"
  density: "5.243"
  melting_point: "1095"
  boiling_point: "1870"
  atomic_radius: "199"
  covalent_radius: "185"
  ionic_radius: (+3e) 95 (+2e) 109
  atomic_volume: "28.9"
  specific_heat: "0.176"
  fusion_heat: "1095"
  evaporation_heat: "176"
  thermal_conductivity: "13.9"
  debye_temperature: n/a
  pauling_negativity_number: 0.0-1.2
  first_ionizing_energy: "546.9"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#8311; 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "4.610"
  lattice_ca_ratio: n/a
  appearance: "Soft, silvery-white metal"
  discovery_date: 1901 (France)
  discovered_by: Eugene-Antole Demarcay
  named_after: Named for the continent of Europe
  pos_x: 9
  pos_y: 7
- name: Gadolinium
  symbol: Gd
  atomic_number: 64
  atomic_weigth: "157.25"
  density: "7.900"
  melting_point: "1586"
  boiling_point: "3539"
  atomic_radius: "179"
  covalent_radius: "161"
  ionic_radius: (+3e) 93.8
  atomic_volume: "19.9"
  specific_heat: "0.230"
  fusion_heat: "10.0"
  evaporation_heat: "398"
  thermal_conductivity: (10.5)
  debye_temperature: n/a
  pauling_negativity_number: "1.20"
  first_ionizing_energy: "594.2"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#8311; 5d&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.640"
  lattice_ca_ratio: "1.588"
  appearance: "Soft, ductile, silvery-white metal"
  discovery_date: 1880 (Switzerland)
  discovered_by: Jean Charles Galissard de Marignac
  named_after: Named after the mineral gadolinite
  pos_x: 10
  pos_y: 7
- name: Terbium
  symbol: Tb
  atomic_number: 65
  atomic_weigth: "158.92535"
  density: "8.229"
  melting_point: "1629"
  boiling_point: "3296"
  atomic_radius: "180"
  covalent_radius: "159"
  ionic_radius: (+4e) 84 (+3e) 92.3
  atomic_volume: "19.2"
  specific_heat: "0.183"
  fusion_heat: n/a
  evaporation_heat: "389"
  thermal_conductivity: "11.1"
  debye_temperature: n/a
  pauling_negativity_number: "1.2"
  first_ionizing_energy: "569"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Xe] 4f&#8313; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.600"
  lattice_ca_ratio: "1.581"
  appearance: "Soft, ductile, silvery-gray, rare-earth metal"
  discovery_date: 1843 (Sweden)
  discovered_by: Carl Mosander
  named_after: "Named after the Swedish town, Ytterby"
  pos_x: 11
  pos_y: 7
- name: Dysprosium
  symbol: Dy
  atomic_number: 66
  atomic_weigth: "162.50"
  density: "8.55"
  melting_point: "1685"
  boiling_point: "2835"
  atomic_radius: "180"
  covalent_radius: "159"
  ionic_radius: (+3e) 90.8
  atomic_volume: "19.0"
  specific_heat: "0.173"
  fusion_heat: n/a
  evaporation_heat: "291"
  thermal_conductivity: "10.7"
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: "567"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#8304; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.590"
  lattice_ca_ratio: "1.573"
  appearance: "Soft. lustrous, silvery metal"
  discovery_date: 1886 (France)
  discovered_by: Paul-Emile Lecoq de Boisbaudran
  named_after: "Greek: dysprositos (hard to get at)"
  pos_x: 12
  pos_y: 7
- name: Holmium
  symbol: Ho
  atomic_number: 67
  atomic_weigth: "164.93032"
  density: "8.795"
  melting_point: "1747"
  boiling_point: "2968"
  atomic_radius: "179"
  covalent_radius: "158"
  ionic_radius: (+3e) 89.4
  atomic_volume: "18.7"
  specific_heat: "0.164"
  fusion_heat: n/a
  evaporation_heat: "301"
  thermal_conductivity: (16.2)
  debye_temperature: n/a
  pauling_negativity_number: "1.23"
  first_ionizing_energy: "574"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.580"
  lattice_ca_ratio: "1.570"
  appearance: "Fairly soft, malleable, lustrous, silvery metal"
  discovery_date: 1879 (Switzerland)
  discovered_by: J.L. Soret
  named_after: "Holmia, the Latinized name for Stockholm, Sweden"
  pos_x: 13
  pos_y: 7
- name: Erbium
  symbol: Er
  atomic_number: 68
  atomic_weigth: "167.259"
  density: "9.06"
  melting_point: "1802"
  boiling_point: "3136"
  atomic_radius: "178"
  covalent_radius: "157"
  ionic_radius: (+3e) 88.1
  atomic_volume: "18.4"
  specific_heat: "0.168"
  fusion_heat: n/a
  evaporation_heat: "317"
  thermal_conductivity: (14.5)
  debye_temperature: n/a
  pauling_negativity_number: "1.24"
  first_ionizing_energy: "581"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#178; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.560"
  lattice_ca_ratio: "1.570"
  appearance: "Soft, malleable, silvery metal"
  discovery_date: 1843 (Sweden)
  discovered_by: Carl Mosander
  named_after: "Named after the Swedish town, Ytterby"
  pos_x: 14
  pos_y: 7
- name: Thulium
  symbol: Tm
  atomic_number: 69
  atomic_weigth: "168.93421"
  density: "9.321"
  melting_point: "1818"
  boiling_point: "2220"
  atomic_radius: "177"
  covalent_radius: "156"
  ionic_radius: (+3e) 87
  atomic_volume: "18.1"
  specific_heat: "0.160"
  fusion_heat: n/a
  evaporation_heat: "232"
  thermal_conductivity: (16.9)
  debye_temperature: n/a
  pauling_negativity_number: "1.25"
  first_ionizing_energy: "589"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#185;&#179; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.540"
  lattice_ca_ratio: "1.570"
  appearance: "Soft, malleable, ductile, silvery metal"
  discovery_date: 1879 (Sweden)
  discovered_by: Per Theodor Cleve
  named_after: "Thule, ancient name of Scandinavia"
  pos_x: 15
  pos_y: 7
- name: Ytterbium
  symbol: Yb
  atomic_number: 70
  atomic_weigth: "173.055"
  density: "6.9654"
  melting_point: "1097"
  boiling_point: "1466"
  atomic_radius: "194"
  covalent_radius: "170"
  ionic_radius: (+3e) 85.8 (+2e) 93
  atomic_volume: "24.8"
  specific_heat: "0.145"
  fusion_heat: "3.35"
  evaporation_heat: "159"
  thermal_conductivity: (34.9)
  debye_temperature: n/a
  pauling_negativity_number: "1.1"
  first_ionizing_energy: "603"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 6s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.490"
  lattice_ca_ratio: n/a
  appearance: "Silvery, lustrous, malleable, and ductile metal"
  discovery_date: 1878 (Switzerland)
  discovered_by: Jean Charles Galissard de Marignac
  named_after: "Named after the Swedish town, Ytterby"
  pos_x: 16
  pos_y: 7
- name: Lutetium
  symbol: Lu
  atomic_number: 71
  atomic_weigth: "174.9668"
  density: "9.8404"
  melting_point: "1936"
  boiling_point: "3668"
  atomic_radius: "175"
  covalent_radius: "156"
  ionic_radius: (+3e) 85
  atomic_volume: "17.8"
  specific_heat: "0.155"
  fusion_heat: n/a
  evaporation_heat: "414"
  thermal_conductivity: (16.4)
  debye_temperature: n/a
  pauling_negativity_number: "1.27"
  first_ionizing_energy: "513"
  oxidation_states: "3"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.510"
  lattice_ca_ratio: "1.585"
  appearance: "Silvery-white, hard, dense, rare-earth metal"
  discovery_date: 1907 (France)
  discovered_by: Georges Urbain
  named_after: "Named for the ancient name of Paris: Lutetia Parisorum"
  pos_x: 17
  pos_y: 7
- name: Hafnium
  symbol: Hf
  atomic_number: 72
  atomic_weigth: "178.49"
  density: "13.31"
  melting_point: 2503-2506
  boiling_point: 4876-5470
  atomic_radius: "167"
  covalent_radius: "144"
  ionic_radius: (+4e) 78
  atomic_volume: "13.6"
  specific_heat: "0.146"
  fusion_heat: (25.1)
  evaporation_heat: "575"
  thermal_conductivity: "23.0"
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "575.2"
  oxidation_states: "4"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#178; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "3.200"
  lattice_ca_ratio: "1.582"
  appearance: "Silvery, ductile metal"
  discovery_date: 1923 (Denmark)
  discovered_by: "Dirk Coster, Georg von Hevesy"
  named_after: "Hafnia, the Latin name of Copenhagen"
  pos_x: 3
  pos_y: 5
- name: Tantalum
  symbol: Ta
  atomic_number: 73
  atomic_weigth: "180.9479"
  density: 16.65-16.654
  melting_point: 3269-3290
  boiling_point: 5698-5731
  atomic_radius: "149"
  covalent_radius: "134"
  ionic_radius: (+5e) 68
  atomic_volume: "10.9"
  specific_heat: "0.140"
  fusion_heat: "24.7"
  evaporation_heat: "758"
  thermal_conductivity: "57.5"
  debye_temperature: "225.00"
  pauling_negativity_number: "1.5"
  first_ionizing_energy: "760.1"
  oxidation_states: "5"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#179; 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "3.310"
  lattice_ca_ratio: n/a
  appearance: "Gray, heavy, hard metal"
  discovery_date: 1802 (Sweden)
  discovered_by: Anders Ekeberg
  named_after: "King Tantalus of Greek mythology, father of Niobe"
  pos_x: 4
  pos_y: 5
- name: Tungsten
  symbol: W
  atomic_number: 74
  atomic_weigth: "183.84"
  density: 19.25-19.3
  melting_point: 3680-3695
  boiling_point: 5828-5930
  atomic_radius: "141"
  covalent_radius: 130-170
  ionic_radius: (+6e) 62 (+4e) 70
  atomic_volume: "9.53"
  specific_heat: "0.133"
  fusion_heat: "35"
  evaporation_heat: "824"
  thermal_conductivity: "173"
  debye_temperature: "310.00"
  pauling_negativity_number: 1.7-2.3
  first_ionizing_energy: "769.7"
  oxidation_states: "6, 5, 4, 3, 2, 0"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8308; 6s&#178;"
  lattice_structure: BCC
  lattice_constant: "3.160"
  lattice_ca_ratio: n/a
  appearance: "Tough, steel-gray to white metal"
  discovery_date: 1783 (Spain)
  discovered_by: "Juan Jose, Fausto Elhuyar"
  named_after: "Swedish: tung sten (heavy stone); symbol from its German name wolfram"
  pos_x: 5
  pos_y: 5
- name: Rhenium
  symbol: Re
  atomic_number: 75
  atomic_weigth: "186.207"
  density: "21.02"
  melting_point: 3453-3459
  boiling_point: 5869-5900
  atomic_radius: "137"
  covalent_radius: "128"
  ionic_radius: (+7e) 53 (+4e) 72
  atomic_volume: "8.85"
  specific_heat: "0.138"
  fusion_heat: "34"
  evaporation_heat: "704"
  thermal_conductivity: "48.0"
  debye_temperature: "416.00"
  pauling_negativity_number: "1.9"
  first_ionizing_energy: "759.1"
  oxidation_states: "7, 6, 5, 4, 3, 2, -1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8309; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.760"
  lattice_ca_ratio: "1.615"
  appearance: "Dense, silvery-white metal"
  discovery_date: 1925 (Germany)
  discovered_by: "Walter Noddack, Ida Tacke, Otto Berg"
  named_after: "Latin: Rhenus, the Rhine River"
  pos_x: 6
  pos_y: 5
- name: Osmium
  symbol: Os
  atomic_number: 76
  atomic_weigth: "190.23"
  density: 22.57-22.61
  melting_point: 3306-3327
  boiling_point: 5285-5300
  atomic_radius: "135"
  covalent_radius: "126"
  ionic_radius: (+6e) 69 (+4e) 88
  atomic_volume: "8.43"
  specific_heat: "0.131"
  fusion_heat: "31.7"
  evaporation_heat: "738"
  thermal_conductivity: (87.6)
  debye_temperature: n/a
  pauling_negativity_number: "2.2"
  first_ionizing_energy: "819.8"
  oxidation_states: "8, 6, 4, 3, 2, 0, -2"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8310; 6s&#178;"
  lattice_structure: HEX
  lattice_constant: "2.740"
  lattice_ca_ratio: "1.579"
  appearance: "Blue-white, lustrous, hard metal"
  discovery_date: 1804 (England)
  discovered_by: Smithson Tenant
  named_after: "Greek: osme (odor)"
  pos_x: 7
  pos_y: 5
- name: Iridium
  symbol: Ir
  atomic_number: 77
  atomic_weigth: "192.217"
  density: 22.42-22.65
  melting_point: 2683-2739
  boiling_point: 4403-4701
  atomic_radius: "136"
  covalent_radius: "127"
  ionic_radius: (+4e) 68
  atomic_volume: "8.54"
  specific_heat: "0.133"
  fusion_heat: 26.0-27.61
  evaporation_heat: 604-610
  thermal_conductivity: "147"
  debye_temperature: "430.00"
  pauling_negativity_number: "2.20"
  first_ionizing_energy: "868.1"
  oxidation_states: "6, 4, 3, 2, 1, 0, -1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8311; 6s&#178;"
  lattice_structure: FCC
  lattice_constant: "3.840"
  lattice_ca_ratio: n/a
  appearance: "White, brittle metal"
  discovery_date: 1804 (England/France)
  discovered_by: "S.Tenant, A.F.Fourcroy, L.N.Vauquelin, H.V.Collet-Descoltils"
  named_after: "Greek: iris (rainbow)"
  pos_x: 8
  pos_y: 5
- name: Platinum
  symbol: Pt
  atomic_number: 78
  atomic_weigth: "195.08"
  density: 21.09-21.45
  melting_point: 2041.4-2045
  boiling_point: 4098-4100
  atomic_radius: "139"
  covalent_radius: "130"
  ionic_radius: (+4e) 65 (+2e) 80
  atomic_volume: "9.10"
  specific_heat: "0.133"
  fusion_heat: "21.76"
  evaporation_heat: ~470
  thermal_conductivity: "71.6"
  debye_temperature: "230.00"
  pauling_negativity_number: "2.28"
  first_ionizing_energy: "868.1"
  oxidation_states: "4, 2, 0"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#8313; 6s&#185;"
  lattice_structure: FCC
  lattice_constant: "3.920"
  lattice_ca_ratio: n/a
  appearance: "Very heavy, soft, silvery-white metal"
  discovery_date: "1557 (Italy), but Incas use before"
  discovered_by: Julius Scaliger
  named_after: "Spanish: platina (little silver)"
  pos_x: 9
  pos_y: 5
- name: Gold
  symbol: Au
  atomic_number: 79
  atomic_weigth: "196.96657"
  density: "19.3"
  melting_point: 1337.33-1337.58
  boiling_point: 3080-3129
  atomic_radius: 144-146
  covalent_radius: "134"
  ionic_radius: (-3e) 185 (+1e) 137 or (+3e) 85 (+1e) 137
  atomic_volume: "10.2"
  specific_heat: "0.129"
  fusion_heat: "12.68"
  evaporation_heat: ~340
  thermal_conductivity: "318"
  debye_temperature: "170.00"
  pauling_negativity_number: 2.54-2.64
  first_ionizing_energy: "889.3"
  oxidation_states: "-1, 1, 3, 5"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#185;/[Xe] 5p&#8310; 5d&#185;&#8304; 6s&#185;"
  lattice_structure: FCC
  lattice_constant: "4.080"
  lattice_ca_ratio: n/a
  appearance: "Soft, malleable, yellow metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: geolo (yellow); symbol from Latin: aurum (shining dawn)"
  pos_x: 10
  pos_y: 5
- name: Mercury
  symbol: Hg
  atomic_number: 80
  atomic_weigth: "200.59"
  density: 13.546 (@ +20 °C)
  melting_point: 234.28-234.32
  boiling_point: 629.73-629.88
  atomic_radius: "157"
  covalent_radius: "149"
  ionic_radius: (+2e) 110 (+1e) 127
  atomic_volume: "14.8"
  specific_heat: "0.138"
  fusion_heat: "2.295"
  evaporation_heat: "58.5"
  thermal_conductivity: "8.3"
  debye_temperature: "100.00"
  pauling_negativity_number: "2.00"
  first_ionizing_energy: "1006.0"
  oxidation_states: "2, 1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178;"
  lattice_structure: RHL
  lattice_constant: "2.990"
  lattice_ca_ratio: "1.94"
  appearance: "Heavy, silver-white metal that is in its liquid state at"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "The Roman god Mercury; symbol from Latin: hydrargyrum (liquid silver)"
  pos_x: 11
  pos_y: 5
- name: Thallium
  symbol: Tl
  atomic_number: 81
  atomic_weigth: "204.3833"
  density: "11.85"
  melting_point: 576.6-577
  boiling_point: 1730-1746
  atomic_radius: "171"
  covalent_radius: "148"
  ionic_radius: (+3e) 95 (+1e) 147
  atomic_volume: "17.2"
  specific_heat: "0.128"
  fusion_heat: "4.31"
  evaporation_heat: "162.4"
  thermal_conductivity: "46.1"
  debye_temperature: "96.00"
  pauling_negativity_number: "1.62"
  first_ionizing_energy: "588.9"
  oxidation_states: "3, 1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#185;"
  lattice_structure: HEX
  lattice_constant: "3.460"
  lattice_ca_ratio: "1.599"
  appearance: "Soft, gray metal"
  discovery_date: 1861 (England)
  discovered_by: Sir William Crookes
  named_after: "Greek: thallos (green twig), for a bright green line in its spectrum"
  pos_x: 12
  pos_y: 5
- name: Lead
  symbol: Pb
  atomic_number: 82
  atomic_weigth: "207.2"
  density: 11.34-11.35
  melting_point: 600.61-600.65
  boiling_point: 2013-2022
  atomic_radius: "175"
  covalent_radius: "147"
  ionic_radius: (+4e) 84 (+2e) 120
  atomic_volume: "18.3"
  specific_heat: "0.159"
  fusion_heat: "4.77"
  evaporation_heat: "177.8"
  thermal_conductivity: "35.3"
  debye_temperature: "88.00"
  pauling_negativity_number: "1.8"
  first_ionizing_energy: "715.2"
  oxidation_states: "4, 2, 0"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#178;"
  lattice_structure: FCC
  lattice_constant: "4.950"
  lattice_ca_ratio: n/a
  appearance: "Very soft, highly malleable and ductile, blue-white shiny metal"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "Anglo-Saxon: lead; symbol from Latin: plumbum"
  pos_x: 13
  pos_y: 5
- name: Bismuth
  symbol: Bi
  atomic_number: 83
  atomic_weigth: "208.9804"
  density: 9.747-9.79
  melting_point: "544.5"
  boiling_point: 1564-1883
  atomic_radius: "170"
  covalent_radius: "146"
  ionic_radius: (+5e) 74 (+3e) 96
  atomic_volume: "21.3"
  specific_heat: "0.124"
  fusion_heat: 11.00-11.30
  evaporation_heat: "172.0"
  thermal_conductivity: "7.9"
  debye_temperature: "120.00"
  pauling_negativity_number: "2.02"
  first_ionizing_energy: "702.9"
  oxidation_states: "5, 3"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#179;"
  lattice_structure: RHL
  lattice_constant: "4.750"
  lattice_ca_ratio: n/a
  appearance: "Hard, brittle, steel-gray metal with a pinkish tinge"
  discovery_date: n/a (Unknown)
  discovered_by: Known to the ancients
  named_after: "German: bisemutum (white mass); now spelled wismut"
  pos_x: 14
  pos_y: 5
- name: Polonium
  symbol: Po
  atomic_number: 84
  atomic_weigth: "208.9824"
  density: 9.196-9.32
  melting_point: "527"
  boiling_point: "1235"
  atomic_radius: "176"
  covalent_radius: "146"
  ionic_radius: (+6e) 67
  atomic_volume: "22.7"
  specific_heat: "0.125"
  fusion_heat: "10"
  evaporation_heat: "102.9"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: 2.0-2.3
  first_ionizing_energy: "813.1"
  oxidation_states: "-2, 2, 4, 6"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#8308;"
  lattice_structure: CUB
  lattice_constant: "3.350"
  lattice_ca_ratio: n/a
  appearance: Silvery-gray metal
  discovery_date: 1898 (France/Poland)
  discovered_by: Pierre and Marie Curie-Sklodowska
  named_after: "Named for Poland, native country of Marie Curie"
  pos_x: 15
  pos_y: 5
- name: Astatine
  symbol: At
  atomic_number: 85
  atomic_weigth: "209.9871"
  density: (6.4)
  melting_point: 503-575
  boiling_point: 575-610
  atomic_radius: "145"
  covalent_radius: (145)
  ionic_radius: (+7e) 62
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: (195)
  pauling_negativity_number: 2.2-2.5
  first_ionizing_energy: "916.3"
  oxidation_states: "7, 5, 3, 1, -1"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#8309;"
  lattice_structure: FCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Unstable, radioactive halogen"
  discovery_date: 1940 (United States)
  discovered_by: "D.R.Corson, K.R.MacKenzie, E. Segre"
  named_after: "Greek: astatos (unstable)"
  pos_x: 16
  pos_y: 5
- name: Radon
  symbol: Rn
  atomic_number: 86
  atomic_weigth: "222.0176"
  density: 4.4 (@ -62 °C)
  melting_point: "202"
  boiling_point: "211.4"
  atomic_radius: "214"
  covalent_radius: 140-150
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: "0.094"
  fusion_heat: "2.7"
  evaporation_heat: "18.1"
  thermal_conductivity: "0.0036"
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: "1036.5"
  oxidation_states: "2, 4, 6, 8"
  eletronic_configuration: "[Xe] 4f&#185;&#8308; 5d&#185;&#8304; 6s&#178; 6p&#8310;"
  lattice_structure: FCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: Heavy radioactive gas
  discovery_date: 1898 (Germany/England)
  discovered_by: "Fredrich Ernst Dorn, Ernest Rutherford"
  named_after: "Variation of the name of another element, radium"
  pos_x: 17
  pos_y: 5
- name: Francium
  symbol: Fr
  atomic_number: 87
  atomic_weigth: "223.0197"
  density: "1.87"
  melting_point: 291-300
  boiling_point: 913-950
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: (+1e) 180
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: ~2-15
  evaporation_heat: ~65
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "0.7"
  first_ionizing_energy: "380"
  oxidation_states: "1, 2"
  eletronic_configuration: "[Rn] 7s&#185;"
  lattice_structure: BCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1939 (France)
  discovered_by: Marguerite Perey
  named_after: "Named for France, the nation of its discovery"
  pos_x: 0
  pos_y: 6
- name: Radium
  symbol: Ra
  atomic_number: 88
  atomic_weigth: "226.0254"
  density: 5.5 (@ +20 °C)
  melting_point: "973"
  boiling_point: 1413-2010
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: (+2e) 143
  atomic_volume: "45.0"
  specific_heat: "0.120"
  fusion_heat: 8.5-(9.6)
  evaporation_heat: "113"
  thermal_conductivity: (18.6)
  debye_temperature: n/a
  pauling_negativity_number: "0.9"
  first_ionizing_energy: 509.3; 979.0
  oxidation_states: "2"
  eletronic_configuration: "[Rn] 7s&#178;"
  lattice_structure: BCC
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery white, radioactive element"
  discovery_date: 1898 (France/Poland)
  discovered_by: Pierre and Marie Curie-Sklodowska
  named_after: "Latin: radius (beam, ray)"
  pos_x: 1
  pos_y: 6
- name: Actinium
  symbol: Ac
  atomic_number: 89
  atomic_weigth: "227.0278"
  density: "10.07"
  melting_point: "1320"
  boiling_point: "3470"
  atomic_radius: "188"
  covalent_radius: n/a
  ionic_radius: (+3e) 118
  atomic_volume: "22.54"
  specific_heat: n/a
  fusion_heat: (10.5)
  evaporation_heat: (292.9)
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.1"
  first_ionizing_energy: "665.5"
  oxidation_states: "3"
  eletronic_configuration: "[Rn] 6d&#185; 7s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.310"
  lattice_ca_ratio: n/a
  appearance: "Heavy, Silvery-white metal that is very radioactive"
  discovery_date: 1899 (France)
  discovered_by: Andre-Louis Debierne
  named_after: "Greek: akis, aktis, aktinos (beam, ray)"
  pos_x: 3
  pos_y: 8
- name: Thorium
  symbol: Th
  atomic_number: 90
  atomic_weigth: "232.0381"
  density: "11.78"
  melting_point: "2028"
  boiling_point: "5060"
  atomic_radius: "180"
  covalent_radius: "165"
  ionic_radius: (+4e) 102
  atomic_volume: "19.8"
  specific_heat: "0.113"
  fusion_heat: "16.11"
  evaporation_heat: "513.7"
  thermal_conductivity: (54.0)
  debye_temperature: "100.00"
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "670.4"
  oxidation_states: "4"
  eletronic_configuration: "[Rn] 6d&#178; 7s&#178;"
  lattice_structure: FCC
  lattice_constant: "5.080"
  lattice_ca_ratio: n/a
  appearance: "Gray, soft, malleable, ductile, radioactive metal"
  discovery_date: 1828 (Sweden)
  discovered_by: Jons Jacob Berzelius
  named_after: "Named for Thor, Norse god of thunder"
  pos_x: 4
  pos_y: 8
- name: Protactinium
  symbol: Pa
  atomic_number: 91
  atomic_weigth: "231.03588"
  density: "15.37"
  melting_point: "2113"
  boiling_point: "4300"
  atomic_radius: "161"
  covalent_radius: n/a
  ionic_radius: (+5e) 89 (+3e) 113
  atomic_volume: "15.0"
  specific_heat: "0.121"
  fusion_heat: "16.7"
  evaporation_heat: "481.2"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.5"
  first_ionizing_energy: "0.0"
  oxidation_states: "5, 4"
  eletronic_configuration: "[Rn] 5f&#178; 6d&#185; 7s&#178;"
  lattice_structure: TET
  lattice_constant: "3.920"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, radioactive metal"
  discovery_date: 1918 (England/France)
  discovered_by: "Fredrich Soddy, John Cranston, Otto Hahn, Lise Meitner"
  named_after: "Greek: proto and actinium (parent of actinium); it forms actinium when it radioactively decays"
  pos_x: 5
  pos_y: 8
- name: Uranium
  symbol: U
  atomic_number: 92
  atomic_weigth: "238.0289"
  density: "19.05"
  melting_point: "1405.5"
  boiling_point: "4018"
  atomic_radius: "138"
  covalent_radius: "142"
  ionic_radius: (+6e) 80 (+4e) 97
  atomic_volume: "12.5"
  specific_heat: "0.115"
  fusion_heat: "12.6"
  evaporation_heat: "417"
  thermal_conductivity: "27.5"
  debye_temperature: n/a
  pauling_negativity_number: "1.38"
  first_ionizing_energy: "686.4"
  oxidation_states: "6, 5, 4, 3"
  eletronic_configuration: "[Rn] 5f&#179; 6d&#185; 7s&#178;"
  lattice_structure: ORC
  lattice_constant: "2.850"
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, dense, ductile and malleable, radioactive metal"
  discovery_date: 1789 (Germany)
  discovered_by: Martin Klaproth
  named_after: Named for the planet Uranus
  pos_x: 6
  pos_y: 8
- name: Neptunium
  symbol: Np
  atomic_number: 93
  atomic_weigth: "237.048"
  density: "20.25"
  melting_point: "913"
  boiling_point: "4175"
  atomic_radius: "130"
  covalent_radius: n/a
  ionic_radius: (+4e) 95 (+3e) 110
  atomic_volume: "21.1"
  specific_heat: n/a
  fusion_heat: (9.6)
  evaporation_heat: "336"
  thermal_conductivity: (6.3)
  debye_temperature: n/a
  pauling_negativity_number: "1.36"
  first_ionizing_energy: "0.0"
  oxidation_states: "7, 6, 5, 4, 3"
  eletronic_configuration: "[Rn] 5f&#8308; 6d&#185; 7s&#178;"
  lattice_structure: ORC
  lattice_constant: "4.720"
  lattice_ca_ratio: n/a
  appearance: Silvery metal
  discovery_date: 1940 (United States)
  discovered_by: "E.M. McMillan, P.H. Abelson"
  named_after: Named for the planet Neptune
  pos_x: 7
  pos_y: 8
- name: Plutonium
  symbol: Pu
  atomic_number: 94
  atomic_weigth: "244.0642"
  density: "19.84"
  melting_point: 641-912.85
  boiling_point: 3505-3508.15
  atomic_radius: 151-162
  covalent_radius: n/a
  ionic_radius: 71-100 or (+4e) 93 (+3e) 108
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: "2.8"
  evaporation_heat: "343.5"
  thermal_conductivity: (6.7)
  debye_temperature: n/a
  pauling_negativity_number: "1.28"
  first_ionizing_energy: 491.9-584.7
  oxidation_states: "2, 3, 4, 5, 6, 7"
  eletronic_configuration: "[Rn] 5f&#8310; 7s&#178;"
  lattice_structure: MCL
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, radioactive metal"
  discovery_date: 1940 (United States)
  discovered_by: "G.T.Seaborg, J.W.Kennedy, E.M.McMillan, A.C.Wohl"
  named_after: Named for the planet Pluto
  pos_x: 8
  pos_y: 8
- name: Americium
  symbol: Am
  atomic_number: 95
  atomic_weigth: "243.0614"
  density: "13.67"
  melting_point: 1267-1448
  boiling_point: 2880-2880.15
  atomic_radius: "173"
  covalent_radius: n/a
  ionic_radius: (+4e) 92 (+3e) 107
  atomic_volume: "20.8"
  specific_heat: n/a
  fusion_heat: (10.0)
  evaporation_heat: "238.5"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "0.0"
  oxidation_states: "6, 5, 4, 3"
  eletronic_configuration: "[Rn] 5f&#8311; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery-white, radioactive metal"
  discovery_date: 1944 (United States)
  discovered_by: "G.T.Seaborg, R.A.James, L.O.Morgan, A.Ghiorso"
  named_after: "Named for the American continent, by analogy with europium"
  pos_x: 9
  pos_y: 8
- name: Curium
  symbol: Cm
  atomic_number: 96
  atomic_weigth: "247.0703"
  density: "13.51"
  melting_point: 1340-1613
  boiling_point: "3383"
  atomic_radius: "299"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: "18.28"
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "581"
  oxidation_states: "4, 3"
  eletronic_configuration: "[Rn] 5f&#8311; 6d&#185; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Silvery, malleable, synthetic radioactive metal"
  discovery_date: 1944 (United States)
  discovered_by: "G.T.Seaborg, R.A.James, A.Ghiorso"
  named_after: Named in honor of Pierre and Marie Curie
  pos_x: 10
  pos_y: 8
- name: Berkelium
  symbol: Bk
  atomic_number: 97
  atomic_weigth: "247.0703"
  density: "13.25"
  melting_point: "1259"
  boiling_point: "2900"
  atomic_radius: "297"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: (600)
  oxidation_states: "4, 3"
  eletronic_configuration: "[Rn] 5f&#8313; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: Radioactive synthetic metal
  discovery_date: 1949 (United States)
  discovered_by: "G.T.Seaborg, S.G.Tompson, A.Ghiorso"
  named_after: "Named after Berkeley, California the city of its discovery"
  pos_x: 11
  pos_y: 8
- name: Californium
  symbol: Cf
  atomic_number: 98
  atomic_weigth: "251.0796"
  density: "15.1"
  melting_point: 900-1173.15
  boiling_point: (1743)
  atomic_radius: "295"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: (610)
  oxidation_states: "2, 3, 4"
  eletronic_configuration: "[Rn] 5f&#185;&#8304; 7s&#178;"
  lattice_structure: HEX
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: Powerful neutron emitter
  discovery_date: 1950 (United States)
  discovered_by: "G.T.Seaborg, S.G.Tompson, A.Ghiorso, K.Street Jr."
  named_after: Named after the US-state and University of California
  pos_x: 12
  pos_y: 8
- name: Einsteinium
  symbol: Es
  atomic_number: 99
  atomic_weigth: "252.083"
  density: "13.5"
  melting_point: "860"
  boiling_point: "1130"
  atomic_radius: "292"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "619"
  oxidation_states: "2, 3, 4"
  eletronic_configuration: "[Rn] 5f&#185;&#185; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1952 (United States)
  discovered_by: "Argonne, Los Alamos, University of California"
  named_after: Named in honor of the scientist Albert Einstein
  pos_x: 13
  pos_y: 8
- name: Fermium
  symbol: Fm
  atomic_number: 100
  atomic_weigth: "257.0951"
  density: n/a
  melting_point: "1800"
  boiling_point: n/a
  atomic_radius: "290"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: 627-630
  oxidation_states: "3"
  eletronic_configuration: "[Rn] 5f&#185;&#178; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1952 (United States)
  discovered_by: "Argonne, Los Alamos, University of California"
  named_after: Named in honor of the scientist Enrico Fermi
  pos_x: 14
  pos_y: 8
- name: Mendelevium
  symbol: Md
  atomic_number: 101
  atomic_weigth: "258.1"
  density: n/a
  melting_point: "1100"
  boiling_point: n/a
  atomic_radius: "287"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "635"
  oxidation_states: "1, 2, 3"
  eletronic_configuration: "[Rn] 5f&#185;&#179; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1955 (United States)
  discovered_by: "G.T.Seaborg, S.G.Tompson, A.Ghiorso, K.Street Jr."
  named_after: "Named in honor of the scientist Dmitry Ivanovich Mendeleev, who devised the periodic table"
  pos_x: 15
  pos_y: 8
- name: Nobelium
  symbol: "No"
  atomic_number: 102
  atomic_weigth: "259.1009"
  density: n/a
  melting_point: "1100"
  boiling_point: n/a
  atomic_radius: "285"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "640"
  oxidation_states: "3, 2"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1963 (USSR)
  discovered_by: Dubna
  named_after: "Named in honor of Alfred Nobel, who invented dynamite and founded Nobel prize"
  pos_x: 16
  pos_y: 8
- name: Lawrencium
  symbol: Lr
  atomic_number: 103
  atomic_weigth: "262.11"
  density: n/a
  melting_point: "1900"
  boiling_point: n/a
  atomic_radius: "282"
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: "1.3"
  first_ionizing_energy: "470"
  oxidation_states: "3"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1961 (USSR/United States)
  discovered_by: Soviet Nuclear Research/University of California at Berkeley
  named_after: "Named in honor of Ernest Orlando Lawrence, inventor of the cyclotron"
  pos_x: 17
  pos_y: 8
- name: Rutherfordium
  symbol: Rf
  atomic_number: 104
  atomic_weigth: "261"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "4"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#178; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1964 (USSR)
  discovered_by: Dubna
  named_after: Named in honor of Ernest Rutherford
  pos_x: 3
  pos_y: 6
- name: Dubnium
  symbol: Db
  atomic_number: 105
  atomic_weigth: "268"
  density: "26.1"
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: "139"
  covalent_radius: "66"
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "2, 3, 4, 5"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#179; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1970 (USSR/United States)
  discovered_by: Soviet Nuclear Research/University of California at Berkeley
  named_after: Named after the science-town Dubna in Russia
  pos_x: 4
  pos_y: 6
- name: Seaborgium
  symbol: Sg
  atomic_number: 106
  atomic_weigth: "[271]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8308; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1974 (USSR/United States)
  discovered_by: Soviet Nuclear Research/University of California at Berkeley
  named_after: "Named in honor of Glenn Theodore Seaborg, American physical chemist known for research on transuranium elements"
  pos_x: 5
  pos_y: 6
- name: Bohrium
  symbol: Bh
  atomic_number: 107
  atomic_weigth: "[267]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: (128)
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: (660)
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8309; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: "Radioactive, synthetic metal"
  discovery_date: 1976 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of Niels Bohr
  pos_x: 6
  pos_y: 6
- name: Hassium
  symbol: Hs
  atomic_number: 108
  atomic_weigth: "[269]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8310; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1984 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: "Named in honor of Hessen county, Germany (latin: Hassia)"
  pos_x: 7
  pos_y: 6
- name: Meitnerium
  symbol: Mt
  atomic_number: 109
  atomic_weigth: "[278]"
  density: "37.4"
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "9, 8, 6, 4, 3, 1"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8311; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1982 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of Lise Meitner
  pos_x: 8
  pos_y: 6
- name: Darmstadtium
  symbol: Ds
  atomic_number: 110
  atomic_weigth: "[281]"
  density: (34.8)
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#8313; 7s&#185;/[Rn] 5f&#185;&#8308; 6d&#8312; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1994 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: "Named after Darmstadt, Germany, the city of its discovery"
  pos_x: 9
  pos_y: 6
- name: Roentgenium
  symbol: Rg
  atomic_number: 111
  atomic_weigth: "[281]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#185;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1994 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of physicist Wilhelm Conrad Roentgen
  pos_x: 10
  pos_y: 6
- name: Copernicium
  symbol: Cn
  atomic_number: 112
  atomic_weigth: "285"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1996 (Germany)
  discovered_by: Heavy Ion Research Laboratory (HIRL)
  named_after: Named in honor of Nicolaus Copernicus
  pos_x: 11
  pos_y: 6
- name: Ununtrium
  symbol: Uut
  atomic_number: 113
  atomic_weigth: (286)
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#185;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2004 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) trium (three)
  pos_x: 12
  pos_y: 6
- name: Flerovium
  symbol: Fl
  atomic_number: 114
  atomic_weigth: "[289]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "4, 2"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#178;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 1998 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Named in honor of Soviet physicist Flerov
  pos_x: 13
  pos_y: 6
- name: Ununpentium
  symbol: Uup
  atomic_number: 115
  atomic_weigth: "288"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#179;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2003 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) pentium (five)
  pos_x: 14
  pos_y: 6
- name: Livermorium
  symbol: Lv
  atomic_number: 116
  atomic_weigth: "[193]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: "-2, 2, 4, 6"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#8308;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2000 (Russia/United States)
  discovered_by: "Dubna, Dimitrovgrad, Lesnoy/Livermore"
  named_after: Named in honor of Livermore National Laboratory (USA)
  pos_x: 15
  pos_y: 6
- name: Ununseptium
  symbol: Uus
  atomic_number: 117
  atomic_weigth: "[294]"
  density: n/a
  melting_point: n/a
  boiling_point: n/a
  atomic_radius: n/a
  covalent_radius: n/a
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: n/a
  evaporation_heat: n/a
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: n/a
  oxidation_states: n/a
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#8309;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2009 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) septium (seven)
  pos_x: 16
  pos_y: 6
- name: Ununoctium
  symbol: Uuo
  atomic_number: 118
  atomic_weigth: "[294]"
  density: "13.65"
  melting_point: n/a
  boiling_point: "350&#177;30"
  atomic_radius: "152"
  covalent_radius: "230"
  ionic_radius: n/a
  atomic_volume: n/a
  specific_heat: n/a
  fusion_heat: "23.5"
  evaporation_heat: "19.4"
  thermal_conductivity: n/a
  debye_temperature: n/a
  pauling_negativity_number: n/a
  first_ionizing_energy: "975&#177;155"
  oxidation_states: "-1, 0, 1, 2, 4, 6, 8"
  eletronic_configuration: "[Rn] 5f&#185;&#8308; 6d&#185;&#8304; 7s&#178; 7p&#8310;"
  lattice_structure: n/a
  lattice_constant: n/a
  lattice_ca_ratio: n/a
  appearance: n/a
  discovery_date: 2006 (Russia/United States)
  discovered_by: Dubna/Livermore
  named_after: Un (one) un (one) octium (eight)
  pos_x: 17
  pos_y: 6"#;
	s
}
