use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 123, 217], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 156229616, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 103, 8, 123, 140, 199, 240, 223, 79, 9], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 95, 8, 123, 231], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 889476030, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 39, 0, 123, 164, 138, 190, 83, 4, 53], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 143, 16, 123, 241], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 239, 0, 123, 2], OperandSize::Qword)
}

