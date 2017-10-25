use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 110, 24, 123, 255], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 102, 8, 123, 28, 91], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 22, 80, 123, 231], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 94, 8, 123, 43], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 182, 24, 123, 242], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 230, 0, 123, 39], OperandSize::Qword)
}

