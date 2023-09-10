        MOV      r0, #10    
        MOV      r1, #3
        ADD      r0, r0, r1    
        MOV      r0, #0x18      
        LDR      r1, =0x20026   
        SVC      #0x123456    
      