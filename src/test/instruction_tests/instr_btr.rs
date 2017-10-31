use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 229], OperandSize::Word)
}

#[test]
fn btr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 53], OperandSize::Word)
}

#[test]
fn btr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 231], OperandSize::Dword)
}

#[test]
fn btr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 26], OperandSize::Dword)
}

#[test]
fn btr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 246], OperandSize::Qword)
}

#[test]
fn btr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RAX, 1676746610, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 160, 114, 31, 241, 99], OperandSize::Qword)
}

#[test]
fn btr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 211], OperandSize::Word)
}

#[test]
fn btr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(DI, 19894, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 173, 182, 77], OperandSize::Word)
}

#[test]
fn btr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 234], OperandSize::Dword)
}

#[test]
fn btr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(EAX, 1255200036, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 144, 36, 213, 208, 74], OperandSize::Dword)
}

#[test]
fn btr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 238], OperandSize::Qword)
}

#[test]
fn btr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RSI, 1258630061, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 142, 173, 43, 5, 75], OperandSize::Qword)
}

#[test]
fn btr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 239], OperandSize::Qword)
}

#[test]
fn btr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 52, 154], OperandSize::Qword)
}

#[test]
fn btr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DI)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 247, 83], OperandSize::Word)
}

#[test]
fn btr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(SI, 139, Some(OperandSize::Word), None)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 180, 139, 0, 8], OperandSize::Word)
}

#[test]
fn btr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(SP)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 244, 118], OperandSize::Dword)
}

#[test]
fn btr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1364550642, Some(OperandSize::Word), None)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 52, 189, 242, 99, 85, 81, 42], OperandSize::Dword)
}

#[test]
fn btr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(SI)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 246, 23], OperandSize::Qword)
}

#[test]
fn btr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RBX, 1393457780, Some(OperandSize::Word), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 179, 116, 122, 14, 83, 110], OperandSize::Qword)
}

#[test]
fn btr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 247, 46], OperandSize::Word)
}

#[test]
fn btr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 51, 89], OperandSize::Word)
}

#[test]
fn btr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 242, 123], OperandSize::Dword)
}

#[test]
fn btr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 94, 32], OperandSize::Dword)
}

#[test]
fn btr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 246, 57], OperandSize::Qword)
}

#[test]
fn btr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 255, 69], OperandSize::Qword)
}

#[test]
fn btr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RCX)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 241, 42], OperandSize::Qword)
}

#[test]
fn btr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RAX, 1095929989, Some(OperandSize::Qword), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 176, 133, 144, 82, 65, 20], OperandSize::Qword)
}

