use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 231, 53], OperandSize::Dword)
}

#[test]
fn psraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM5)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 229, 116], OperandSize::Qword)
}

#[test]
fn psraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 225, 49], OperandSize::Dword)
}

#[test]
fn psraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 226, 84], OperandSize::Qword)
}

#[test]
fn psraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 248], OperandSize::Dword)
}

#[test]
fn psraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EAX, 1961982293, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 152, 85, 121, 241, 116], OperandSize::Dword)
}

#[test]
fn psraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 225], OperandSize::Qword)
}

#[test]
fn psraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 496366090, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 172, 138, 10, 242, 149, 29], OperandSize::Qword)
}

#[test]
fn psraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 209], OperandSize::Dword)
}

#[test]
fn psraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 1495480240, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 131, 176, 55, 35, 89], OperandSize::Dword)
}

#[test]
fn psraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 205], OperandSize::Qword)
}

#[test]
fn psraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 18742516, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 187, 244, 252, 29, 1], OperandSize::Qword)
}

