use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 208, 61], OperandSize::Dword)
}

#[test]
fn dpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 44, 200, 27], OperandSize::Dword)
}

#[test]
fn dpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 219, 65], OperandSize::Qword)
}

#[test]
fn dpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 919728534, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 164, 127, 150, 241, 209, 54, 124], OperandSize::Qword)
}

