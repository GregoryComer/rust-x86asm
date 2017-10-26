use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 247, 75], OperandSize::Dword)
}

#[test]
fn cmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1394072644, Some(OperandSize::Dword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 12, 157, 68, 220, 23, 83, 46], OperandSize::Dword)
}

#[test]
fn cmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 248, 23], OperandSize::Qword)
}

#[test]
fn cmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 691682652, Some(OperandSize::Dword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 44, 205, 92, 61, 58, 41, 125], OperandSize::Qword)
}

