use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ror_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 19], OperandSize::Word)
}

#[test]
fn ror_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BX, 8885, Some(OperandSize::Byte), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 143, 181, 34, 76], OperandSize::Word)
}

#[test]
fn ror_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 201, 107], OperandSize::Dword)
}

#[test]
fn ror_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1804110730, Some(OperandSize::Byte), None)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 140, 241, 138, 139, 136, 107, 54], OperandSize::Dword)
}

#[test]
fn ror_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 201, 32], OperandSize::Qword)
}

#[test]
fn ror_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 9, 114], OperandSize::Qword)
}

#[test]
fn ror_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 41], OperandSize::Qword)
}

#[test]
fn ror_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 75, 92], OperandSize::Qword)
}

#[test]
fn ror_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 206, 20], OperandSize::Word)
}

#[test]
fn ror_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(SI, 6030, Some(OperandSize::Word), None)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 140, 142, 23, 8], OperandSize::Word)
}

#[test]
fn ror_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BX)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 203, 61], OperandSize::Dword)
}

#[test]
fn ror_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 12, 94, 118], OperandSize::Dword)
}

#[test]
fn ror_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SP)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 204, 37], OperandSize::Qword)
}

#[test]
fn ror_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 10, 43], OperandSize::Qword)
}

#[test]
fn ror_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 201, 34], OperandSize::Word)
}

#[test]
fn ror_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BX, 24312, Some(OperandSize::Dword), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 143, 248, 94, 85], OperandSize::Word)
}

#[test]
fn ror_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 207, 33], OperandSize::Dword)
}

#[test]
fn ror_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(EDI, 954745756, Some(OperandSize::Dword), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 143, 156, 67, 232, 56, 14], OperandSize::Dword)
}

#[test]
fn ror_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 201, 96], OperandSize::Qword)
}

#[test]
fn ror_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1269097825, Some(OperandSize::Dword), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 12, 69, 97, 229, 164, 75, 69], OperandSize::Qword)
}

#[test]
fn ror_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 206, 3], OperandSize::Qword)
}

#[test]
fn ror_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 12, 155, 47], OperandSize::Qword)
}

#[test]
fn ror_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 202], OperandSize::Word)
}

#[test]
fn ror_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BX, 65, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 79, 65], OperandSize::Word)
}

#[test]
fn ror_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 202], OperandSize::Dword)
}

#[test]
fn ror_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(EBX, 573814115, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 139, 99, 181, 51, 34], OperandSize::Dword)
}

#[test]
fn ror_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 203], OperandSize::Qword)
}

#[test]
fn ror_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 15], OperandSize::Qword)
}

#[test]
fn ror_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 202], OperandSize::Qword)
}

#[test]
fn ror_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RDI, 1197348969, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 143, 105, 24, 94, 71], OperandSize::Qword)
}

#[test]
fn ror_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 205], OperandSize::Word)
}

#[test]
fn ror_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 8], OperandSize::Word)
}

#[test]
fn ror_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 202], OperandSize::Dword)
}

#[test]
fn ror_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 12, 139], OperandSize::Dword)
}

#[test]
fn ror_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 207], OperandSize::Qword)
}

#[test]
fn ror_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1722729175, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 12, 181, 215, 194, 174, 102], OperandSize::Qword)
}

#[test]
fn ror_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 204], OperandSize::Word)
}

#[test]
fn ror_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(SI, 27888, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 140, 240, 108], OperandSize::Word)
}

#[test]
fn ror_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 203], OperandSize::Dword)
}

#[test]
fn ror_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 12, 121], OperandSize::Dword)
}

#[test]
fn ror_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 206], OperandSize::Qword)
}

#[test]
fn ror_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 8], OperandSize::Qword)
}

#[test]
fn ror_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 207], OperandSize::Qword)
}

#[test]
fn ror_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 12, 144], OperandSize::Qword)
}

#[test]
fn ror_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Word)
}

#[test]
fn ror_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 254, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 138, 254, 0], OperandSize::Word)
}

#[test]
fn ror_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Dword)
}

#[test]
fn ror_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(ESI, 1307979601, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 142, 81, 47, 246, 77], OperandSize::Dword)
}

#[test]
fn ror_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 202], OperandSize::Qword)
}

#[test]
fn ror_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1628434800, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 140, 176, 112, 241, 15, 97], OperandSize::Qword)
}

#[test]
fn ror_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Qword)
}

#[test]
fn ror_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 14], OperandSize::Qword)
}

#[test]
fn ror_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 207], OperandSize::Word)
}

#[test]
fn ror_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 22958, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 138, 174, 89], OperandSize::Word)
}

#[test]
fn ror_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 202], OperandSize::Dword)
}

#[test]
fn ror_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1447922292, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 140, 153, 116, 138, 77, 86], OperandSize::Dword)
}

#[test]
fn ror_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 206], OperandSize::Qword)
}

#[test]
fn ror_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1788527523, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 12, 181, 163, 195, 154, 106], OperandSize::Qword)
}

#[test]
fn ror_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 206], OperandSize::Word)
}

#[test]
fn ror_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 11], OperandSize::Word)
}

#[test]
fn ror_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 204], OperandSize::Dword)
}

#[test]
fn ror_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 196275971, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 12, 125, 3, 239, 178, 11], OperandSize::Dword)
}

#[test]
fn ror_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 205], OperandSize::Qword)
}

#[test]
fn ror_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 12, 215], OperandSize::Qword)
}

#[test]
fn ror_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RSP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 204], OperandSize::Qword)
}

#[test]
fn ror_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 963458143, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 12, 213, 95, 52, 109, 57], OperandSize::Qword)
}

