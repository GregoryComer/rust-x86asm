use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 223, 59], OperandSize::Dword)
}

#[test]
fn dpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 30, 50], OperandSize::Dword)
}

#[test]
fn dpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 224, 117], OperandSize::Qword)
}

#[test]
fn dpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1924927589, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 140, 67, 101, 16, 188, 114, 96], OperandSize::Qword)
}

