use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn test_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 217], OperandSize::Word)
}

fn test_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25014, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 155, 182, 97], OperandSize::Word)
}

fn test_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 211], OperandSize::Dword)
}

fn test_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(EDI, 788123089, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 159, 209, 205, 249, 46], OperandSize::Dword)
}

fn test_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 218], OperandSize::Qword)
}

fn test_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1904772741, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 12, 205, 133, 134, 136, 113], OperandSize::Qword)
}

fn test_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 217], OperandSize::Qword)
}

fn test_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1097733898, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 140, 184, 10, 23, 110, 65], OperandSize::Qword)
}

fn test_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 243], OperandSize::Word)
}

fn test_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(DI, 168, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 157, 168, 0], OperandSize::Word)
}

fn test_11() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 252], OperandSize::Dword)
}

fn test_12() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(EDI, 1212948082, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 183, 114, 30, 76, 72], OperandSize::Dword)
}

fn test_13() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 251], OperandSize::Qword)
}

fn test_14() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RCX, 1200971727, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 185, 207, 95, 149, 71], OperandSize::Qword)
}

fn test_15() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 254], OperandSize::Word)
}

fn test_16() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27630, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 153, 238, 107], OperandSize::Word)
}

fn test_17() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 251], OperandSize::Dword)
}

fn test_18() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 672137195, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 180, 151, 235, 255, 15, 40], OperandSize::Dword)
}

fn test_19() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 243], OperandSize::Qword)
}

fn test_20() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1993248817, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 12, 213, 49, 144, 206, 118], OperandSize::Qword)
}

fn test_21() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 226], OperandSize::Qword)
}

fn test_22() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 60, 192], OperandSize::Qword)
}

fn test_23() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 56], OperandSize::Word)
}

fn test_24() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 43], OperandSize::Dword)
}

fn test_25() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 40], OperandSize::Qword)
}

fn test_26() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(21874)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 114, 85], OperandSize::Word)
}

fn test_27() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(5824)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 192, 22], OperandSize::Dword)
}

fn test_28() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(3163)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 91, 12], OperandSize::Qword)
}

fn test_29() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(357428464)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 240, 236, 77, 21], OperandSize::Word)
}

fn test_30() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(748186032)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 176, 105, 152, 44], OperandSize::Dword)
}

fn test_31() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2009464192)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 128, 253, 197, 119], OperandSize::Qword)
}

fn test_32() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RAX)), operand2: Some(Literal32(775311830)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 169, 214, 81, 54, 46], OperandSize::Qword)
}

fn test_33() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 115], OperandSize::Word)
}

fn test_34() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 5897, Some(OperandSize::Byte), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 129, 9, 23, 75], OperandSize::Word)
}

fn test_35() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 195, 64], OperandSize::Dword)
}

fn test_36() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(ECX, 1242563917, Some(OperandSize::Byte), None)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 129, 77, 5, 16, 74, 15], OperandSize::Dword)
}

fn test_37() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 70], OperandSize::Qword)
}

fn test_38() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 478984636, Some(OperandSize::Byte), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 132, 194, 188, 185, 140, 28, 29], OperandSize::Qword)
}

fn test_39() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 70], OperandSize::Qword)
}

fn test_40() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 4, 67, 79], OperandSize::Qword)
}

fn test_41() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SP)), operand2: Some(Literal16(17072)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 196, 176, 66], OperandSize::Word)
}

fn test_42() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(DI, 12422, Some(OperandSize::Word), None)), operand2: Some(Literal16(5403)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 133, 134, 48, 27, 21], OperandSize::Word)
}

fn test_43() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DI)), operand2: Some(Literal16(17133)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 199, 237, 66], OperandSize::Dword)
}

fn test_44() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1785068344, Some(OperandSize::Word), None)), operand2: Some(Literal16(21355)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 4, 189, 56, 251, 101, 106, 107, 83], OperandSize::Dword)
}

fn test_45() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SP)), operand2: Some(Literal16(26703)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 196, 79, 104], OperandSize::Qword)
}

fn test_46() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1081110875, Some(OperandSize::Word), None)), operand2: Some(Literal16(31843)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 4, 125, 91, 113, 112, 64, 99, 124], OperandSize::Qword)
}

fn test_47() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBP)), operand2: Some(Literal32(544886333)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 197, 61, 78, 122, 32], OperandSize::Word)
}

fn test_48() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1087869544)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 5, 104, 146, 215, 64], OperandSize::Word)
}

fn test_49() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBP)), operand2: Some(Literal32(873292571)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 197, 27, 99, 13, 52], OperandSize::Dword)
}

fn test_50() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1850233233, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1431771809)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 132, 87, 145, 81, 72, 110, 161, 26, 87, 85], OperandSize::Dword)
}

fn test_51() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Literal32(171288612)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 194, 36, 168, 53, 10], OperandSize::Qword)
}

fn test_52() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1949377513, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1528323950)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 132, 128, 233, 35, 49, 116, 110, 95, 24, 91], OperandSize::Qword)
}

fn test_53() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RSI)), operand2: Some(Literal32(566188871)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 198, 71, 91, 191, 33], OperandSize::Qword)
}

fn test_54() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal32(290555717)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 4, 202, 69, 135, 81, 17], OperandSize::Qword)
}

