### **The Official Wall Blueprint (Codes 0-15)**

Here is what each `secret_code` means, and what you must name your file.

#### **Group 0: The Lonely Pillar (No Connections)**

*   **Code 0:** `wall_0.png`
    *   **Connections:** None.
    *   **Description:** A single, isolated wall tile. A sad, lonely post in the middle of nowhere. Maybe has cracks on all sides.

#### **Group 1: The End Caps (1 Connection)**

*   **Code 1:** `wall_1.png`
    *   **Connections:** UP only.
    *   **Description:** The bottom end of a vertical wall. Is a floor piece.
*   **Code 2:** `wall_2.png`
    *   **Connections:** DOWN only.
    *   **Description:** The top end of a vertical wall. Is a ceiling piece.
*   **Code 4:** `wall_4.png`
    *   **Connections:** LEFT only.
    *   **Description:** The right end of a horizontal wall.
*   **Code 8:** `wall_8.png`
    *   **Connections:** RIGHT only.
    *   **Description:** The left end of a horizontal wall.

#### **Group 2: Straights & Corners (2 Connections)**

*   **Code 3:** `wall_3.png` (`1+2`)
    *   **Connections:** UP and DOWN.
    *   **Description:** A glorious vertical pillar. A straight up-and-down wall piece.
*   **Code 12:** `wall_12.png` (`4+8`)
    *   **Connections:** LEFT and RIGHT.
    *   **Description:** A sturdy horizontal beam. A straight left-and-right wall piece.
*   **Code 5:** `wall_5.png` (`1+4`)
    *   **Connections:** UP and LEFT.
    *   **Description:** A bottom-right corner piece. Shaped like an L.
*   **Code 6:** `wall_6.png` (`2+4`)
    *   **Connections:** DOWN and LEFT.
    *   **Description:** A top-right corner piece.
*   **Code 9:** `wall_9.png` (`1+8`)
    *   **Connections:** UP and RIGHT.
    *   **Description:** A bottom-left corner piece.
*   **Code 10:** `wall_10.png` (`2+8`)
    *   **Connections:** DOWN and RIGHT.
    *   **Description:** A top-left corner piece.

#### **Group 3: The T-Junctions (3 Connections)**

*   **Code 7:** `wall_7.png` (`1+2+4`)
    *   **Connections:** UP, DOWN, and LEFT.
    *   **Description:** A T-junction that opens to the RIGHT.
*   **Code 11:** `wall_11.png` (`1+2+8`)
    *   **Connections:** UP, DOWN, and RIGHT.
    *   **Description:** A T-junction that opens to the LEFT.
*   **Code 13:** `wall_13.png` (`1+4+8`)
    *   **Connections:** UP, LEFT, and RIGHT.
    *   **Description:** A T-junction that opens DOWN. Like a hanging hook from ceiling.
*   **Code 14:** `wall_14.png` (`2+4+8`)
    *   **Connections:** DOWN, LEFT, and RIGHT.
    *   **Description:** A T-junction that opens UP. Like a post sticking up from floor.

#### **Group 4: The Solid Block (4 Connections)**

*   **Code 15:** `wall_15.png` (`1+2+4+8`)
    *   **Connections:** All four sides.
    *   **Description:** This piece is surrounded by comrades! It is a solid block of wall. Maybe no borders, just solid texture.
