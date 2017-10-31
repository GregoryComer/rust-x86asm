use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM1)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 209, 85], OperandSize::Dword)
}

#[test]
fn psrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM4)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 212, 65], OperandSize::Qword)
}

#[test]
fn psrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 214, 112], OperandSize::Dword)
}

#[test]
fn psrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 211, 16], OperandSize::Qword)
}

#[test]
fn psrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 196], OperandSize::Dword)
}

#[test]
fn psrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 2057058785, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 140, 250, 225, 57, 156, 122], OperandSize::Dword)
}

#[test]
fn psrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 201], OperandSize::Qword)
}

#[test]
fn psrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 343818178, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 209, 36, 133, 194, 63, 126, 20], OperandSize::Qword)
}

#[test]
fn psrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 227], OperandSize::Dword)
}

#[test]
fn psrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1948438139, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 140, 240, 123, 206, 34, 116], OperandSize::Dword)
}

#[test]
fn psrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 192], OperandSize::Qword)
}

#[test]
fn psrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1655039380, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 209, 132, 139, 148, 229, 165, 98], OperandSize::Qword)
}

