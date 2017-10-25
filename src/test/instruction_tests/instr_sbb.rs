use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sbb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Word)
}

#[test]
fn sbb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 9882, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 137, 154, 38], OperandSize::Word)
}

#[test]
fn sbb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 211], OperandSize::Dword)
}

#[test]
fn sbb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 15], OperandSize::Dword)
}

#[test]
fn sbb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Qword)
}

#[test]
fn sbb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1810024785, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 20, 117, 81, 201, 226, 107], OperandSize::Qword)
}

#[test]
fn sbb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Qword)
}

#[test]
fn sbb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1936800840, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 148, 241, 72, 60, 113, 115], OperandSize::Qword)
}

#[test]
fn sbb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 236], OperandSize::Word)
}

#[test]
fn sbb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 170, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 176, 170, 0], OperandSize::Word)
}

#[test]
fn sbb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 226], OperandSize::Dword)
}

#[test]
fn sbb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 36, 184], OperandSize::Dword)
}

#[test]
fn sbb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 242], OperandSize::Qword)
}

#[test]
fn sbb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 36, 115], OperandSize::Qword)
}

#[test]
fn sbb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 209], OperandSize::Word)
}

#[test]
fn sbb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 46, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 107, 46], OperandSize::Word)
}

#[test]
fn sbb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 221], OperandSize::Dword)
}

#[test]
fn sbb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 18], OperandSize::Dword)
}

#[test]
fn sbb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 210], OperandSize::Qword)
}

#[test]
fn sbb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RAX, 2111699336, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 160, 136, 249, 221, 125], OperandSize::Qword)
}

#[test]
fn sbb_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 205], OperandSize::Qword)
}

#[test]
fn sbb_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 296650370, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 172, 138, 130, 134, 174, 17], OperandSize::Qword)
}

#[test]
fn sbb_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Word)
}

#[test]
fn sbb_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(SI, 24637, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 148, 61, 96], OperandSize::Word)
}

#[test]
fn sbb_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Dword)
}

#[test]
fn sbb_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 12, 186], OperandSize::Dword)
}

#[test]
fn sbb_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Qword)
}

#[test]
fn sbb_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 95180936, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 20, 93, 136, 88, 172, 5], OperandSize::Qword)
}

#[test]
fn sbb_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 219], OperandSize::Qword)
}

#[test]
fn sbb_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 185717833, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 12, 133, 73, 212, 17, 11], OperandSize::Qword)
}

#[test]
fn sbb_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 237], OperandSize::Word)
}

#[test]
fn sbb_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 58], OperandSize::Word)
}

#[test]
fn sbb_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 218], OperandSize::Dword)
}

#[test]
fn sbb_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1649212317, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 52, 69, 157, 251, 76, 98], OperandSize::Dword)
}

#[test]
fn sbb_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 213], OperandSize::Qword)
}

#[test]
fn sbb_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 410420428, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 188, 79, 204, 132, 118, 24], OperandSize::Qword)
}

#[test]
fn sbb_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 207], OperandSize::Word)
}

#[test]
fn sbb_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 97, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 107, 97], OperandSize::Word)
}

#[test]
fn sbb_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 245], OperandSize::Dword)
}

#[test]
fn sbb_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ECX, 1449846629, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 169, 101, 231, 106, 86], OperandSize::Dword)
}

#[test]
fn sbb_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 213], OperandSize::Qword)
}

#[test]
fn sbb_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 677704028, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 140, 118, 92, 241, 100, 40], OperandSize::Qword)
}

#[test]
fn sbb_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 204], OperandSize::Qword)
}

#[test]
fn sbb_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RAX, 2056734713, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 27, 160, 249, 71, 151, 122], OperandSize::Qword)
}

#[test]
fn sbb_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 9], OperandSize::Word)
}

#[test]
fn sbb_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 5], OperandSize::Dword)
}

#[test]
fn sbb_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 95], OperandSize::Qword)
}

#[test]
fn sbb_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(16968)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 72, 66], OperandSize::Word)
}

#[test]
fn sbb_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(1405)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 125, 5], OperandSize::Dword)
}

#[test]
fn sbb_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(4041)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 201, 15], OperandSize::Qword)
}

#[test]
fn sbb_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1079280567)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 183, 131, 84, 64], OperandSize::Word)
}

