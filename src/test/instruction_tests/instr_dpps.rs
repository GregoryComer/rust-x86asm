use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 252, 121], OperandSize::Dword)
}

#[test]
fn dpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 349772803, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 44, 157, 3, 28, 217, 20, 105], OperandSize::Dword)
}

#[test]
fn dpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 218, 15], OperandSize::Qword)
}

#[test]
fn dpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 675867358, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 172, 254, 222, 234, 72, 40, 42], OperandSize::Qword)
}

