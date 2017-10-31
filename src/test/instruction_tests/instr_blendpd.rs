use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 219, 2], OperandSize::Dword)
}

#[test]
fn blendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1375625917, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 140, 251, 189, 98, 254, 81, 101], OperandSize::Dword)
}

#[test]
fn blendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 247, 8], OperandSize::Qword)
}

#[test]
fn blendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 536446275, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 180, 192, 67, 133, 249, 31, 94], OperandSize::Qword)
}