#[test]
fn sbb_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1230118036)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 148, 28, 82, 73], OperandSize::Dword)
}

#[test]
fn sbb_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(572246404)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 132, 201, 27, 34], OperandSize::Qword)
}

#[test]
fn sbb_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1958358673)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 29, 145, 46, 186, 116], OperandSize::Qword)
}

#[test]
fn sbb_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 2], OperandSize::Word)
}

#[test]
fn sbb_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 25, 76], OperandSize::Word)
}

#[test]
fn sbb_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 69], OperandSize::Dword)
}

#[test]
fn sbb_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 144, 53], OperandSize::Dword)
}

#[test]
fn sbb_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 73], OperandSize::Qword)
}

#[test]
fn sbb_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1495782938, Some(OperandSize::Byte), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 156, 159, 26, 214, 39, 89, 92], OperandSize::Qword)
}

#[test]
fn sbb_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 218, 86], OperandSize::Qword)
}

#[test]
fn sbb_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 72, 50], OperandSize::Qword)
}

#[test]
fn sbb_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal16(7553)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 129, 29], OperandSize::Word)
}

#[test]
fn sbb_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(SI, 166, Some(OperandSize::Word), None)), operand2: Some(Literal16(4607)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 156, 166, 0, 255, 17], OperandSize::Word)
}

#[test]
fn sbb_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Literal16(23742)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 220, 190, 92], OperandSize::Dword)
}

#[test]
fn sbb_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal16(22768)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 27, 240, 88], OperandSize::Dword)
}

#[test]
fn sbb_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Literal16(14816)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 217, 224, 57], OperandSize::Qword)
}

#[test]
fn sbb_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(6369)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 28, 131, 225, 24], OperandSize::Qword)
}

#[test]
fn sbb_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(504148020)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 219, 52, 176, 12, 30], OperandSize::Word)
}

#[test]
fn sbb_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 12281, Some(OperandSize::Dword), None)), operand2: Some(Literal32(504976353)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 153, 249, 47, 225, 83, 25, 30], OperandSize::Word)
}

#[test]
fn sbb_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(843755803)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 27, 177, 74, 50], OperandSize::Dword)
}

#[test]
fn sbb_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1548743459)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 28, 186, 35, 243, 79, 92], OperandSize::Dword)
}

#[test]
fn sbb_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1250478512)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 176, 201, 136, 74], OperandSize::Qword)
}

#[test]
fn sbb_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1231456599)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 24, 87, 137, 102, 73], OperandSize::Qword)
}

#[test]
fn sbb_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBX)), operand2: Some(Literal32(1836102790)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 219, 134, 180, 112, 109], OperandSize::Qword)
}

#[test]
fn sbb_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1962171846, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1976101947)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 156, 81, 198, 93, 244, 116, 59, 236, 200, 117], OperandSize::Qword)
}

#[test]
fn sbb_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 217, 54], OperandSize::Word)
}

#[test]
fn sbb_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(SI, 9, Some(OperandSize::Word), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 92, 9, 104], OperandSize::Word)
}

#[test]
fn sbb_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 218, 55], OperandSize::Dword)
}

#[test]
fn sbb_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EBX, Two, 414848723, Some(OperandSize::Word), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 28, 93, 211, 22, 186, 24, 18], OperandSize::Dword)
}

#[test]
fn sbb_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 218, 89], OperandSize::Qword)
}

#[test]
fn sbb_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 28, 158, 7], OperandSize::Qword)
}

#[test]
fn sbb_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 221, 126], OperandSize::Word)
}

#[test]
fn sbb_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 22623, Some(OperandSize::Dword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 152, 95, 88, 25], OperandSize::Word)
}

#[test]
fn sbb_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 219, 73], OperandSize::Dword)
}

#[test]
fn sbb_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1163922586, Some(OperandSize::Dword), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 28, 133, 154, 12, 96, 69, 67], OperandSize::Dword)
}

#[test]
fn sbb_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 217, 102], OperandSize::Qword)
}

#[test]
fn sbb_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RSI, 1100010255, Some(OperandSize::Dword), None)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 158, 15, 211, 144, 65, 121], OperandSize::Qword)
}

#[test]
fn sbb_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSP)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 220, 53], OperandSize::Qword)
}

#[test]
fn sbb_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 28, 154, 70], OperandSize::Qword)
}

