use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM4)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 228, 99], OperandSize::Dword)
}

#[test]
fn psraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM4)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 228, 38], OperandSize::Qword)
}

#[test]
fn psraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 227, 40], OperandSize::Dword)
}

#[test]
fn psraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 229, 37], OperandSize::Qword)
}

#[test]
fn psraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 214], OperandSize::Dword)
}

#[test]
fn psraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(ECX, 2079856724, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 153, 84, 24, 248, 123], OperandSize::Dword)
}

#[test]
fn psraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 206], OperandSize::Qword)
}

#[test]
fn psraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 655903294, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 36, 77, 62, 74, 24, 39], OperandSize::Qword)
}

#[test]
fn psraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 196], OperandSize::Dword)
}

#[test]
fn psraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 60, 128], OperandSize::Dword)
}

#[test]
fn psraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 198], OperandSize::Qword)
}

#[test]
fn psraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1198956357, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 164, 88, 69, 159, 118, 71], OperandSize::Qword)
}

