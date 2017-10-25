use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 192, 110], OperandSize::Dword)
}

#[test]
fn blendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 28, 251, 17], OperandSize::Dword)
}

#[test]
fn blendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 193, 32], OperandSize::Qword)
}

#[test]
fn blendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 452501486, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 158, 238, 159, 248, 26, 117], OperandSize::Qword)
}

