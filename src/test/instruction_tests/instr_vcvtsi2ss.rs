use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 42, 193], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1161398301, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 42, 180, 154, 29, 136, 57, 69], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 42, 211], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDX, 1838303121, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 42, 170, 145, 71, 146, 109], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 194, 42, 223], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 194, 42, 54], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 88, 42, 194], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 42, 40], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 22, 16, 42, 229], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 14, 8, 42, 47], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 206, 88, 42, 252], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 142, 0, 42, 6], OperandSize::Qword)
}

