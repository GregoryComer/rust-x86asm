use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 253, 125], OperandSize::Dword)
}

#[test]
fn dppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 200589099, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 12, 149, 43, 191, 244, 11, 69], OperandSize::Dword)
}

#[test]
fn dppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 211, 124], OperandSize::Qword)
}

#[test]
fn dppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 395559918, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 132, 112, 238, 195, 147, 23, 54], OperandSize::Qword)
}

