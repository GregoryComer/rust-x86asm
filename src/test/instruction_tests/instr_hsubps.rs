use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 210], OperandSize::Dword)
}

#[test]
fn hsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1488252253, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 172, 200, 93, 237, 180, 88], OperandSize::Dword)
}

#[test]
fn hsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 237], OperandSize::Qword)
}

#[test]
fn hsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1232416952, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 4, 245, 184, 48, 117, 73], OperandSize::Qword)
}

