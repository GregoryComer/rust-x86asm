use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 199, 68], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 2, 4], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 195, 106], OperandSize::Qword)
}

#[test]
fn vaeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 704376700, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 156, 246, 124, 239, 251, 41, 124], OperandSize::Qword)
}

