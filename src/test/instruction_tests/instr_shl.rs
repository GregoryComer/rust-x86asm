use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 89], OperandSize::Word)
}

#[test]
fn shl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 795, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 162, 27, 3, 50], OperandSize::Word)
}

#[test]
fn shl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 15], OperandSize::Dword)
}

#[test]
fn shl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1182669490, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 117, 178, 26, 126, 70, 71], OperandSize::Dword)
}

#[test]
fn shl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 47], OperandSize::Qword)
}

#[test]
fn shl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 152, 60], OperandSize::Qword)
}

#[test]
fn shl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 66], OperandSize::Qword)
}

#[test]
fn shl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RAX, Four, 957650710, Some(OperandSize::Byte), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 133, 22, 151, 20, 57, 7], OperandSize::Qword)
}

#[test]
fn shl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 90], OperandSize::Word)
}

#[test]
fn shl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(SI, 2179, Some(OperandSize::Word), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 164, 131, 8, 81], OperandSize::Word)
}

#[test]
fn shl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 225, 4], OperandSize::Dword)
}

#[test]
fn shl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(EDI, 1457522455, Some(OperandSize::Word), None)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 167, 23, 7, 224, 86, 97], OperandSize::Dword)
}

#[test]
fn shl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DI)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 231, 101], OperandSize::Qword)
}

#[test]
fn shl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 34, 12], OperandSize::Qword)
}

#[test]
fn shl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 225, 90], OperandSize::Word)
}

#[test]
fn shl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(SI, 69, Some(OperandSize::Dword), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 100, 69, 19], OperandSize::Word)
}

#[test]
fn shl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 58], OperandSize::Dword)
}

#[test]
fn shl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(EDX, Two, 1795009445, Some(OperandSize::Dword), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 36, 85, 165, 171, 253, 106, 103], OperandSize::Dword)
}

#[test]
fn shl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 230, 90], OperandSize::Qword)
}

#[test]
fn shl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 958355281, Some(OperandSize::Dword), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 36, 189, 81, 87, 31, 57, 70], OperandSize::Qword)
}

#[test]
fn shl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RBX)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 227, 119], OperandSize::Qword)
}

#[test]
fn shl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RSI, Four, 95692004, Some(OperandSize::Qword), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 36, 181, 228, 36, 180, 5, 112], OperandSize::Qword)
}

#[test]
fn shl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Word)
}

#[test]
fn shl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 33], OperandSize::Word)
}

#[test]
fn shl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Dword)
}

#[test]
fn shl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1774222657, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 93, 65, 125, 192, 105], OperandSize::Dword)
}

#[test]
fn shl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Qword)
}

#[test]
fn shl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RDX, Four, 186088041, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 149, 105, 122, 23, 11], OperandSize::Qword)
}

#[test]
fn shl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Qword)
}

#[test]
fn shl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 468765628, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 164, 179, 188, 203, 240, 27], OperandSize::Qword)
}

#[test]
fn shl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 227], OperandSize::Word)
}

#[test]
fn shl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 37], OperandSize::Word)
}

#[test]
fn shl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Dword)
}

#[test]
fn shl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(ECX, 541367314, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 161, 18, 156, 68, 32], OperandSize::Dword)
}

#[test]
fn shl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 227], OperandSize::Qword)
}

#[test]
fn shl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RBX, 2093133738, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 163, 170, 175, 194, 124], OperandSize::Qword)
}

#[test]
fn shl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 230], OperandSize::Word)
}

#[test]
fn shl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 34], OperandSize::Word)
}

#[test]
fn shl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Dword)
}

#[test]
fn shl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(ECX, 1273238149, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 161, 133, 18, 228, 75], OperandSize::Dword)
}

#[test]
fn shl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Qword)
}

#[test]
fn shl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1074139039, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 164, 122, 159, 15, 6, 64], OperandSize::Qword)
}

#[test]
fn shl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 225], OperandSize::Qword)
}

#[test]
fn shl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1602688593, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 36, 85, 81, 22, 135, 95], OperandSize::Qword)
}

#[test]
fn shl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Word)
}

#[test]
fn shl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 106, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 97, 106], OperandSize::Word)
}

#[test]
fn shl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Dword)
}

#[test]
fn shl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 87], OperandSize::Dword)
}

#[test]
fn shl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Qword)
}

#[test]
fn shl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RSI, Four, 256423839, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 181, 159, 183, 72, 15], OperandSize::Qword)
}

#[test]
fn shl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Qword)
}

#[test]
fn shl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RCX, Four, 22004299, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 141, 75, 194, 79, 1], OperandSize::Qword)
}

#[test]
fn shl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 230], OperandSize::Word)
}

#[test]
fn shl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(DI, 6580, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 165, 180, 25], OperandSize::Word)
}

#[test]
fn shl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 229], OperandSize::Dword)
}

#[test]
fn shl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 433979368, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 164, 127, 232, 255, 221, 25], OperandSize::Dword)
}

#[test]
fn shl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 230], OperandSize::Qword)
}

#[test]
fn shl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 38], OperandSize::Qword)
}

#[test]
fn shl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 228], OperandSize::Word)
}

#[test]
fn shl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 49, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 99, 49], OperandSize::Word)
}

#[test]
fn shl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 225], OperandSize::Dword)
}

#[test]
fn shl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36, 246], OperandSize::Dword)
}

#[test]
fn shl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 231], OperandSize::Qword)
}

#[test]
fn shl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36, 121], OperandSize::Qword)
}

#[test]
fn shl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 227], OperandSize::Qword)
}

#[test]
fn shl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 39], OperandSize::Qword)
}

