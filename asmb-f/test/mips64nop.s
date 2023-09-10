    lw     $t0, 0($gp)   
    lw     $t1, 4($gp)  
    slt    $t1, $t0, $t1  
    beq    $t1, $zero, skip  
    sll    $t0, $t0, 2      
    add    $t0, $t0, $gp      
    sw     $zero, 28($t0)    